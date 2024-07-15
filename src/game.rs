mod components;
mod entities;
mod events;
mod resources;
mod systems;
mod util;

use bevy::prelude::*;
use systems::prelude::*;

pub mod prelude {
    pub use super::components::prelude::*;
    pub use super::entities::prelude::*;
    pub use super::events::prelude::*;
    pub use super::resources::prelude::*;
    pub use super::util::*;
}

pub struct StartupPlugin;
impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
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
            .add_plugins(BlockPlugin);
    }
}
