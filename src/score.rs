use crate::{FontAssets, GameState};
use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::{
    default, in_state, Camera, Color, Commands, Component, Entity, FixedUpdate, GlobalTransform,
    IntoSystemConfigs, JustifyText, Local, OnEnter, OnExit, Plugin, Query, Res, ResMut, Resource,
    Text, Text2dBundle, TextStyle, Transform, Update, Vec3, With,
};
use bevy::sprite::Anchor;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(OnEnter(GameState::InGame), create_score_text)
            .add_systems(OnExit(GameState::InGame), remove_score_text)
            .add_systems(Update, scroll_score.run_if(in_state(GameState::InGame)))
            .add_systems(
                FixedUpdate,
                update_score.run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnEnter(GameState::GameOver), create_game_over)
            .add_systems(OnExit(GameState::GameOver), remove_score_text);
    }
}

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

fn create_score_text(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    mut score: ResMut<Score>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
) {
    score.0 = 0;
    let (camera, camera_transform) = query_camera.single();
    let score_pos = camera
        .viewport_to_world_2d(camera_transform, Vec2::new(10., 10.))
        .unwrap_or(Vec2::ZERO);

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
            transform: Transform::from_translation(score_pos.extend(10.)),
            ..default()
        },
        ScoreText,
    ));
}

fn remove_score_text(mut commands: Commands, query_score_text: Query<Entity, With<ScoreText>>) {
    for entity in query_score_text.iter() {
        commands.entity(entity).despawn();
    }
}

fn scroll_score(
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mut score_query: Query<&mut Transform, With<ScoreText>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let score_pos = camera
        .viewport_to_world_2d(camera_transform, Vec2::new(10., 10.))
        .unwrap_or(Vec2::ZERO);

    for mut transform in score_query.iter_mut() {
        transform.translation = score_pos.extend(10.);
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

fn create_game_over(mut commands: Commands, fonts: Res<FontAssets>, score: Res<Score>) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Game Over\nScore {}", score.0),
                TextStyle {
                    font: fonts.segmental.clone(),
                    font_size: 70.0,
                    color: Color::srgb(2.0, 2.0, 0.0),
                },
            )
            .with_justify(JustifyText::Center),
            text_anchor: Anchor::Center,
            transform: Transform::from_translation(Vec3::new(0., 50., 0.)),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Press any key to play again",
                TextStyle {
                    font: fonts.segmental.clone(),
                    font_size: 30.0,
                    color: Color::srgb(1.5, 1.5, 1.5),
                },
            )
            .with_justify(JustifyText::Center),
            text_anchor: Anchor::Center,
            transform: Transform::from_translation(Vec3::new(0., -50., 0.)),
            ..default()
        },
        ScoreText,
    ));
}
