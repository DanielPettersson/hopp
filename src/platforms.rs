use crate::{GameState, Height, ImageAssets};
use avian2d::collision::Collider;
use avian2d::dynamics::solver::xpbd::XpbdConstraint;
use avian2d::position::{Position, Rotation};
use avian2d::prelude::{DistanceJoint, Joint, LinearVelocity, RigidBody};
use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{
    default, in_state, Bundle, Camera, Color, Commands, Component, Entity, FixedUpdate, Gizmos,
    GlobalTransform, Image, ImageScaleMode, IntoSystemConfigs, OnEnter, OnExit, Or, Plugin, Query,
    Rect, Res, ResMut, Resource, Sprite, SpriteBundle, Time, Transform, Update, Vec2, Vec3, Window,
    With,
};
use bevy::window::PrimaryWindow;
use bevy_magic_light_2d::prelude::LightOccluder2D;
use rand::Rng;
use crate::camera::MainCamera;

static PLATFORM_TEXTURE_SIZE: f32 = 46.;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HighestPlatformInfo::default())
            .add_systems(OnEnter(GameState::InGame), create_initial_platforms)
            .add_systems(OnExit(GameState::InGame), remove_all_platforms)
            .add_systems(
                FixedUpdate,
                (add_platforms, remove_platforms, scroll_platforms)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, draw_ropes);
    }
}

#[derive(Component, Clone, Default, PartialEq)]
enum Platform {
    #[default]
    Static,
    Hanging,
    Moving {
        velocity: f32,
        range: f32,
    },
}

impl Platform {
    fn get_rigid_body(&self) -> RigidBody {
        match self {
            Platform::Static => RigidBody::Static,
            Platform::Hanging => RigidBody::Dynamic,
            Platform::Moving {
                velocity: _,
                range: _,
            } => RigidBody::Kinematic,
        }
    }

    fn get_image_index(&self) -> usize {
        match self {
            Platform::Static => 0,
            Platform::Hanging => 1,
            Platform::Moving {
                velocity: _,
                range: _,
            } => 2,
        }
    }

    fn spawn(&self, commands: &mut Commands, images: &ImageAssets, pos: Vec2) {
        let platform = commands
            .spawn((
                PlatformBundle::new(
                    images.platforms[self.get_image_index()].clone(),
                    92.,
                    20.,
                    pos.extend(0.),
                    self.clone(),
                ),
                LightOccluder2D {
                    h_size: Vec2::new(46., 10.),
                },
            ))
            .id();

        if let Platform::Hanging = self {
            let bolt = commands
                .spawn(BoltBundle::new(
                    images.bolt.clone(),
                    pos.extend(0.) + Vec3::new(0., 50., -5.),
                ))
                .id();
            commands.spawn((
                Rope,
                DistanceJoint::new(bolt, platform)
                    .with_local_anchor_2(Vec2::new(40., 0.))
                    .with_rest_length(64.),
            ));
            commands.spawn((
                Rope,
                DistanceJoint::new(bolt, platform)
                    .with_local_anchor_2(Vec2::new(-40., 0.))
                    .with_rest_length(64.),
            ));
        };
    }
}

#[derive(Component)]
struct Bolt;

#[derive(Component)]
struct Rope;

#[derive(Resource, Default)]

struct HighestPlatformInfo {
    pos: Vec2,
    platform: Platform,
}

#[derive(Bundle)]
struct PlatformBundle {
    rigid_body: RigidBody,
    collider: Collider,
    sprite: SpriteBundle,
    platform: Platform,
    image_scale_mode: ImageScaleMode,
    linear_velocity: LinearVelocity,
}

impl PlatformBundle {
    fn new(
        texture: Handle<Image>,
        width: f32,
        height: f32,
        translation: Vec3,
        platform: Platform,
    ) -> Self {
        Self {
            rigid_body: platform.get_rigid_body(),
            collider: Collider::rectangle(width, height),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(width, height)),
                    rect: Some(Rect::new(0., 0., PLATFORM_TEXTURE_SIZE, height)),
                    ..default()
                },
                transform: Transform::from_translation(translation),
                texture,
                ..default()
            },
            platform,
            image_scale_mode: ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            linear_velocity: LinearVelocity::default(),
        }
    }
}

