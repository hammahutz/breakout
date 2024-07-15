use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec2,
}
impl Velocity {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}
