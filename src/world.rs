use crate::{Height, MaterialHandles, WORLD_SIZE};
use avian2d::collision::Collider;
use avian2d::prelude::RigidBody;
use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{default, AssetServer, Bundle, Commands, Component, Entity, FixedUpdate, Image, ImageScaleMode, Plugin, Query, Rect, Res, ResMut, Resource, Sprite, SpriteBundle, Startup, Transform, Vec2, With};
use rand::Rng;
use std::ops::{Deref, DerefMut};

static PLATFORM_TEXTURE_SIZE: f32 = 46.;

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

#[derive(Resource)]
struct HighestPlatformPos(Vec2);

impl Deref for HighestPlatformPos {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HighestPlatformPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Bundle)]
struct PlatformBundle {
    rigid_body: RigidBody,
    collider: Collider,
    sprite: SpriteBundle,
    platform: Platform,
    image_scale_mode: ImageScaleMode,
}

impl PlatformBundle {
    fn new(texture: Handle<Image>, width: f32, height: f32, translation: Vec2) -> Self {
        Self {
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(width, height),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(width, height)),
                    rect: Some(Rect::new(0., 0., PLATFORM_TEXTURE_SIZE, height)),
                    ..default()
                },
                transform: Transform::from_translation(translation.extend(0.0)),
                texture,
                ..default()
            },
            platform: Platform,
            image_scale_mode: ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("images/platform1.png");

    commands.spawn(PlatformBundle::new(
        texture.clone(),
        400.,
        PLATFORM_TEXTURE_SIZE,
        Vec2::new(0., -180.),
    ));

    commands.spawn(PlatformBundle::new(
        texture.clone(),
        PLATFORM_TEXTURE_SIZE * 2.,
        20.,
        Vec2::new(-100., -75.),
    ));

    commands.spawn(PlatformBundle::new(
        texture.clone(),
        PLATFORM_TEXTURE_SIZE * 2.,
        20.,
        Vec2::new(100., 0.),
    ));

    commands.insert_resource(HighestPlatformPos(Vec2::new(100., 0.)));
}

fn add_platforms(
    mut commands: Commands,
    material_handles: Res<MaterialHandles>,
    height: Res<Height>,
    mut highest_platform_pos: ResMut<HighestPlatformPos>,
) {
    if height.0 > highest_platform_pos.y - WORLD_SIZE / 2. {
        highest_platform_pos.y += 100.;
        let mut new_x = highest_platform_pos.x;
        let mut diff_x = 0.;
        while !(50. ..200.).contains(&diff_x) {
            new_x = rand::random::<f32>() * 300. - 150.;
            diff_x = (new_x - highest_platform_pos.x).abs();
        }
        highest_platform_pos.x = new_x;

        let mut rng = rand::thread_rng();
        commands.spawn(PlatformBundle::new(
            material_handles.platforms[rng.gen_range(0..material_handles.platforms.len())].clone(),
            92.,
            20.,
            **highest_platform_pos,
        ));
    }
}

fn remove_platforms(
    mut commands: Commands,
    height: Res<Height>,
    platform_query: Query<(Entity, &Transform), With<Platform>>,
) {
    for (entity, transform) in platform_query.iter() {
        if transform.translation.y < height.0 - WORLD_SIZE / 2. - 20. {
            commands.entity(entity).despawn();
        }
    }
}
