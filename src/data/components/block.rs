use bevy::prelude::*;
#[derive(Component, Debug, Clone, Copy)]
pub struct Block {
    pub position: Vec2,
    pub dimension: Vec2,
}

impl Block {
    pub fn new(position: Vec2, dimension: Vec2) -> Self {
        Self {
            position,
            dimension,
        }
    }
}
