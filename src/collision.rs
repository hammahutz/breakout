use bevy::{
    ecs::entity,
    math::bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    utils::tracing::event_enabled,
};

use crate::{ball::Ball, movement::Velocity, paddle::Paddle};

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
struct CollisionEvent(Entity, Entity, CollisionSide);

fn update_collsions(
    balls: Query<(Entity, &CircleCollider)>,
    rectangles: Query<(Entity, &RectangleCollider)>,
    mut collision_event: EventWriter<CollisionEvent>,
) {
    for (ball, ball_collider) in balls.iter() {
        for (rectangle, rectangle_collider) in rectangles.iter() {
            if let Some(collision_side) = get_side_of_collision(ball_collider, rectangle_collider) {
                collision_event.send(CollisionEvent(ball, rectangle, collision_side));
            }
        }
    }
}

#[derive(Debug)]
enum CollisionSide {
    Right,
    Bottom,
    Left,
    Top,
}

fn get_side_of_collision(
    ball: &CircleCollider,
    rectangle: &RectangleCollider,
) -> Option<CollisionSide> {
    let diff = ball.volume.center() - rectangle.volume.closest_point(ball.volume.center());
    // println!("{}", diff);

    if !&rectangle.volume.intersects(&ball.volume) {
        return None;
    }

    let side = if diff.x.abs() > diff.y.abs() {
        if diff.x > 0.0 {
            CollisionSide::Left
        } else {
            CollisionSide::Right
        }
    } else if diff.y > 0.0 {
        CollisionSide::Top
    } else {
        CollisionSide::Bottom
    };

    Some(side)
}

fn recive_collsison(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
    mut balls: Query<&mut Velocity, With<Ball>>,
) {
    for event in collision_event.read() {
        if let Ok(mut ball) = balls.get_mut(event.0) {
            match event.2 {
                CollisionSide::Right => ball.value.x = ball.value.x.abs() * -1.0,
                CollisionSide::Bottom => ball.value.y = ball.value.y.abs() * -1.0,
                CollisionSide::Left => ball.value.x = ball.value.x.abs(),
                CollisionSide::Top => ball.value.y = ball.value.y.abs(),
            };
        }
    }
}
