use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct VelocityComponent {
    pub value: Vec2,
}
impl VelocityComponent {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}
