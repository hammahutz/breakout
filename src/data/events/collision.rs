use bevy::prelude::*;

use crate::data::util::CollisionSide;

#[derive(Event)]
pub struct CollisionEvent(pub Entity, pub Entity, pub CollisionSide);

#[derive(Event)]
pub struct PaddleCollisionEvent(pub Entity, pub Entity, pub Vec2);
