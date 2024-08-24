use bevy::prelude::*;

use crate::data::components::{
    ball::Ball, circle_collider::CircleCollider, damage::DamageComponent,
    velocity::VelocityComponent,
};

#[derive(Bundle)]
pub struct BallBundle {
    pub sprite: SpriteBundle,
    pub ball: Ball,
    pub velocity: VelocityComponent,
    pub circle_collider: CircleCollider,
    pub damage: DamageComponent,
}
