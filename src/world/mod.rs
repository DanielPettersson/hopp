mod r#box;
mod platform;

use crate::world::platform::{Platform, PlatformBundle, PlatformsPlugin};
use crate::{GameState, ImageAssets};
use bevy::app::App;
use bevy::prelude::{
    in_state, Camera, Commands, Component, Entity, FixedUpdate, GlobalTransform, IntoSystemConfigs,
    OnEnter, OnExit, Plugin, Query, Res, ResMut, Resource, Sprite, Time, Timer, TimerMode,
    Transform, Vec2, Vec3, Window, With,
};
use bevy::window::PrimaryWindow;
use rand::Rng;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlatformsPlugin)
            .insert_resource(HighestPlatformInfo::default())
            .insert_resource(DespawnTimer {
                timer: Timer::from_seconds(1., TimerMode::Repeating),
            })
            .add_systems(OnEnter(GameState::InGame), create_initial_world_entities)
            .add_systems(
                FixedUpdate,
                (add_platforms, remove_scrolled_out_world_entities)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::GameOver), remove_all_world_entities);
    }
}

#[derive(Resource)]
struct DespawnTimer {
    timer: Timer,
}

#[derive(Resource, Default)]

struct HighestPlatformInfo {
    pos: Vec2,
    platform: Platform,
}

#[derive(Component)]
struct WorldEntity;

fn create_initial_world_entities(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut highest_platform: ResMut<HighestPlatformInfo>,
    mut platform_despawn_timer: ResMut<DespawnTimer>,
) {
    platform_despawn_timer.timer.reset();

    commands.spawn(PlatformBundle::new(
        images.platforms[0].clone(),
        10000.,
        1000.,
        Vec3::new(0., -680., 10.),
        Platform::Static,
    ));

    highest_platform.pos = Vec2::new(0., -180.);
    highest_platform.platform = Platform::Static;
}

fn remove_all_world_entities(
    mut commands: Commands,
    mut query_world_entity: Query<Entity, With<WorldEntity>>,
) {
    for entity in query_world_entity.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn add_platforms(
    mut commands: Commands,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    images: Res<ImageAssets>,
    mut highest_platform: ResMut<HighestPlatformInfo>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window_top = camera
        .viewport_to_world_2d(camera_transform, Vec2::ZERO)
        .unwrap_or(Vec2::ZERO)
        .y;

    if window_top > highest_platform.pos.y {
        let mut rng = rand::thread_rng();

        let mut pos = highest_platform.pos;
        pos.y += rng.gen_range(60.0..75.0);
        let mut new_x = pos.x;
        let mut diff_x = 0.;
        while !(75. ..190.).contains(&diff_x) {
            new_x = rng.gen_range(-150.0..150.);
            diff_x = (new_x - pos.x).abs();
        }
        pos.x = new_x;

        let platform = if highest_platform.platform == Platform::Static {
            match rng.gen() {
                0.0..0.4 => Platform::Static,
                0.4..0.7 => Platform::Hanging,
                _ => Platform::Moving {
                    velocity: rng.gen_range(0.5..1.0),
                    range: rng.gen_range(20. ..40.),
                },
            }
        } else {
            Platform::Static
        };

        platform.spawn(&mut commands, &images, pos);

        highest_platform.pos = pos;
        highest_platform.platform = platform;
    }
}

fn remove_scrolled_out_world_entities(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mut despawn_timer: ResMut<DespawnTimer>,
    time: Res<Time>,
    query_world_entity: Query<(Entity, &Sprite, &Transform), With<WorldEntity>>,
) {
    despawn_timer.timer.tick(time.delta());

    if despawn_timer.timer.finished() {
        let (camera, camera_transform) = query_camera.single();
        let window_size = query_window.single().size();
        let window_bottom = camera
            .viewport_to_world_2d(camera_transform, window_size)
            .unwrap_or(Vec2::ZERO)
            .y;

        for (entity, sprite, transform) in query_world_entity.iter() {
            let sprite_half_height = sprite.custom_size.unwrap_or(Vec2::ZERO).y / 2.;
            if transform.translation.y < window_bottom - sprite_half_height - 100. {
                commands.entity(entity).despawn();
            }
        }
    }
}
