use crate::{Height, MovementState, WORLD_SIZE};
use bevy::app::{App, Plugin, Startup};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::math::Vec2;
use bevy::prelude::{default, Camera, Camera2dBundle, Commands, FixedUpdate, Query, Res, With};
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, camera_scroll);
    }
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: WORLD_SIZE,
        min_height: WORLD_SIZE,
    };
    commands.spawn((
        camera,
        BloomSettings::default(),
        MovementState::new(Vec2::ZERO),
    ));
}

fn camera_scroll(
    mut query_camera_movement: Query<&mut MovementState, With<Camera>>,
    height: Res<Height>,
) {
    for mut movement in query_camera_movement.iter_mut() {
        let movement = &mut *movement;
        movement.old_position = movement.position;
        movement.position = Vec2::new(0., height.0);
    }
}
