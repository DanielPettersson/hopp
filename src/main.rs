mod camera;
mod clouds;
mod drag;
mod game_over_line;
mod platforms;
mod player;
mod score;

use crate::camera::CameraPlugin;
use crate::clouds::CloudsPlugin;
use crate::drag::DragPlugin;
use crate::game_over_line::GameOverLinePlugin;
use crate::platforms::PlatformsPlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::texture::{ImageFilterMode, ImageSamplerDescriptor};
use bevy::sprite::Mesh2dHandle;
use bevy::window::PrimaryWindow;
use bevy_asset_loader::prelude::{
    AssetCollection, ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use bevy_magic_light_2d::prelude::{
    setup_post_processing_camera, BevyMagicLight2DPlugin, BevyMagicLight2DSettings,
    LightPassParams, OmniLightSource2D,
};
use bevy_persistent::prelude::*;
use bevy_persistent_windows::prelude::*;
use std::path::PathBuf;

static WORLD_SIZE: f32 = 400.;
static HALF_WORLD_SIZE: f32 = WORLD_SIZE / 2.;

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
    #[asset(
        paths(
            "images/cloud1.png",
            "images/cloud2.png",
            "images/cloud3.png",
            "images/cloud4.png",
            "images/cloud5.png"
        ),
        collection(typed)
    )]
    clouds: Vec<Handle<Image>>,

    #[asset(path = "images/bolt.png")]
    bolt: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
struct FontAssets {
    #[asset(path = "fonts/segmental.ttf")]
    segmental: Handle<Font>,
}

#[derive(Resource)]
struct MaterialHandles {
    black: Handle<ColorMaterial>,
    red: Handle<ColorMaterial>,
    red_transparent: Handle<ColorMaterial>,
    bright_red: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct MeshHandles {
    rectangle: Mesh2dHandle,
    rectangle_2: Mesh2dHandle,
}

#[derive(Resource)]
struct Height(f32);

#[derive(Component)]
struct GlobalLight;

fn main() {
    let mut app = App::new();

    let window_plugin = WindowPlugin {
        primary_window: None,
        ..Default::default()
    };
    app.add_plugins(
        DefaultPlugins
            .set(window_plugin)
            .set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    mag_filter: ImageFilterMode::Nearest,
                    min_filter: ImageFilterMode::Nearest,
                    ..default()
                },
            })
            .build(),
    );

    app.world_mut().spawn((
        PrimaryWindow,
        PersistentWindowBundle {
            window: Window {
                title: "Hopp!".to_owned(),
                ..Default::default()
            },
            state: Persistent::<WindowState>::builder()
                .name("primary window state")
                .format(StorageFormat::Json)
                .path(get_state_directory().join("primary-window.json"))
                .default(WindowState::windowed(1280, 720))
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to create the persistent primary window state"),
        },
    ));
    app.insert_resource(BevyMagicLight2DSettings {
        light_pass_params: LightPassParams {
            smooth_kernel_size: (4, 4),
            ..default()
        },
        ..default()
    })
    .add_plugins((
        BevyMagicLight2DPlugin,
        PersistentWindowsPlugin,
        PhysicsPlugins::default().with_length_unit(100.0),
        DragPlugin,
        PlayerPlugin,
        CameraPlugin,
        PlatformsPlugin,
        GameOverLinePlugin,
        ScorePlugin,
        CloudsPlugin,
    ))
    .init_state::<GameState>()
    .add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::InGame)
            .load_collection::<ImageAssets>()
            .load_collection::<FontAssets>(),
    )
    .add_systems(Startup, setup.after(setup_post_processing_camera))
    .add_systems(
        FixedUpdate,
        increase_height.run_if(in_state(GameState::InGame)),
    )
    .add_systems(Update, light_scroll.run_if(in_state(GameState::InGame)))
    .add_systems(
        Update,
        game_over_lights.run_if(in_state(GameState::GameOver)),
    )
    .add_systems(Update, restart_game.run_if(in_state(GameState::GameOver)))
    .add_systems(OnExit(GameState::GameOver), cleanup_game)
    .insert_resource(SubstepCount(6))
    .insert_resource(Gravity(Vec2::NEG_Y * 981.0))
    .insert_resource(Height(0.0))
    .insert_resource(ClearColor(Color::srgb(0.46, 0.58, 1.0)))
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(MaterialHandles {
        black: materials.add(Color::srgb(0.1, 0.1, 0.1)),
        red: materials.add(Color::srgb(1., 0., 0.)),
        red_transparent: materials.add(Color::srgba(1., 0., 0., 0.5)),
        bright_red: materials.add(Color::srgb(4., 0., 0.)),
    });
    commands.insert_resource(MeshHandles {
        rectangle: Mesh2dHandle(meshes.add(Rectangle::new(1., 1.))),
        rectangle_2: Mesh2dHandle(meshes.add(Rectangle::new(2., 2.))),
    });

    commands.spawn((
        OmniLightSource2D {
            intensity: 1.0,
            color: Color::WHITE,
            falloff: Vec3::new(50., 50., 0.005),
            ..default()
        },
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(250., 150., 0.)),
            ..default()
        },
        GlobalLight,
    ));

    commands.spawn((
        OmniLightSource2D {
            intensity: 0.5,
            color: Color::WHITE,
            falloff: Vec3::new(50., 50., 0.005),
            ..default()
        },
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(-250., 150., 0.)),
            ..default()
        },
        GlobalLight,
    ));
}

fn light_scroll(
    mut query_light_movement: Query<&mut Transform, With<GlobalLight>>,
    height: Res<Height>,
) {
    for mut transform in query_light_movement.iter_mut() {
        let transform_y_diff = (height.0 + 150.0 - transform.translation.y) * 0.05;
        transform.translation.y += transform_y_diff;
    }
}

fn game_over_lights(
    mut query_light_movement: Query<&mut Transform, With<GlobalLight>>,
    time: Res<Time>,
    height: Res<Height>,
) {
    for mut transform in query_light_movement.iter_mut() {
        let xx = time.elapsed_seconds().sin();
        let yy = time.elapsed_seconds().cos();
        transform.translation.x = transform.translation.x.signum() * (300. + xx * 50.);
        transform.translation.y = height.0 + 150. + yy * 50.;
    }
}

fn increase_height(time: Res<Time>, mut height: ResMut<Height>) {
    if height.0 > 50. {
        height.0 += time.delta_seconds() * 15.0;
    }
}

fn cleanup_game(
    mut height: ResMut<Height>,
    mut commands: Commands,
    query_joints: Query<Entity, With<DistanceJoint>>,
    mut query_light_movement: Query<&mut Transform, With<GlobalLight>>
) {
    height.0 = 0.;

    for entity in query_joints.iter() {
        commands.entity(entity).despawn();
    }

    for mut transform in query_light_movement.iter_mut() {
        transform.translation.x = transform.translation.x.signum() * 250.0;
        transform.translation.y = 150.0;
    }

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

fn get_state_directory() -> PathBuf {
    dirs::data_dir()
        .expect("failed to get the platforms data directory")
        .join("hopp")
        .join("state")
}
