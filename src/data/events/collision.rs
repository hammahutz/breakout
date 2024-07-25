use bevy::prelude::*;

use crate::data::util::CollisionSide;

#[derive(Event, Debug)]
pub struct CollisionEvent(pub Entity, pub Entity, pub CollisionSide);

#[derive(Event, Debug)]
pub struct PaddleCollisionEvent(pub Entity, pub Entity, pub Vec2);
