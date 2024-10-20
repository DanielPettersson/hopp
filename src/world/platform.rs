use crate::world::WorldEntity;
use crate::{GameState, ImageAssets};
use avian2d::collision::Collider;
use avian2d::dynamics::solver::xpbd::XpbdConstraint;
use avian2d::position::{Position, Rotation};
use avian2d::prelude::{DistanceJoint, Joint, LinearVelocity, RigidBody};
use bevy::app::{App, Update};
use bevy::color::Color;
use bevy::math::{Rect, Vec2, Vec3};
use bevy::prelude::{
    default, in_state, Bundle, Commands, Component, FixedUpdate, Gizmos, Handle, Image,
    ImageScaleMode, IntoSystemConfigs, Plugin, Query, Res, Sprite, SpriteBundle, Time, Transform,
    With,
};

static PLATFORM_TEXTURE_SIZE: f32 = 46.;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_ropes).add_systems(
            FixedUpdate,
            scroll_platforms.run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Bundle)]
struct BoltBundle {
    rigid_body: RigidBody,
    sprite: SpriteBundle,
    bolt: Bolt,
    world_entity: WorldEntity,
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
            world_entity: WorldEntity,
        }
    }
}

#[derive(Component, Clone, Default, PartialEq)]
pub enum Platform {
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

    pub fn spawn(&self, commands: &mut Commands, images: &ImageAssets, pos: Vec2) {
        let platform = commands
            .spawn((PlatformBundle::new(
                images.platforms[self.get_image_index()].clone(),
                92.,
                20.,
                pos.extend(0.),
                self.clone(),
            ),))
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

#[derive(Bundle)]
pub struct PlatformBundle {
    rigid_body: RigidBody,
    collider: Collider,
    sprite: SpriteBundle,
    platform: Platform,
    image_scale_mode: ImageScaleMode,
    linear_velocity: LinearVelocity,
    world_entity: WorldEntity,
}

impl PlatformBundle {
    pub fn new(
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
            world_entity: WorldEntity,
        }
    }
}

#[derive(Component)]
struct Bolt;

#[derive(Component)]
struct Rope;

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
