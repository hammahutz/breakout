mod asset_loader;
mod camera;
mod debug;
mod paddle;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;

use camera::CameraPlugins;
use debug::DebugPlugin;
use paddle::PaddlePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugins)
        .add_plugins(DebugPlugin)
        .add_plugins(PaddlePlugin)
        .run();
}
