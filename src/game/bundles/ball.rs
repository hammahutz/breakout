use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct BallBundle {
    pub sprite: SpriteBundle,
    pub ball: Ball,
    pub velocity: VelocityComponent,
    pub circle_collider: CircleCollider,
    pub damage: DamageComponent,
}
