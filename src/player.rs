use crate::drag::Drag;
use crate::score::Score;
use crate::{GameState, Height, MaterialHandles, MeshHandles, HALF_WORLD_SIZE};
use avian2d::prelude::{
    AngularDamping, Collider, ColliderMassProperties, DistanceJoint, ExternalAngularImpulse,
    ExternalForce, ExternalImpulse, Friction, Joint, LinearDamping, Restitution, RigidBody,
};
use bevy::app::{App, Plugin, Update};
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, in_state, Bundle, ColorMaterial, Commands, Component, Entity, EventReader, FixedUpdate, Handle, IntoSystemConfigs, NextState, OnEnter, OnExit, Or, Query, Res, ResMut, Resource, Time, Timer, Transform, Vec3, With, Without};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::time::TimerMode;
use std::cmp::Ordering;
use std::time::Duration;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct InnerPlayer;

#[derive(Component)]
struct DragIndicator;

#[derive(Resource)]
struct MaxDrag(f32);
#[derive(Resource)]
struct JumpTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(JumpTimer(Timer::new(
            Duration::from_millis(700),
            TimerMode::Once,
        )))
        .insert_resource(MaxDrag(120.0))
        .add_systems(OnEnter(GameState::InGame), create_player)
        .add_systems(OnExit(GameState::InGame), remove_player)
        .add_systems(
            Update,
            (jump, drag_indicator).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (player_height, light_up_player).run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    rigid_body: RigidBody,
    collider: Collider,
    external_force: ExternalForce,
    external_impulse: ExternalImpulse,
    external_angular_impulse: ExternalAngularImpulse,
    linear_damping: LinearDamping,
    angular_damping: AngularDamping,
    friction: Friction,
    restitution: Restitution,
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
    player: Player,
}

impl PlayerBundle {
    pub fn new(
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<ColorMaterial>,
        pos: Vec3,
        size: f32,
    ) -> Self {
        PlayerBundle {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(1., 1.),
            external_force: ExternalForce::new(Vec2::ZERO).with_persistence(false),
            external_impulse: ExternalImpulse::new(Vec2::ZERO).with_persistence(false),
            external_angular_impulse: ExternalAngularImpulse::new(0.).with_persistence(false),
            linear_damping: LinearDamping(0.1),
            angular_damping: AngularDamping(0.1),
            friction: Friction::new(0.7),
            restitution: Restitution::new(0.5),
            material_mesh: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(size)),
                ..default()
            },
            player: Player,
        }
    }
}

type WithPlayerOrDragIndicator = Or<(With<Player>, With<DragIndicator>)>;

