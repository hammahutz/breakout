use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct AccelerationComponent {
    pub value: Vec2,
}

impl AccelerationComponent {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}
