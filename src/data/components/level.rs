use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
pub struct LevelComponent {
    pub blocks: Vec<Vec<i8>>,
}
