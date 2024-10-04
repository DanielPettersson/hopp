use crate::{FontAssets, GameState, Height, WORLD_SIZE};
use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::{default, in_state, Camera, Color, Commands, Component, Event, EventReader, FixedUpdate, GlobalTransform, IntoSystemConfigs, JustifyText, Local, OnEnter, Plugin, Query, Res, ResMut, Resource, Text, Text2dBundle, TextStyle, Transform, Update, Vec3, With};
use bevy::sprite::Anchor;
use crate::drag::Drag;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(OnEnter(GameState::InGame), initialize_score_text)
            .add_systems(Update, scroll_score.run_if(in_state(GameState::InGame)))
            .add_systems(
                FixedUpdate,
                update_score.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

fn initialize_score_text(mut commands: Commands, fonts: Res<FontAssets>, mut score: ResMut<Score>) {
    score.0 = 0;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("{}", score.0),
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
        ScoreText,
    ));
}

fn scroll_score(
    query_camera: Query<&GlobalTransform, With<Camera>>,
    mut score_query: Query<&mut Transform, With<ScoreText>>,
) {
    for mut transform in score_query.iter_mut() {
        transform.translation.y = query_camera.single().translation().y + WORLD_SIZE / 2. - 10.;
    }
}

fn update_score(
    mut old_score: Local<u32>,
    score: Res<Score>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
) {
    if *old_score != score.0 {
        *old_score = score.0;
        for mut text in score_query.iter_mut() {
            text.sections[0].value = format!("{}", score.0);
        }
    }
}
