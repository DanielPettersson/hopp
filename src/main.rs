mod camera;
mod drag;
mod player;
mod score;
mod world;

use crate::camera::CameraPlugin;
use crate::drag::DragPlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::world::WorldPlugin;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_asset_loader::prelude::{
    AssetCollection, ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};

static WORLD_SIZE: f32 = 400.;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Loading,
    InGame,
    GameOver,
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(
        paths("images/platform1.png", "images/platform2.png", "images/platform3.png"),
        collection(typed)
    )]
    platforms: Vec<Handle<Image>>,
}

#[derive(AssetCollection, Resource)]
struct FontAssets {
    #[asset(path = "fonts/segmental.ttf")]
    segmental: Handle<Font>,
}

#[derive(Resource)]
struct MaterialHandles {
    red: Handle<ColorMaterial>,
    red_transparent: Handle<ColorMaterial>,
    yellow: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct MeshHandles {
    rectangle: Mesh2dHandle,
    rectangle_2: Mesh2dHandle,
}

#[derive(Resource)]
struct Height(f32);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(100.0),
            DragPlugin,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            ScorePlugin,
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::InGame)
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            increase_height.run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnEnter(GameState::InGame), reset_game)
        .add_systems(
            Update,
            restart_game.run_if(in_state(GameState::GameOver)),
        )
        .insert_resource(SubstepCount(6))
        .insert_resource(Gravity(Vec2::NEG_Y * 981.0))
        .insert_resource(Height(0.0))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 1.0)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(MaterialHandles {
        red: materials.add(Color::srgb(1., 0., 0.)),
        red_transparent: materials.add(Color::srgba(1., 0., 0., 0.5)),
        yellow: materials.add(Color::srgb(3., 3., 0.)),
    });
    commands.insert_resource(MeshHandles {
        rectangle: Mesh2dHandle(meshes.add(Rectangle::new(1., 1.))),
        rectangle_2: Mesh2dHandle(meshes.add(Rectangle::new(2., 2.))),
    });
}

fn increase_height(time: Res<Time>, mut height: ResMut<Height>) {
    if height.0 > 50. {
        height.0 += time.delta_seconds() * 15.0;
    }
}

fn reset_game(mut height: ResMut<Height>) {
    height.0 = 0.;
}

fn restart_game(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if keys.get_just_pressed().len() > 0 || buttons.get_just_pressed().len() > 0 {
        next_state.set(GameState::InGame);
    }
}
