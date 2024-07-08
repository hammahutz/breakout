mod asset_loader;
mod ball;
mod camera;
mod collision;
mod debug;
mod input;
mod movement;
mod paddle;
mod util;
mod wall;

use asset_loader::AssetLoaderPlugin;
use ball::BallPlugin;
use bevy::prelude::*;

use camera::CameraPlugins;
use collision::CollsionPlugin;
use debug::DebugPlugin;
use input::GameInput;
use movement::MovementPlugin;
use paddle::PaddlePlugin;
use wall::WallPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugins)
        .add_plugins(DebugPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(GameInput)
        .add_plugins(CollsionPlugin)
        .add_plugins(WallPlugin)
        .run();
}
