use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct GameSettings {
    pub wall_thickness: f32,
}

impl GameSettings {
    pub fn new(wall_thickness: f32) -> Self {
        Self { wall_thickness }
    }
}
