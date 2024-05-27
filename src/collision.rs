use bevy::prelude::*;

pub struct CollsionPlugin;

impl Plugin for CollsionPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Debug)]
pub struct Collider {
    pub rectangle: Rectangle,
    pub collided_with: Vec<Entity>,
}

impl Collider {
    fn new(rectangle: Rectangle) -> Self {
        rectangle.
        Self {
            rectangle,
            collided_with: vec![],
        }
    }
}
