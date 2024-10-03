use crate::{Height, WORLD_SIZE};
use bevy::app::App;
use bevy::prelude::{default, AssetServer, Camera, Commands, Component, FixedUpdate, GlobalTransform, JustifyText, Local, Plugin, Query, Res, Startup, Text, Text2dBundle, TextStyle, Transform, Update, Vec3, With};
use bevy::sprite::Anchor;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, scroll_score).add_systems(FixedUpdate, update_score);
    }
}

#[derive(Component)]
pub struct Score;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/segmental.ttf");

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "100",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
            )
            .with_justify(JustifyText::Left),
            text_anchor: Anchor::TopLeft,
            transform: Transform::from_translation(Vec3::new(
                -WORLD_SIZE / 2. + 10.,
                WORLD_SIZE / 2.,
                0.0,
            )),
            ..default()
        },
        Score,
    ));
}

fn scroll_score(query_camera: Query<&GlobalTransform, With<Camera>>, mut score_query: Query<&mut Transform, With<Score>>) {
    for mut transform in score_query.iter_mut() {
        transform.translation.y = query_camera.single().translation().y + WORLD_SIZE / 2. - 10.;
    }
}

fn update_score(mut score: Local<u32>, height: Res<Height>, mut score_query: Query<&mut Text, With<Score>>) {
    if *score < height.0 as u32 {
        *score = height.0 as u32;
        for mut text in score_query.iter_mut() {
            text.sections[0].value = format!("{}", *score);
        }
    }
}
