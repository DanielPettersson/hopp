mod drag;
mod player;
mod camera;

use crate::camera::CameraPlugin;
use crate::drag::DragPlugin;
use crate::player::PlayerPlugin;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Resource)]
struct MaterialHandles {
    red: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct MeshHandles {
    rectangle: Mesh2dHandle,
}

#[derive(Component)]
struct MovementState {
    position: Vec2,
    old_position: Vec2,
    velocity: Vec2,
}

impl MovementState {
    fn new(position: Vec2, velocity: Vec2) -> Self {
        Self {
            position,
            old_position: position,
            velocity,
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(100.0),
            DragPlugin,
            PlayerPlugin,
            CameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_movement)
        .insert_resource(SubstepCount(6))
        .insert_resource(Gravity(Vec2::NEG_Y * 981.0))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let red = materials.add(Color::srgb(1., 0., 0.));
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
    commands.spawn(create_rectangle(
        rectangle.clone(),
        green.clone(),
        100.,
        20.,
        Vec3::new(-90., -50., 0.),
    ));
    commands.spawn(create_rectangle(
        rectangle.clone(),
        green.clone(),
        100.,
        20.,
        Vec3::new(90., -90., 0.),
    ));

    
}



fn update_movement(
    fixed_time: Res<Time<Fixed>>,
    mut movement_query: Query<(&mut Transform, &MovementState)>,
) {
    for (mut transform, state) in movement_query.iter_mut() {
        transform.translation = state
            .old_position
            .lerp(state.position, fixed_time.overstep_fraction())
            .extend(transform.translation.z);
    }
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
