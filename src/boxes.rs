use crate::camera::MainCamera;
use crate::{GameState, Height, ImageAssets};
use avian2d::collision::Collider;
use avian2d::prelude::{AngularVelocity, LinearVelocity, RigidBody};
use bevy::app::{App, FixedUpdate};
use bevy::asset::Handle;
use bevy::math::Vec2;
use bevy::prelude::{
    default, in_state, Bundle, Camera, Commands, Component, Entity, GlobalTransform, Image,
    IntoSystemConfigs, OnExit, Plugin, Query, Res, ResMut, Resource, Sprite, SpriteBundle, Time,
    Timer, TimerMode, Transform, Vec3, Window, With,
};
use bevy::window::PrimaryWindow;
use rand::Rng;
use std::time::Duration;

pub struct BoxesPlugin;

impl Plugin for BoxesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoxSpawnTimer {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        })
        .add_systems(
            FixedUpdate,
            (spawn_boxes, remove_boxes).run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(GameState::GameOver), remove_all_boxes);
    }
}

#[derive(Resource)]
struct BoxSpawnTimer {
    timer: Timer,
}

#[derive(Component)]
struct Box;

#[derive(Bundle)]
struct BoxBundle {
    rigid_body: RigidBody,
    collider: Collider,
    sprite: SpriteBundle,
    r#box: Box,
    linear_velocity: LinearVelocity,
    angular_velocity: AngularVelocity,
}

impl BoxBundle {
    fn new(
        translation: Vec3,
        linear_velocity: LinearVelocity,
        angular_velocity: AngularVelocity,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(10.0, 10.0),
            sprite: SpriteBundle {
                transform: Transform::from_translation(translation),
                texture,
                ..default()
            },
            r#box: Box,
            linear_velocity,
            angular_velocity,
        }
    }
}

fn spawn_boxes(
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    images: Res<ImageAssets>,
    height: Res<Height>,
    mut commands: Commands,
    mut box_spawn_timer: ResMut<BoxSpawnTimer>,
    time: Res<Time>,
) {
    box_spawn_timer.timer.tick(time.delta());
    
    if height.0 > 50. && box_spawn_timer.timer.finished() {
        let (camera, camera_transform) = query_camera.single();
        let window_top = camera
            .viewport_to_world_2d(camera_transform, Vec2::ZERO)
            .unwrap_or(Vec2::ZERO)
            .y;

        let mut rng = rand::thread_rng();

        commands.spawn(BoxBundle::new(
            Vec3::new(rng.gen_range(-200.0..200.), window_top + 50., 0.),
            LinearVelocity(Vec2::new(rng.gen_range(-10.0..10.), 0.)),
            AngularVelocity(rng.gen_range(-1.0..1.)),
            images.boxes[rng.gen_range(0..images.boxes.len())].clone(),
        ));
    }
}

fn remove_boxes(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    boxes_query: Query<(Entity, &Sprite, &Transform), With<Box>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window_size = query_window.single().size();
    let window_bottom = camera
        .viewport_to_world_2d(camera_transform, window_size)
        .unwrap_or(Vec2::ZERO)
        .y;

    for (entity, sprite, transform) in boxes_query.iter() {
        let sprite_half_height = sprite.custom_size.unwrap_or(Vec2::ZERO).y / 2.;
        if transform.translation.y < window_bottom - sprite_half_height - 100. {
            commands.entity(entity).despawn();
        }
    }
}

fn remove_all_boxes(mut commands: Commands, mut boxes_query: Query<Entity, With<Box>>) {
    for entity in boxes_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
