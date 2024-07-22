use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct RectangleCollider {
    pub shape: Rectangle,
    pub volume: Aabb2d,
}

impl RectangleCollider {
    pub fn new(rectangle: Rectangle) -> RectangleCollider {
        RectangleCollider {
            shape: rectangle,
            volume: Aabb2d::new(Vec2::ZERO, Vec2::ZERO),
        }
    }
}
