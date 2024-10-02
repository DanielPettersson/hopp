use crate::drag::Drag;
use avian2d::prelude::{
    AngularDamping, Collider, ColliderMassProperties, DistanceJoint, ExternalAngularImpulse,
    ExternalImpulse, Friction, Joint, LinearDamping, Restitution, RigidBody,
};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Bundle, ColorMaterial, Commands, Component, Entity, EventReader, Handle, Mesh, Query, Rectangle, Res, ResMut, Resource, Transform, Vec3, With, Without};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::{MaterialHandles, MeshHandles};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct DragIndicator;

#[derive(Resource)]
struct MaxDrag(f32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, (jump, drag_indicator));
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    rigid_body: RigidBody,
    collider: Collider,
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
        mesh_handle: Handle<Mesh>,
        material_handle: Handle<ColorMaterial>,
        pos: Vec2,
        size: f32,
    ) -> Self {
        PlayerBundle {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(1., 1.),
            external_impulse: ExternalImpulse::new(Vec2::ZERO).with_persistence(false),
            external_angular_impulse: ExternalAngularImpulse::new(0.).with_persistence(false),
            linear_damping: LinearDamping(0.1),
            angular_damping: AngularDamping(0.1),
            friction: Friction::new(0.4),
            restitution: Restitution::new(0.5),
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle::from(mesh_handle),
                material: material_handle,
                transform: Transform::from_xyz(pos.x, pos.y, -1.).with_scale(Vec3::splat(size)),
                ..default()
            },
            player: Player,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let yellow = materials.add(Color::srgb(1., 1., 0.));

    let num_rows = 9;
    let num_cols = num_rows;
    let size = 2.;
    let gap: f32 = size / 2.;
    let d_gap = (size * size + gap * gap).sqrt();
    let compliance = 0.00015 / size;
    let rectangle = meshes.add(Rectangle::new(2., 2.));

    let mut rows: Vec<Vec<Entity>> = Vec::with_capacity(num_rows);
    for r in 0..num_rows {
        let mut row = Vec::with_capacity(num_cols);
        for c in 0..num_cols {
            let x = c as f32 * (size + gap) + size / 2.;
            let y = r as f32 * (size + gap) + size / 2.;
            row.push(
                commands
                    .spawn(PlayerBundle::new(
                        rectangle.clone(),
                        yellow.clone(),
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
    
    commands.insert_resource(MaxDrag(100.0));
}

fn jump(
    mut query_jumper: Query<(&mut ExternalImpulse, &mut ExternalAngularImpulse, &ColliderMassProperties), With<Player>>,
    mut mouse_drag_event: EventReader<Drag>,
    max_drag: Res<MaxDrag>,
) {
    for drag in mouse_drag_event.read() {
        if drag.done {
            for (mut impulse, mut angular_impulse, mass_props) in query_jumper.iter_mut() {
                let drag = (drag.end - drag.start).clamp_length_max(max_drag.0);
                let impulse_vec = Vec2 {
                    x: drag.x.signum() * drag.x.abs().sqrt() * mass_props.mass.0 * -30.,
                    y: drag.y.signum() * drag.y.abs().sqrt() * mass_props.mass.0 * -60.,
                };
                impulse.set_impulse(impulse_vec);
                
                angular_impulse.set_impulse(drag.x * 40.);
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
        let drag_length = drag_vec.length().max(10.);
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
