use crate::game::prelude::*;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

fn update_position(mut query: Query<(&mut Transform, &VelocityComponent)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += Vec3::from((velocity.value, 0.0)) * time.delta_seconds();
    }
}

fn update_velocity(
    mut query: Query<(&mut VelocityComponent, &AccelerationComponent)>,
    time: Res<Time>,
) {
    for (mut velocity, aceleration) in query.iter_mut() {
        velocity.value += aceleration.value * time.delta_seconds();
    }
}
