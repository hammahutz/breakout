mod camera;

use bevy::prelude::*;

use camera::CameraPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugins)
        .run();
}
