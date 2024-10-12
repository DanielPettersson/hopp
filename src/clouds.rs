use crate::{GameState, Height, ImageAssets};
use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{default, in_state, Bundle, Camera, Commands, Component, Entity, FixedUpdate, GlobalTransform, Image, IntoSystemConfigs, OnEnter, OnExit, Plugin, Query, Res, ResMut, Resource, SpriteBundle, Time, Timer, Transform, Update, Vec2, Vec3, Window, With};
use bevy::time::TimerMode;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::camera::MainCamera;

static CLOUD_MAX_WIDTH: f32 = 220.0;
static CLOUD_MAX_HALF_WIDTH: f32 = CLOUD_MAX_WIDTH / 2.0;

pub struct CloudsPlugin;

impl Plugin for CloudsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CloudSpawnConfig {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
        })
            .add_systems(OnEnter(GameState::InGame), add_initial_clouds)
            .add_systems(OnExit(GameState::InGame), remove_all_clouds)
        .add_systems(
            FixedUpdate,
            (add_clouds, remove_clouds).run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, scroll_clouds);
    }
}

#[derive(Component)]
struct Cloud {
    height: f32,
    velocity: f32,
}

#[derive(Bundle)]
struct CloudBundle {
    cloud: Cloud,
    sprite: SpriteBundle,
}

#[derive(Resource)]
struct CloudSpawnConfig {
    timer: Timer,
}

impl CloudBundle {
    fn new(texture: Handle<Image>, translation: Vec3, velocity: f32) -> Self {
        Self {
            cloud: Cloud {
                height: translation.y,
                velocity },
            sprite: SpriteBundle {
                transform: Transform::from_translation(translation),
                texture,
                ..default()
            },
        }
    }
}

fn remove_all_clouds(mut commands: Commands, query_clouds: Query<Entity, With<Cloud>>) {
    for entity in query_clouds.iter() {
        commands.entity(entity).despawn();
    }
}

fn add_initial_clouds(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    images: Res<ImageAssets>,
) {
    
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let (camera, camera_transform) = query_camera.single();
        let window_size = query_window.single().size();
        let window_top_left = camera
            .viewport_to_world_2d(camera_transform, Vec2::ZERO)
            .unwrap_or(Vec2::ZERO);
        let window_bottom_right = camera
            .viewport_to_world_2d(camera_transform, window_size)
            .unwrap_or(Vec2::ZERO);

        commands.spawn(CloudBundle::new(
            images.clouds[rng.gen_range(0..images.clouds.len())].clone(),
            Vec3::new(
                rng.gen_range(window_top_left.x..window_bottom_right.x),
                rng.gen_range(window_bottom_right.y..window_top_left.y),
                -10.,
            ),
            rng.gen_range(15. ..20.),
        ));
    }
}

fn add_clouds(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    images: Res<ImageAssets>,
    mut cloud_spawn_config: ResMut<CloudSpawnConfig>,
    time: Res<Time>,
) {
    cloud_spawn_config.timer.tick(time.delta());

    if cloud_spawn_config.timer.finished() {
        let mut rng = rand::thread_rng();
        let (camera, camera_transform) = query_camera.single();
        let window_size = query_window.single().size();
        let window_top_left = camera
            .viewport_to_world_2d(camera_transform, Vec2::ZERO)
            .unwrap_or(Vec2::ZERO);
        let window_bottom_right = camera
            .viewport_to_world_2d(camera_transform, window_size)
            .unwrap_or(Vec2::ZERO);

        commands.spawn(CloudBundle::new(
            images.clouds[rng.gen_range(0..images.clouds.len())].clone(),
            Vec3::new(
                window_top_left.x - CLOUD_MAX_HALF_WIDTH,
                rng.gen_range(window_bottom_right.y..window_top_left.y),
                -10.,
            ),
            rng.gen_range(20. ..30.),
        ));
    }
}

fn remove_clouds(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    cloud_query: Query<(Entity, &Transform), With<Cloud>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window_size = query_window.single().size();
    let window_right = camera
        .viewport_to_world_2d(camera_transform, window_size)
        .unwrap_or(Vec2::ZERO)
        .x;

    for (entity, transform) in cloud_query.iter() {
        if transform.translation.x > window_right + CLOUD_MAX_HALF_WIDTH {
            commands.entity(entity).despawn();
        }
    }
}

fn scroll_clouds(time: Res<Time>, height: Res<Height>, mut cloud_query: Query<(&mut Transform, &Cloud)>) {
    for (mut transform, cloud) in cloud_query.iter_mut() {
        transform.translation.x += time.delta_seconds() * cloud.velocity;
        let target_height = cloud.height + height.0 / 2.;
        let y_diff = target_height - transform.translation.y;
        transform.translation.y += y_diff * 0.1 
    }
}
