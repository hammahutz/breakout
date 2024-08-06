use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct LevelResource {
    pub blocks: Vec<Vec<i8>>,
}
