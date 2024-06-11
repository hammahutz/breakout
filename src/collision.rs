use bevy::{
    math::bounding::{Aabb2d, Bounded2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

pub struct CollsionPlugin;

impl Plugin for CollsionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_collsions, update_volume));
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

#[derive(Component, Debug)]
pub struct RectangleCollider {
    pub shape: Rectangle,
}

impl RectangleCollider {
    pub fn new(collider: Rectangle) -> Self {
        Self { shape: collider }
    }
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub shape: Circle,
}

impl CircleCollider {
    pub fn new(collider: Circle) -> Self {
        Self { shape: collider }
    }
}

// TODO: Create a way to detect colliision from a circle
// fn update_aabb_circle(mut query: Query<(&Transform, &CircleCollider, &mut CollisionDetection)>) {
//     for (transform, circle, mut collision_detection) in query.iter_mut() {
//         collision_detection.aabb = circle.primitived.aabb_2d(
//             transform.translation.xy(),
//             transform.rotation.to_euler(EulerRot::XYZ).2,
//         );
//     }
// }
//

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

fn update_collsion(query: Query<(Entity, &RectangleCollider, &Transform)>) {
    for (entity_1, collider_1, transform_1) in query.iter() {
        for (entity_2, collider_2, transform_2) in query.iter() {
            if entity_1 == entity_2 {
                break;
            }

            let aabb1 = collider_1
                .shape
                .aabb_2d(transform_1.translation.xy(), transform_1.rotation.y);

            let aabb2 = collider_2
                .shape
                .aabb_2d(transform_2.translation.xy(), transform_2.rotation.y);

            let collision = aabb1.intersects(&aabb2);

            if collision {
                println!("Collsision");
            }
        }
    }
}

fn update_collsions(query: Query<(Entity, &Volume)>) {
    for (entity_1, volume_1) in query.iter() {
        for (entity_2, volume_2) in query.iter() {
            if entity_1 == entity_2 {
                break;
            }

            let collision = match volume_1 {
                Volume::Aabb2d(a) => match volume_2 {
                    Volume::Aabb2d(b) => a.intersects(b),
                    Volume::BoundingCircle(b) => a.intersects(b),
                },
                Volume::BoundingCircle(a) => match volume_2 {
                    Volume::Aabb2d(b) => a.intersects(b),
                    Volume::BoundingCircle(b) => a.intersects(b),
                },
            };

            if collision {
                println!("Collision");
            }
        }
    }
}
