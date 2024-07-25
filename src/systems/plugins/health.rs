use crate::data::{
    components::{Ball, Block, DamageComponent, HealthComponent},
    events::CollisionEvent,
};
use bevy::prelude::*;

pub struct HealthSystemPlugin;

impl Plugin for HealthSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_health_after_collision);
    }
}

fn update_health_after_collision(
    mut collision_event: EventReader<CollisionEvent>,
    balls: Query<&DamageComponent, With<Ball>>,
    mut blocks: Query<&mut HealthComponent, With<Block>>,
    mut command: Commands,
) {
    for event in collision_event.read() {
        println!("{:?}", event);
        if let Ok(damage) = balls.get(event.0) {
            if let Ok(mut health) = blocks.get_mut(event.1) {
                if health.value > damage.value {
                    health.value -= damage.value;
                } else {
                    command.entity(event.1).despawn();
                }
            }
        }
    }
}