fn create_player(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    material_handles: Res<MaterialHandles>,
    mut jump_timer: ResMut<JumpTimer>,
) {
    jump_timer.0.reset();

    let num_rows = 9;
    let num_cols = num_rows;
    let size = 2.;
    let gap: f32 = size / 2.;
    let d_gap = (size * size + gap * gap).sqrt();
    let compliance = 0.00015 / size;

    let mut rows: Vec<Vec<Entity>> = Vec::with_capacity(num_rows);
    for r in 0..num_rows {
        let mut row = Vec::with_capacity(num_cols);
        for c in 0..num_cols {
            let edge = r == 0 || r == num_rows - 1 || c == 0 || c == num_cols - 1;
            let x = c as f32 * (size + gap) + size / 2.;
            let y = r as f32 * (size + gap) + size / 2. - 160.;

            let player_bundle = PlayerBundle::new(
                mesh_handles.rectangle_2.clone(),
                if edge {
                    material_handles.black.clone()
                } else {
                    material_handles.red.clone()
                },
                Vec3::new(x, y, if edge { 0. } else { -1. }),
                size,
            );

            if edge {
                row.push(commands.spawn(player_bundle).id());
            } else {
                row.push(commands.spawn((player_bundle, InnerPlayer)).id());
            }
        }
        rows.push(row);
    }

    for (r, row) in rows.iter().enumerate() {
        for (c, square) in row.iter().enumerate() {
            if c < num_cols - 1 {
                commands.spawn(
                    DistanceJoint::new(*square, row[c + 1])
                        .with_local_anchor_1(Vec2::new(size * 0.5, -size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_limits(0., gap)
                        .with_rest_length(gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, row[c + 1])
                        .with_local_anchor_1(Vec2::new(size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, size * 0.5))
                        .with_compliance(compliance)
                        .with_limits(0., gap)
                        .with_rest_length(gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, row[c + 1])
                        .with_local_anchor_1(Vec2::new(size * 0.5, -size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, size * 0.5))
                        .with_compliance(compliance)
                        .with_limits(0., d_gap)
                        .with_rest_length(d_gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, row[c + 1])
                        .with_local_anchor_1(Vec2::new(size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_limits(0., d_gap)
                        .with_rest_length(d_gap),
                );
            }
            if r < num_rows - 1 {
                commands.spawn(
                    DistanceJoint::new(*square, rows[r + 1][c])
                        .with_local_anchor_1(Vec2::new(-size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_limits(0., gap)
                        .with_rest_length(gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, rows[r + 1][c])
                        .with_local_anchor_1(Vec2::new(size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_limits(0., gap)
                        .with_rest_length(gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, rows[r + 1][c])
                        .with_local_anchor_1(Vec2::new(-size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_limits(0., d_gap)
                        .with_rest_length(d_gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, rows[r + 1][c])
                        .with_local_anchor_1(Vec2::new(size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_limits(0., d_gap)
                        .with_rest_length(d_gap),
                );
            }
        }
    }
}

fn remove_player(
    mut commands: Commands,
    query_player: Query<Entity, WithPlayerOrDragIndicator>,
) {
    for entity in query_player.iter() {
        commands.entity(entity).despawn();
    }
}

fn jump(
    mut query_player: Query<
        (
            &mut ExternalImpulse,
            &mut ExternalAngularImpulse,
            &mut ExternalForce,
            &ColliderMassProperties,
        ),
        With<Player>,
    >,
    mut mouse_drag_event: EventReader<Drag>,
    max_drag: Res<MaxDrag>,
    mut jump_timer: ResMut<JumpTimer>,
    time: Res<Time>,
) {
    jump_timer.0.tick(time.delta());
    for drag in mouse_drag_event.read() {
        if jump_timer.0.finished() {
            if drag.done {
                for (mut impulse, mut angular_impulse, _, mass_props) in query_player.iter_mut() {
                    let drag = (drag.end - drag.start).clamp_length_max(max_drag.0);
                    let impulse_vec = Vec2 {
                        x: drag.x.signum() * drag.x.abs().sqrt() * mass_props.mass.0 * -30.,
                        y: drag.y.signum() * drag.y.abs().sqrt() * mass_props.mass.0 * -60.,
                    };
                    impulse.set_impulse(impulse_vec);

                    angular_impulse.set_impulse(drag.x * 40.);
                }
                jump_timer.0.reset();
            } else {
                for (_, _, mut force, mass_props) in query_player.iter_mut() {
                    let drag = (drag.end - drag.start).clamp_length_max(max_drag.0);
                    let force_vec = Vec2 {
                        x: drag.x.signum() * drag.x.abs().sqrt() * mass_props.mass.0 * 30.,
                        y: drag.y.signum() * drag.y.abs().sqrt() * mass_props.mass.0 * 60.,
                    };
                    force.set_force(force_vec);
                }
            }
        }
    }
}

fn drag_indicator(
    mut commands: Commands,
    mut mouse_drag_event: EventReader<Drag>,
    mut query_drag_indicator: Query<(&mut Transform, Entity), With<DragIndicator>>,
    query_player: Query<&Transform, (With<Player>, Without<DragIndicator>)>,
    material_handles: Res<MaterialHandles>,
    mesh_handles: Res<MeshHandles>,
    max_drag: Res<MaxDrag>,
) {
    let mut drag_done = false;
    for drag in mouse_drag_event.read() {
        drag_done = drag.done || drag_done;

        let translation = query_player
            .iter()
            .map(|t| t.translation)
            .fold(Vec3::ZERO, |a, v| a + v)
            / query_player.iter().count() as f32;

        let drag_vec = (drag.end - drag.start).clamp_length_max(max_drag.0);
        let drag_length = drag_vec.length().max(1.);
        let drag_mid = drag_vec / 2.;
        let drag_indicator_transform =
            Transform::from_translation(translation - drag_mid.extend(-1.))
                .with_scale(Vec3::new(drag_length, 2., 1.))
                .with_rotation(Quat::from_rotation_z(if drag_length > 0. {
                    drag_mid.to_angle()
                } else {
                    0.
                }));

        if query_drag_indicator.is_empty() {
            commands.spawn((
                DragIndicator,
                MaterialMesh2dBundle {
                    mesh: mesh_handles.rectangle.clone(),
                    material: material_handles.red.clone(),
                    transform: drag_indicator_transform,
                    ..default()
                },
            ));
        }
        for (mut tranform, _) in query_drag_indicator.iter_mut() {
            *tranform = drag_indicator_transform;
        }
    }
    if drag_done {
        for (_, entity) in query_drag_indicator.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

fn player_height(
    mut next_state: ResMut<NextState<GameState>>,
    mut height: ResMut<Height>,
    mut score: ResMut<Score>,
    query_player: Query<&Transform, With<Player>>,
) {
    let player_pos = query_player
        .iter()
        .map(|t| t.translation)
        .max_by(|t1, t2| t1.y.partial_cmp(&t2.y).unwrap_or(Ordering::Equal))
        .unwrap_or(Vec3::ZERO);

    if player_pos.y > height.0 + 50. {
        height.0 = player_pos.y;
    }

    if player_pos.y as u32 > score.0 {
        score.0 = player_pos.y as u32;
    }

    if player_pos.y < height.0 - HALF_WORLD_SIZE
        || !((-HALF_WORLD_SIZE - 50.)..(HALF_WORLD_SIZE + 50.)).contains(&player_pos.x)
    {
        next_state.set(GameState::GameOver);
    }
}

fn light_up_player(
    jump_timer: Res<JumpTimer>,
    mut inner_player_query: Query<&mut Handle<ColorMaterial>, With<InnerPlayer>>,
    material_handles: Res<MaterialHandles>,
) {
    for mut material_handle in inner_player_query.iter_mut() {
        *material_handle = if jump_timer.0.finished() {
            material_handles.bright_red.clone()
        } else {
            material_handles.red.clone()
        };
    }
}
