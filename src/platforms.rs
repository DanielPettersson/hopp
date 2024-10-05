use crate::{GameState, Height, ImageAssets};
use avian2d::collision::Collider;
use avian2d::prelude::RigidBody;
use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{default, in_state, Bundle, Camera, Commands, Component, Entity, FixedUpdate, GlobalTransform, Image, ImageScaleMode, IntoSystemConfigs, OnEnter, OnExit, Plugin, Query, Rect, Res, ResMut, Resource, Sprite, SpriteBundle, Transform, Vec2, Vec3, Window, With};
use bevy::window::PrimaryWindow;
use rand::Rng;
use std::ops::{Deref, DerefMut};

static PLATFORM_TEXTURE_SIZE: f32 = 46.;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HighestPlatformPos::default())
            .add_systems(OnEnter(GameState::InGame), create_initial_platforms)
            .add_systems(OnExit(GameState::InGame), remove_all_platforms)
            .add_systems(
                FixedUpdate,
                (add_platforms, remove_platforms).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
struct Platform;

#[derive(Resource, Default)]

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
    fn new(texture: Handle<Image>, width: f32, height: f32, translation: Vec3) -> Self {
        Self {
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(width, height),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(width, height)),
                    rect: Some(Rect::new(0., 0., PLATFORM_TEXTURE_SIZE, height)),
                    ..default()
                },
                transform: Transform::from_translation(translation),
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

fn create_initial_platforms(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut highest_platform_pos: ResMut<HighestPlatformPos>,
) {
    commands.spawn(PlatformBundle::new(
        images.platforms[0].clone(),
        10000.,
        1000.,
        Vec3::new(0., -680., 10.),
    ));

    highest_platform_pos.0 = Vec2::new(0., -180.);
}

fn remove_all_platforms(mut commands: Commands, query_platforms: Query<Entity, With<Platform>>) {
    for entity in query_platforms.iter() {
        commands.entity(entity).despawn();
    }
}

fn add_platforms(
    mut commands: Commands,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    images: Res<ImageAssets>,
    height: Res<Height>,
    mut highest_platform_pos: ResMut<HighestPlatformPos>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window_top = camera
        .viewport_to_world_2d(camera_transform, Vec2::ZERO)
        .unwrap_or(Vec2::ZERO)
        .y;
    
    if height.0 > highest_platform_pos.y - window_top {
        let mut rng = rand::thread_rng();

        highest_platform_pos.y += rng.gen_range(65.0..85.0);
        let mut new_x = highest_platform_pos.x;
        let mut diff_x = 0.;
        while !(75. ..190.).contains(&diff_x) {
            new_x = rng.gen_range(-150.0..150.);
            diff_x = (new_x - highest_platform_pos.x).abs();
        }
        highest_platform_pos.x = new_x;

        commands.spawn(PlatformBundle::new(
            images.platforms[rng.gen_range(0..images.platforms.len())].clone(),
            92.,
            20.,
            highest_platform_pos.extend(0.),
        ));
    }
}

fn remove_platforms(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    platform_query: Query<(Entity, &Sprite, &Transform), With<Platform>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window_size = query_window.single().size();
    let window_bottom = camera
        .viewport_to_world_2d(camera_transform, window_size)
        .unwrap_or(Vec2::ZERO)
        .y;

    for (entity, sprite, transform) in platform_query.iter() {
        if transform.translation.y < window_bottom - sprite.custom_size.unwrap_or(Vec2::ZERO).y / 2.
        {
            commands.entity(entity).despawn();
        }
    }
}
