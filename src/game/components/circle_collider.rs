use bevy::math::bounding::BoundingCircle;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub shape: Circle,
    pub volume: BoundingCircle,
}

impl CircleCollider {
    pub fn new(circle: Circle) -> CircleCollider {
        CircleCollider {
            shape: circle,
            volume: BoundingCircle::new(Vec2::ZERO, 0.0),
        }
    }
}
