use crate::ImageAssets;
use avian2d::collision::Collider;
use avian2d::prelude::{AngularVelocity, LinearVelocity, RigidBody};
use bevy::prelude::{default, Bundle, Component, Res, SpriteBundle, Transform, Vec3};
use rand::Rng;
use crate::world::WorldEntity;

#[derive(Component)]
struct Box;

#[derive(Bundle)]
struct BoxBundle {
    rigid_body: RigidBody,
    collider: Collider,
    sprite: SpriteBundle,
    r#box: Box,
    linear_velocity: LinearVelocity,
    angular_velocity: AngularVelocity,
    world_entity: WorldEntity,
}

impl BoxBundle {
    fn new(
        translation: Vec3,
        linear_velocity: LinearVelocity,
        angular_velocity: AngularVelocity,
        images: Res<ImageAssets>,
    ) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(20.0, 20.0),
            sprite: SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: images.boxes[rand::thread_rng().gen_range(0..images.boxes.len())].clone(),
                ..default()
            },
            r#box: Box,
            linear_velocity,
            angular_velocity,
            world_entity: WorldEntity,
        }
    }
}
