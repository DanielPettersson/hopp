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
        .add_systems(Update, jump)
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
        MouseDrag::default(),
    ));

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin { min_width: 400.0, min_height: 400.0 };
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
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut query_draggable: Query<(&mut ExternalImpulse, &mut MouseDrag, &ColliderMassProperties)>,
    query_window: Query<&Window, With<PrimaryWindow>>,
) {
    let cursor_pos = query_window.single().cursor_position();

    for (mut impulse, mut mouse_drag, mass_props) in query_draggable.iter_mut() {
        if mouse_button.just_pressed(MouseButton::Left) {
            mouse_drag.start = cursor_pos;
        }
        if mouse_button.just_released(MouseButton::Left) {
            if let Some(drag_start) = mouse_drag.start {
                if let Some(drag_end) = cursor_pos {
                    let drag = drag_end - drag_start;
                    let impulse_vec = Vec2 {
                        x: drag.x.signum() * drag.x.abs().sqrt() * mass_props.mass.0 * -28.,
                        y: drag.y.signum() * drag.y.abs().sqrt() * mass_props.mass.0 * 56.,
                    };
                    println!("{:?}", mass_props.mass);
                    impulse.apply_impulse(impulse_vec);
                }
            }
            mouse_drag.start = None;
        }
    }
}

#[derive(Component, Default)]
struct MouseDrag {
    start: Option<Vec2>,
}
