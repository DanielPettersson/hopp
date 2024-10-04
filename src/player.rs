use std::cmp::Ordering;
use crate::drag::Drag;
use crate::{GameState, Height, MaterialHandles, MeshHandles};
use avian2d::prelude::{
    AngularDamping, Collider, ColliderMassProperties, DistanceJoint, ExternalAngularImpulse,
    ExternalForce, ExternalImpulse, Friction, Joint, LinearDamping, Restitution, RigidBody,
};
use bevy::app::{App, Plugin, Update};
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, in_state, Bundle, ColorMaterial, Commands, Component, Entity, EventReader, EventWriter, FixedUpdate, Handle, IntoSystemConfigs, OnEnter, Query, Res, ResMut, Resource, Transform, Vec3, With, Without};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::score::Score;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct DragIndicator;

#[derive(Resource)]
struct MaxDrag(f32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MaxDrag(120.0))
            .add_systems(OnEnter(GameState::InGame), create_player)
            .add_systems(
                Update,
                (jump, drag_indicator).run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                increase_height.run_if(in_state(GameState::InGame)),
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
        pos: Vec2,
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
            friction: Friction::new(0.4),
            restitution: Restitution::new(0.5),
            material_mesh: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_xyz(pos.x, pos.y, -1.).with_scale(Vec3::splat(size)),
                ..default()
            },
            player: Player,
        }
    }
}

fn create_player(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    material_handles: Res<MaterialHandles>,
) {
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
            let x = c as f32 * (size + gap) + size / 2.;
            let y = r as f32 * (size + gap) + size / 2. - num_rows as f32 * (size + gap);
            row.push(
                commands
                    .spawn(PlayerBundle::new(
                        mesh_handles.rectangle_2.clone(),
                        material_handles.yellow.clone(),
                        Vec2::new(x, y),
                        size,
                    ))
                    .id(),
            );
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

fn jump(
    mut query_jumper: Query<
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
) {
    for drag in mouse_drag_event.read() {
        if drag.done {
            for (mut impulse, mut angular_impulse, _, mass_props) in query_jumper.iter_mut() {
                let drag = (drag.end - drag.start).clamp_length_max(max_drag.0);
                let impulse_vec = Vec2 {
                    x: drag.x.signum() * drag.x.abs().sqrt() * mass_props.mass.0 * -30.,
                    y: drag.y.signum() * drag.y.abs().sqrt() * mass_props.mass.0 * -60.,
                };
                impulse.set_impulse(impulse_vec);

                angular_impulse.set_impulse(drag.x * 40.);
            }
        } else {
            for (_, _, mut force, mass_props) in query_jumper.iter_mut() {
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

fn increase_height(mut height: ResMut<Height>, mut score: ResMut<Score>, query_player: Query<&Transform, With<Player>>) {
    let max_y = query_player.iter().map(|t| t.translation.y).max_by(|y1, y2| y1.partial_cmp(y2).unwrap_or(Ordering::Equal)).unwrap_or(0.);
    
    if max_y > height.0 {
        height.0 = max_y;
    }

    if max_y as u32 > score.0 {
        score.0 = max_y as u32;        
    }
}
