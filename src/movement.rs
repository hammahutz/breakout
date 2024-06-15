use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    value: Vec2,
}
impl Velocity {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    value: Vec2,
}
// impl Acceleration {
//     fn new(value: Vec2) -> Self {
//         Self { value }
//     }
// }

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += Vec3::from((velocity.value, 0.0)) * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, aceleration) in query.iter_mut() {
        velocity.value += aceleration.value * time.delta_seconds();
    }
}
