mod drag;
mod player;
mod camera;
mod world;
mod score;

use crate::camera::CameraPlugin;
use crate::drag::DragPlugin;
use crate::player::PlayerPlugin;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use crate::score::ScorePlugin;
use crate::world::WorldPlugin;

static WORLD_SIZE: f32 = 400.;

#[derive(Resource)]
struct MaterialHandles {
    red: Handle<ColorMaterial>,
    green: Handle<ColorMaterial>,
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
            old_position: position
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
) {
    let red = materials.add(Color::srgb(1., 0., 0.));
    let green = materials.add(Color::srgb(0., 1., 0.));
    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(1., 1.)));

    commands.insert_resource(MaterialHandles { red: red.clone(), green: green.clone() });
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

fn increase_height(
    time: Res<Time>,
    mut height: ResMut<Height>,

) {
    height.0 += time.delta_seconds() * 5.0;
}
