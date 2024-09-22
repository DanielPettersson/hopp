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
    create_rectangle(
        &mut commands,
        &mut meshes,
        &mut materials,
        400.,
        20.,
        Vec3::new(0., -200., 0.),
    );

    create_rectangle(
        &mut commands,
        &mut meshes,
        &mut materials,
        400.,
        20.,
        Vec3::new(0., 200., 0.),
    );

    create_rectangle(
        &mut commands,
        &mut meshes,
        &mut materials,
        20.,
        400.,
        Vec3::new(-200., 0., 0.),
    );

    create_rectangle(
        &mut commands,
        &mut meshes,
        &mut materials,
        20.,
        400.,
        Vec3::new(200., 0., 0.),
    );

    create_rectangle(
        &mut commands,
        &mut meshes,
        &mut materials,
        100.,
        20.,
        Vec3::new(-100., 70., 0.),
    );

    create_rectangle(
        &mut commands,
        &mut meshes,
        &mut materials,
        100.,
        20.,
        Vec3::new(100., 00., 0.),
    );

    commands.spawn((
        RigidBody::Dynamic,
        Collider::capsule(5., 10.),
        ExternalAngularImpulse::new(0.).with_persistence(false),
        ExternalImpulse::new(Vec2::ZERO).with_persistence(false),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(5., 10.))),
            material: materials.add(Color::srgb(1., 1., 0.)),
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
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    width: f32,
    height: f32,
    translation: Vec3,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(width, height),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(width, height))),
            material: materials.add(Color::srgb(0., 1., 0.)),
            transform: Transform::from_translation(translation),
            ..default()
        },
    ));
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_drag_event: EventReader<MouseDrag>,
    mut query_drag_indicator: Query<(&mut Transform, Entity), With<DragIndicator>>,
    query_jumper: Query<&Transform, (With<Jumper>, Without<DragIndicator>)>,
) {
    for (_, entity) in query_drag_indicator.iter_mut() {
        commands.entity(entity).despawn();
    }
    for drag in mouse_drag_event.read() {
        if !drag.done {
            for jumper in query_jumper.iter() {
                let drag_length = (drag.end - drag.start).length().max(10.);
                let drag_mid = (drag.end - drag.start) / 2.;
                let drag_indicator_transform =
                    Transform::from_translation(jumper.translation - drag_mid.extend(1.))
                        .with_rotation(Quat::from_rotation_z(if drag_length > 0. {
                            drag_mid.to_angle()
                        } else {
                            0.
                        }));
                commands.spawn((
                    DragIndicator,
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(drag_length, 2.))),
                        material: materials.add(Color::srgb(1., 0., 0.)),
                        transform: drag_indicator_transform,
                        ..default()
                    },
                ));
            }
        }
    }
}

#[derive(Component)]
struct DragIndicator;

#[derive(Component)]
struct Jumper;
