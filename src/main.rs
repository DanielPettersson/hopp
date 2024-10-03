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

static WORLD_SIZE: f32 = 400.;

#[derive(Resource)]
struct MaterialHandles {
    red: Handle<ColorMaterial>,
    platforms: [Handle<Image>; 3],
}

#[derive(Resource)]
struct MeshHandles {
    rectangle: Mesh2dHandle,
}

#[derive(Component)]
struct MovementState {
    position: Vec2,
    old_position: Vec2,
}

impl MovementState {
    fn new(position: Vec2) -> Self {
        Self {
            position,
            old_position: position,
        }
    }
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
        .add_systems(Startup, setup)
        .add_systems(Update, update_movement)
        .add_systems(FixedUpdate, increase_height)
        .insert_resource(SubstepCount(6))
        .insert_resource(Gravity(Vec2::NEG_Y * 981.0))
        .insert_resource(Height(0.0))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let red = materials.add(Color::srgb(1., 0., 0.));
    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(1., 1.)));

    let platform_textures = [
        asset_server.load("images/platform1.png"),
        asset_server.load("images/platform2.png"),
        asset_server.load("images/platform3.png"),
    ];

    commands.insert_resource(MaterialHandles {
        red: red.clone(),
        platforms: platform_textures,
    });
    commands.insert_resource(MeshHandles {
        rectangle: rectangle.clone(),
    });
}

fn update_movement(
    fixed_time: Res<Time<Fixed>>,
    mut movement_query: Query<(&mut Transform, &MovementState)>,
) {
    for (mut transform, state) in movement_query.iter_mut() {
        transform.translation = state
            .old_position
            .lerp(state.position, fixed_time.overstep_fraction())
            .extend(transform.translation.z);
    }
}

fn increase_height(time: Res<Time>, mut height: ResMut<Height>) {
    if height.0 > 50. {
        height.0 += time.delta_seconds() * 10.0;
    }
}
