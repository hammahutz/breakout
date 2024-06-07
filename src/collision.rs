use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

pub struct CollsionPlugin;

impl Plugin for CollsionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_collsion, update_aabb_rectangle));
    }
}

#[derive(Component, Debug)]
pub struct RectangleCollider {
    pub primitive2d: Rectangle,
}

impl RectangleCollider {
    pub fn new(collider: Rectangle) -> Self {
        Self {
            primitive2d: collider,
        }
    }
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub primitive2d: Circle,
}

impl CircleCollider {
    pub fn new(collider: Circle) -> Self {
        Self {
            primitive2d: collider,
        }
    }
}

#[derive(Component, Debug)]
pub struct CollisionDetection {
    pub aabb: Aabb2d,
}

impl CollisionDetection{
    pub fn new () -> Self {
       Self {aabb: Aabb2d::new(Vec2::ZERO, Vec2::ZERO)}
    }
}

fn update_aabb_rectangle(mut query: Query<(&Transform, &RectangleCollider, &mut CollisionDetection)>) {
    for (transform, rectangle, mut collision_detection) in query.iter_mut() {
        collision_detection.aabb = rectangle.primitive2d.aabb_2d(
            transform.translation.xy(),
            transform.rotation.to_euler(EulerRot::XYZ).2,
        );
    }
}

//TODO Create a way to detect colliision from a circle
// fn update_aabb_circle(mut query: Query<(&Transform, &CircleCollider, &mut CollisionDetection)>) {
//     for (transform, circle, mut collision_detection) in query.iter_mut() {
//         collision_detection.aabb = circle.primitive2d.aabb_2d(
//             transform.translation.xy(),
//             transform.rotation.to_euler(EulerRot::XYZ).2,
//         );
//     }
// }

fn update_collsion(query: Query<(Entity, &CollisionDetection)>) {

    for (entity_1, collision_1) in query.iter() {
        for (entity_2, collision_2) in query.iter() {

            if entity_1 == entity_2{
                break;
            }

            let test = collision_1.aabb.intersects(&collision_2.aabb);

            if test {
                println!("COLLSISION!! ;:D");
            }
        }
    }
}