#[derive(Bundle)]
struct BoltBundle {
    rigid_body: RigidBody,
    sprite: SpriteBundle,
    bolt: Bolt,
}

impl BoltBundle {
    fn new(texture: Handle<Image>, translation: Vec3) -> Self {
        Self {
            rigid_body: RigidBody::Static,
            sprite: SpriteBundle {
                transform: Transform::from_translation(translation),
                texture,
                ..default()
            },
            bolt: Bolt,
        }
    }
}

type WithPlatformOrBolt = Or<(With<Platform>, With<Bolt>)>;

fn create_initial_platforms(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut highest_platform: ResMut<HighestPlatformInfo>,
) {
    commands.spawn(PlatformBundle::new(
        images.platforms[0].clone(),
        10000.,
        1000.,
        Vec3::new(0., -680., 10.),
        Platform::Static,
    ));

    highest_platform.pos = Vec2::new(0., -180.);
    highest_platform.platform = Platform::Static;
}

fn remove_all_platforms(
    mut commands: Commands,
    query_platforms_and_bolts: Query<Entity, WithPlatformOrBolt>,
) {
    for entity in query_platforms_and_bolts.iter() {
        commands.entity(entity).despawn();
    }
}

fn add_platforms(
    mut commands: Commands,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    images: Res<ImageAssets>,
    height: Res<Height>,
    mut highest_platform: ResMut<HighestPlatformInfo>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window_top = camera
        .viewport_to_world_2d(camera_transform, Vec2::ZERO)
        .unwrap_or(Vec2::ZERO)
        .y;

    if height.0 > highest_platform.pos.y - window_top {
        let mut rng = rand::thread_rng();

        let mut pos = highest_platform.pos;
        pos.y += rng.gen_range(65.0..85.0);
        let mut new_x = pos.x;
        let mut diff_x = 0.;
        while !(75. ..190.).contains(&diff_x) {
            new_x = rng.gen_range(-150.0..150.);
            diff_x = (new_x - pos.x).abs();
        }
        pos.x = new_x;

        let platform = if highest_platform.platform == Platform::Static {
            match rng.gen() {
                0.0..0.4 => Platform::Static,
                0.4..0.7 => Platform::Hanging,
                _ => Platform::Moving {
                    velocity: rng.gen_range(0.5..1.0),
                    range: rng.gen_range(20. ..40.),
                },
            }
        } else {
            Platform::Static
        };

        platform.spawn(&mut commands, &images, pos);

        highest_platform.pos = pos;
        highest_platform.platform = platform;
    }
}

fn remove_platforms(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    platform_or_bolt_query: Query<(Entity, &Sprite, &Transform), WithPlatformOrBolt>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window_size = query_window.single().size();
    let window_bottom = camera
        .viewport_to_world_2d(camera_transform, window_size)
        .unwrap_or(Vec2::ZERO)
        .y;

    for (entity, sprite, transform) in platform_or_bolt_query.iter() {
        if transform.translation.y < window_bottom - sprite.custom_size.unwrap_or(Vec2::ZERO).y / 2.
        {
            commands.entity(entity).despawn();
        }
    }
}

fn scroll_platforms(time: Res<Time>, mut platform_query: Query<(&mut LinearVelocity, &Platform)>) {
    for (mut linear_velocity, platform) in platform_query.iter_mut() {
        if let Platform::Moving { velocity, range } = platform {
            linear_velocity.x = (time.elapsed_seconds() * velocity).sin() * range;
        }
    }
}

fn draw_ropes(
    mut gizmos: Gizmos,
    bodies: Query<(&Position, &Rotation)>,
    rope_query: Query<&DistanceJoint, With<Rope>>,
) {
    for distance_joint in rope_query.iter() {
        if let Ok([(pos1, rot1), (pos2, rot2)]) = bodies.get_many(distance_joint.entities()) {
            gizmos.line_2d(
                pos1.0 + rot1 * distance_joint.local_anchor_1(),
                pos2.0 + rot2 * distance_joint.local_anchor_2(),
                Color::srgb(1.0, 0.6, 0.4),
            );
        }
    }
}
