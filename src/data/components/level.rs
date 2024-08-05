use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GameDimension {
    pub width: u16,
    pub height: u16,
}

#[derive(Component, Serialize, Deserialize)]
pub struct LevelComponent {
    pub game_dimension: GameDimension,
    pub blocks: Vec<Vec<i8>>,
}
