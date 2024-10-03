use crate::MovementState;
use bevy::app::{App, Plugin, Startup};
use bevy::math::Vec2;
use bevy::prelude::{Camera, Camera2dBundle, Commands, FixedUpdate, Query, Res, Time, With};
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(FixedUpdate, camera_scroll);
    }
}

fn setup(
    mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 400.0,
        min_height: 400.0,
    };
    commands.spawn((camera, MovementState::new(Vec2::ZERO, Vec2::new(0., 5.))));
}

fn camera_scroll(
    mut query_camera_movement: Query<&mut MovementState, With<Camera>>,
    time: Res<Time>,
) {
    for mut movement in query_camera_movement.iter_mut() {
        let movement = &mut *movement;
        movement.old_position = movement.position;
        movement.position += movement.velocity * time.delta_seconds();
    }
}