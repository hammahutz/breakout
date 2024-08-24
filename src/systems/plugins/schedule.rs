use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameLoop {
    UserInput,
    UpdateEntities,
    CollisionDetection,
    DamageControll,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameLoop::UserInput,
                GameLoop::UpdateEntities,
                GameLoop::CollisionDetection,
                GameLoop::DamageControll,
            )
                .chain(),
        );
    }
}
