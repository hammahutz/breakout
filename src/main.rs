pub mod data;
pub mod systems;

use bevy::prelude::*;
use data::resources::GameSettings;
use systems::plugins::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .insert_resource(GameSettings::new(10.0))
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(HealthSystemPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugins)
        .add_plugins(DebugPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(GameInput)
        .add_plugins(CollsionPlugin)
        .add_plugins(BlockPlugin)
        .run();
}
