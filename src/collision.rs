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
