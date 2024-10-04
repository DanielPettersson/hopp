use crate::{FontAssets, GameState, Height, WORLD_SIZE};
use bevy::app::App;
use bevy::prelude::{default, in_state, Camera, Color, Commands, Component, FixedUpdate, GlobalTransform, IntoSystemConfigs, JustifyText, Local, OnEnter, Plugin, Query, Res, Text, Text2dBundle, TextStyle, Transform, Update, Vec3, With};
use bevy::sprite::Anchor;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), create_score)
            .add_systems(Update, scroll_score.run_if(in_state(GameState::InGame)))
            .add_systems(FixedUpdate, update_score.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Score;

fn create_score(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: fonts.segmental.clone(),
                    font_size: 30.0,
                    color: Color::srgb(1.5, 1.5, 1.5),
                },
            )
            .with_justify(JustifyText::Left),
            text_anchor: Anchor::TopLeft,
            transform: Transform::from_translation(Vec3::new(
                -WORLD_SIZE / 2. + 10.,
                WORLD_SIZE / 2.,
                2.0,
            )),
            ..default()
        },
        Score,
    ));
}

fn scroll_score(
    query_camera: Query<&GlobalTransform, With<Camera>>,
    mut score_query: Query<&mut Transform, With<Score>>,
) {
    for mut transform in score_query.iter_mut() {
        transform.translation.y = query_camera.single().translation().y + WORLD_SIZE / 2. - 10.;
    }
}

fn update_score(
    mut score: Local<u32>,
    height: Res<Height>,
    mut score_query: Query<&mut Text, With<Score>>,
) {
    if *score < height.0 as u32 {
        *score = height.0 as u32;
        for mut text in score_query.iter_mut() {
            text.sections[0].value = format!("{}", *score);
        }
    }
}
