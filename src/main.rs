use avian2d::parry::na::ComplexField;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(100.0),
            // PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, jump)
        .add_systems(FixedUpdate, mouse_drag)
        .add_systems(Update, drag_indicator)
        .insert_resource(Gravity(Vec2::NEG_Y * 981.0))
        .add_event::<MouseDrag>()
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
    mut query_jumper: Query<(&mut ExternalImpulse, &ColliderMassProperties)>,
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
                impulse.apply_impulse(impulse_vec);
            }
        }
    }
}

fn drag_indicator(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_drag_event: EventReader<MouseDrag>,
    mut indicator_entity: Local<Option<Entity>>,
) {
    for drag in mouse_drag_event.read() {
        if drag.done {
            if let Some(entity) = *indicator_entity {
                commands.entity(entity).despawn();
                *indicator_entity = None;
            }
        } else if indicator_entity.is_none() {
            *indicator_entity = Some(commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle::new(10.))),
                    material: materials.add(Color::srgb(1., 0., 0.)),
                    transform: Transform::from_xyz(drag.start.x, drag.start.y, 1.),
                    ..default()
                },
            )).id());
        }
    }
}

fn mouse_drag(
    mouse_button: Res<ButtonInput<MouseButton>>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mut event_writer: EventWriter<MouseDrag>,
    mut drag_start: Local<Vec2>,
) {
    if mouse_button.pressed(MouseButton::Left) || mouse_button.just_released(MouseButton::Left) {
        let (camera, camera_transform) = query_camera.single();

        let cursor_pos = query_window.single().cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate());

        if mouse_button.just_pressed(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                *drag_start = pos;
                event_writer.send(MouseDrag {
                    start: pos,
                    end: pos,
                    done: false,
                });
            }
        } else if mouse_button.pressed(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                event_writer.send(MouseDrag {
                    start: *drag_start,
                    end: pos,
                    done: false,
                });
            }
        } else if mouse_button.just_released(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                event_writer.send(MouseDrag {
                    start: *drag_start,
                    end: pos,
                    done: true
                });
            }
        }
    }
}

#[derive(Event, Default, Debug)]
struct MouseDrag {
    start: Vec2,
    end: Vec2,
    done: bool
}
