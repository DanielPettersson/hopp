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
            // PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, jump)
        .add_systems(Update, drag_indicator)
        .insert_resource(Gravity(Vec2::NEG_Y * 981.0))
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

    commands.spawn((
        RigidBody::Dynamic,
        Collider::capsule(5., 10.),
        ExternalAngularImpulse::new(0.).with_persistence(false),
        ExternalImpulse::new(Vec2::ZERO).with_persistence(false),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(5., 10.))),
            material: yellow.clone(),
            transform: Transform::from_xyz(0., 50., 0.),
            ..default()
        },
        Jumper,
    ));

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
