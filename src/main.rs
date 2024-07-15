mod game;

use bevy::prelude::*;
use game::StartupPlugin;

fn main() {
    App::new().add_plugins(StartupPlugin).run();
}
