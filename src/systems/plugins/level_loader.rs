use std::{fs::File, io::BufReader};

use bevy::prelude::*;

use crate::data::resources::LevelResource;

pub struct LevelLoaderPlugin;
impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelResource>()
            .add_systems(Startup, load_level);
    }
}

fn load_level(mut level: ResMut<LevelResource>) {
    let file = match File::open("assets/levels/level1.ron") {
        Ok(f) => f,
        Err(e) => {
            eprint!("Can't open the fille: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let loaded_level: LevelResource = match ron::de::from_reader(reader) {
        Ok(l) => l,
        Err(e) => {
            eprint!("Can't read buffer: {}", e);
            return;
        }
    };

    level.blocks = loaded_level.blocks;
    level.game_dimension = loaded_level.game_dimension;

    println!("{:?}", level);
}
