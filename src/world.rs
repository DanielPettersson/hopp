use crate::{Height, MaterialHandles, MeshHandles, WORLD_SIZE};
use avian2d::collision::Collider;
use avian2d::prelude::RigidBody;
use bevy::app::App;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::prelude::{default, Bundle, ColorMaterial, Commands, Component, Entity, FixedUpdate, Local, Mesh, Plugin, Query, Rectangle, Res, ResMut, Startup, Transform, Vec2, With};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, add_platforms)
        .add_systems(FixedUpdate, remove_platforms);
    }
}

#[derive(Component)]
struct Platform;

#[derive(Bundle)]
struct PlatformBundle {
    rigid_body: RigidBody,
    collider: Collider,
    material_mesh2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
    platform: Platform,
}

impl PlatformBundle {
    fn new(
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<ColorMaterial>,
        width: f32,
        height: f32,
        translation: Vec2,
    ) -> Self {
        Self {
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(1., 1.),
            material_mesh2d_bundle: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_translation(translation.extend(0.0))
                    .with_scale(Vec3::new(width, height, 1.)),
                ..default()
            },
            platform: Platform,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let green = materials.add(Color::srgb(0., 1., 0.));
    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(1., 1.)));
    commands.spawn(PlatformBundle::new(
        rectangle.clone(),
        green.clone(),
        400.,
        20.,
        Vec2::new(0., -150.),
    ));

    commands.spawn(PlatformBundle::new(
        rectangle.clone(),
        green.clone(),
        100.,
        20.,
        Vec2::new(-100., -75.),
    ));

    commands.spawn(PlatformBundle::new(
        rectangle.clone(),
        green.clone(),
        100.,
        20.,
        Vec2::new(100., 0.),
    ));
}

fn add_platforms(
    mut commands: Commands,
    material_handles: Res<MaterialHandles>,
    mesh_handles: Res<MeshHandles>,
    height: Res<Height>,
    mut highest_platform: Local<Vec2>,
) {
    if height.0 > highest_platform.y - WORLD_SIZE / 2. {
        highest_platform.y += 100.;
        highest_platform.x = rand::random::<f32>() * 300. - 150.;

        commands.spawn(PlatformBundle::new(
            mesh_handles.rectangle.clone(),
            material_handles.green.clone(),
            100.,
            20.,
            *highest_platform,
        ));
    }
}

fn remove_platforms(
    mut commands: Commands,
    height: Res<Height>,
    platform_query: Query<(Entity, &Transform), With<Platform>>,
) {
    for (entity, transform) in platform_query.iter() {
        if transform.translation.y < height.0 - WORLD_SIZE / 2. - transform.scale.y {
            commands.entity(entity).despawn();
        }
    }
}
