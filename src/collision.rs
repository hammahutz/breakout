use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::paddle::Paddle;

pub struct CollsionPlugin;

impl Plugin for CollsionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                recive_collsison,
                update_collsions,
                update_rectanle,
                update_circle,
            ),
        )
        .add_event::<CollisionEvent>();
    }
}

#[derive(Component, Debug)]
pub struct RectangleCollider {
    pub shape: Rectangle,
    pub volume: Aabb2d,
}

impl RectangleCollider {
    pub fn new(rectangle: Rectangle) -> RectangleCollider {
        RectangleCollider {
            shape: rectangle,
            volume: Aabb2d::new(Vec2::ZERO, Vec2::ZERO),
        }
    }
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub shape: Circle,
    pub volume: BoundingCircle,
}

impl CircleCollider {
    pub fn new(circle: Circle) -> CircleCollider {
        CircleCollider {
            shape: circle,
            volume: BoundingCircle::new(Vec2::ZERO, 0.0),
        }
    }
}

fn update_circle(mut query: Query<(&mut CircleCollider, &Transform)>) {
    for (mut collider, transform) in query.iter_mut() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;

        collider.volume = collider.shape.bounding_circle(translation, rotation);
    }
}
fn update_rectanle(mut query: Query<(&mut RectangleCollider, &Transform)>) {
    for (mut collider, transform) in query.iter_mut() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;

        collider.volume = collider.shape.aabb_2d(translation, rotation);
    }
}

#[derive(Event)]
struct CollisionEvent(Entity, Entity);

fn update_collsions(
    balls: Query<(Entity, &CircleCollider)>,
    rectangles: Query<(Entity, &RectangleCollider)>,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    for (ball, circle_collider) in balls.iter() {
        for (rectangle, rectangle_collider) in rectangles.iter() {
            let collision = &circle_collider
                .volume
                .intersects(&rectangle_collider.volume);

            if !collision {
                continue;
            }

            collision_event.send(CollisionEvent(ball, rectangle));
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
            println!("Paddle: {:?}", paddle);
        } else {
            commands.entity(event.0).despawn();
        }
    }
}
