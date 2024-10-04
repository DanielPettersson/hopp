use crate::{GameState, Height, WORLD_SIZE};
use bevy::app::{App, Plugin, Startup};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::{
    default, in_state, Camera, Camera2dBundle, Commands, IntoSystemConfigs, OnExit, Query, Res,
    Transform, Update, With,
};
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, camera_scroll.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), reset_camera_position);
    }
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::default(),
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: WORLD_SIZE,
        min_height: WORLD_SIZE,
    };
    commands.spawn((camera, BloomSettings::default()));
}

fn camera_scroll(
    mut query_camera_movement: Query<&mut Transform, With<Camera>>,
    height: Res<Height>,
) {
    for mut transform in query_camera_movement.iter_mut() {
        let transform_y_diff = (height.0 - transform.translation.y) * 0.05;
        transform.translation.y += transform_y_diff;
    }
}

fn reset_camera_position(mut query_camera_movement: Query<&mut Transform, With<Camera>>) {
    for mut transform in query_camera_movement.iter_mut() {
        *transform = Transform::default();
    }
}
