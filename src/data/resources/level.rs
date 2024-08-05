use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::data::components::GameDimension;

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct LevelResource {
    pub game_dimension: GameDimension,
    pub blocks: Vec<Vec<i8>>,
}
