use bevy::prelude::*;

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldCoordinates>()
    }
}

#[derive(Resource, Default)]
struct WorldCoordinates {
    global: Vec3,
    local: Vec2,
}
