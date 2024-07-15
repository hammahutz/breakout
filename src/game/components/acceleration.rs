use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec2,
}

impl Acceleration {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}
