use crate::{GameState, MaterialHandles, MeshHandles, HALF_WORLD_SIZE};
use bevy::app::App;
use bevy::prelude::{default, in_state, Camera, Commands, Component, Entity, GlobalTransform, IntoSystemConfigs, OnEnter, OnExit, Plugin, Query, Res, Transform, Update, Vec3, With};
use bevy::sprite::MaterialMesh2dBundle;

pub struct GameOverLinePlugin;

impl Plugin for GameOverLinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), create_game_over_line)
            .add_systems(OnExit(GameState::InGame), remove_game_over_line)
            .add_systems(Update, scroll_game_over_line.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
struct GameOverLine;

fn create_game_over_line(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    material_handles: Res<MaterialHandles>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handles.rectangle.clone(),
            material: material_handles.red_transparent.clone(),
            transform: Transform::from_xyz(0., -HALF_WORLD_SIZE - 6., 2.).with_scale(Vec3::new(10000., 10., 1.)),
            ..default()
        },
        GameOverLine,
    ));
}

fn remove_game_over_line(mut commands: Commands, query_game_over_line: Query<Entity, With<GameOverLine>>) {
    for entity in query_game_over_line.iter() {
        commands.entity(entity).despawn();
    }
}

fn scroll_game_over_line(
    query_camera: Query<&GlobalTransform, With<Camera>>,
    mut game_over_line_query: Query<&mut Transform, With<GameOverLine>>,
) {
    for mut transform in game_over_line_query.iter_mut() {
        transform.translation.y = query_camera.single().translation().y - HALF_WORLD_SIZE - transform.scale.y - 1.;
    }
}
