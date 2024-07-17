use crate::game::prelude::*;
use bevy::prelude::*;

pub struct HealthSystemPlugin;

impl Plugin for HealthSystemPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<MousePosition>()
        //     .add_systems(Update, cursor_position);
    }
}
