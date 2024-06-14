use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::{ball::Ball, paddle::Paddle};

pub struct CollsionPlugin;

impl Plugin for CollsionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (recive_collsison, update_collsions, update_volume))
            .add_event::<CollisionEvent>();
    }
}

#[derive(Component, Debug)]
pub enum Collider {
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Component, Debug)]
pub enum Volume {
    Aabb2d(Aabb2d),
    BoundingCircle(BoundingCircle),
}

fn update_volume(mut commands: Commands, query: Query<(Entity, &Collider, &Transform)>) {
    for (entity, collider, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;

        match collider {
            Collider::Rectangle(r) => {
                let aabb2d = r.aabb_2d(translation, rotation);
                commands.entity(entity).insert(Volume::Aabb2d(aabb2d));
            }
            Collider::Circle(c) => {
                let bouding_circle = c.bounding_circle(translation, rotation);
                commands
                    .entity(entity)
                    .insert(Volume::BoundingCircle(bouding_circle));
            }
        }
    }
}

#[derive(Event)]
struct CollisionEvent(Entity, Entity);

fn update_collsions(
    balls: Query<(Entity, &Volume), With<Ball>>,
    colliders: Query<(Entity, &Volume), Without<Ball>>,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    for (ball, ball_volume) in balls.iter() {
        for (collider, collider_volyme) in colliders.iter() {
            let collision = match ball_volume {
                Volume::Aabb2d(a) => match collider_volyme {
                    Volume::Aabb2d(b) => a.intersects(b),
                    Volume::BoundingCircle(b) => a.intersects(b),
                },
                Volume::BoundingCircle(a) => match collider_volyme {
                    Volume::Aabb2d(b) => a.intersects(b),
                    Volume::BoundingCircle(b) => a.intersects(b),
                },
            };

            if collision {
                collision_event.send(CollisionEvent(ball, collider));
            }
        }
    }
}

fn recive_collsison(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
    query: Query<&Paddle>,
) {
    for event in collision_event.read() {
        println!("{:?} collided with {:?}", event.0, event.1);

        if let Ok(paddle) = query.get(event.1) {
            println!("Paddle");
        } else {
            commands.entity(event.0).despawn();
        }
    }
}
