use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct CollisionEvent(pub Entity, pub Entity, pub CollisionSide);
