mod mouse_drag;

use crate::mouse_drag::{MouseDrag, MouseDragPlugin};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(100.0),
            MouseDragPlugin,
            //PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, jump)
        .add_systems(Update, drag_indicator)
        .insert_resource(SubstepCount(15))
        .insert_resource(Gravity(Vec2::NEG_Y * 981.0))
        .insert_gizmo_config(
            PhysicsGizmos {
                axis_lengths: None,
                ..default()
            },
            GizmoConfig::default(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let red = materials.add(Color::srgb(1., 0., 0.));
    let yellow = materials.add(Color::srgb(1., 1., 0.));
    let green = materials.add(Color::srgb(0., 1., 0.));
    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(1., 1.)));

    commands.insert_resource(MaterialHandles { red: red.clone() });

    commands.insert_resource(MeshHandles {
        rectangle: rectangle.clone(),
    });

    commands.spawn(create_rectangle(
        rectangle.clone(),
        green.clone(),
        379.,
        20.,
        Vec3::new(0., -200., 0.),
    ));

    commands.spawn(create_rectangle(
        rectangle.clone(),
        green.clone(),
        379.,
        20.,
        Vec3::new(0., 200., 0.),
    ));

    commands.spawn(create_rectangle(
        rectangle.clone(),
        green.clone(),
        20.,
        379.,
        Vec3::new(-200., 0., 0.),
    ));

    commands.spawn(create_rectangle(
        rectangle.clone(),
        green.clone(),
        20.,
        379.,
        Vec3::new(200., 0., 0.),
    ));
    commands.spawn(create_rectangle(
        rectangle.clone(),
        green.clone(),
        100.,
        20.,
        Vec3::new(-100., 70., 0.),
    ));
    commands.spawn(create_rectangle(
        rectangle.clone(),
        green.clone(),
        100.,
        20.,
        Vec3::new(100., 00., 0.),
    ));

    let num_rows = 10;
    let num_cols = 10;
    let size = 1.5;
    let gap: f32 = size / 2.;
    let d_gap = (size * size + gap * gap).sqrt();
    let compliance = 0.0002 / size;
    let blob_r = Mesh2dHandle(meshes.add(Rectangle::new(2., 2.)));

    let mut rows: Vec<Vec<Entity>> = Vec::with_capacity(num_rows);
    for r in 0..num_rows {
        let mut row = Vec::with_capacity(num_cols);
        for c in 0..num_cols {
            let x = c as f32 * (size + gap) + size / 2.;
            let y = r as f32 * (size + gap) + size / 2.;
            row.push(
                commands
                    .spawn((
                        RigidBody::Dynamic,
                        Collider::rectangle(1., 1.),
                        ExternalAngularImpulse::new(0.).with_persistence(false),
                        ExternalImpulse::new(Vec2::ZERO).with_persistence(false),
                        LinearDamping(0.1),
                        AngularDamping(0.1),
                        Friction::new(0.4),
                        Restitution::new(0.5),
                        MaterialMesh2dBundle {
                            mesh: blob_r.clone(),
                            material: yellow.clone(),
                            transform: Transform::from_xyz(x, y, -1.).with_scale(Vec3::splat(size)),
                            ..default()
                        },
                        Jumper,
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
                        .with_rest_length(gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, row[c + 1])
                        .with_local_anchor_1(Vec2::new(size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, size * 0.5))
                        .with_compliance(compliance)
                        .with_rest_length(gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, row[c + 1])
                        .with_local_anchor_1(Vec2::new(size * 0.5, -size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, size * 0.5))
                        .with_compliance(compliance)
                        .with_rest_length(d_gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, row[c + 1])
                        .with_local_anchor_1(Vec2::new(size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_rest_length(d_gap),
                );
            }
            if r < num_rows - 1 {
                commands.spawn(
                    DistanceJoint::new(*square, rows[r + 1][c])
                        .with_local_anchor_1(Vec2::new(-size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_rest_length(gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, rows[r + 1][c])
                        .with_local_anchor_1(Vec2::new(size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_rest_length(gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, rows[r + 1][c])
                        .with_local_anchor_1(Vec2::new(-size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_rest_length(d_gap),
                );
                commands.spawn(
                    DistanceJoint::new(*square, rows[r + 1][c])
                        .with_local_anchor_1(Vec2::new(size * 0.5, size * 0.5))
                        .with_local_anchor_2(Vec2::new(-size * 0.5, -size * 0.5))
                        .with_compliance(compliance)
                        .with_rest_length(d_gap),
                );
            }
        }
    }

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 400.0,
        min_height: 400.0,
    };
    commands.spawn(camera);
}

fn create_rectangle(
    mesh_handle: Mesh2dHandle,
    material_handle: Handle<ColorMaterial>,
    width: f32,
    height: f32,
    translation: Vec3,
) -> impl Bundle {
    (
        RigidBody::Static,
        Collider::rectangle(1., 1.),
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: material_handle,
            transform: Transform::from_translation(translation)
                .with_scale(Vec3::new(width, height, 1.)),
            ..default()
        },
    )
}

fn jump(
    mut query_jumper: Query<(&mut ExternalImpulse, &ColliderMassProperties), With<Jumper>>,
    mut mouse_drag_event: EventReader<MouseDrag>,
) {
    for drag in mouse_drag_event.read() {
        if drag.done {
            for (mut impulse, mass_props) in query_jumper.iter_mut() {
                let drag = drag.end - drag.start;
                let impulse_vec = Vec2 {
                    x: drag.x.signum() * drag.x.abs().sqrt() * mass_props.mass.0 * -30.,
                    y: drag.y.signum() * drag.y.abs().sqrt() * mass_props.mass.0 * -60.,
                };
                impulse.set_impulse(impulse_vec);
            }
        }
    }
}

fn drag_indicator(
    mut commands: Commands,
    mut mouse_drag_event: EventReader<MouseDrag>,
    mut query_drag_indicator: Query<(&mut Transform, Entity), With<DragIndicator>>,
    query_jumper: Query<&Transform, (With<Jumper>, Without<DragIndicator>)>,
    material_handles: Res<MaterialHandles>,
    mesh_handles: Res<MeshHandles>,
) {
    let mut drag_done = false;
    for drag in mouse_drag_event.read() {
        drag_done = drag.done || drag_done;

        for jumper in query_jumper.iter() {
            let drag_length = (drag.end - drag.start).length().max(10.);
            let drag_mid = (drag.end - drag.start) / 2.;
            let drag_indicator_transform =
                Transform::from_translation(jumper.translation - drag_mid.extend(1.))
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
    }
    if drag_done {
        for (_, entity) in query_drag_indicator.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Resource)]
struct MaterialHandles {
    red: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct MeshHandles {
    rectangle: Mesh2dHandle,
}

#[derive(Component)]
struct DragIndicator;

#[derive(Component)]
struct Jumper;
