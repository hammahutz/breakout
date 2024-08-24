use bevy::{
    math::bounding::{Bounded2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::data::{
    components::{CircleCollider, PaddleComponent, RectangleCollider},
    events::{CollisionEvent, PaddleCollisionEvent},
    util::CollisionSide,
};

use super::GameLoop;

pub struct CollsionPlugin;
impl Plugin for CollsionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_block_collsions,
                update_rectanle,
                update_circle,
                update_paddle_collision,
            )
                .in_set(GameLoop::CollisionDetection),
        )
        .add_event::<CollisionEvent>()
        .add_event::<PaddleCollisionEvent>();
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

//FIXME Double collsiion
fn update_block_collsions(
    balls: Query<(Entity, &CircleCollider)>,
    rectangles: Query<(Entity, &RectangleCollider), Without<PaddleComponent>>,
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

fn get_side_of_collision(
    ball: &CircleCollider,
    rectangle: &RectangleCollider,
) -> Option<CollisionSide> {
    let diff = ball.volume.center() - rectangle.volume.closest_point(ball.volume.center());

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

fn update_paddle_collision(
    balls: Query<(Entity, &CircleCollider)>,
    rectangles: Query<(Entity, &RectangleCollider), With<PaddleComponent>>,
    mut collision_event: EventWriter<PaddleCollisionEvent>,
) {
    let (ball_entity, ball_collider) = balls.single();
    let (paddle_entity, paddle_collider) = rectangles.single();

    if !paddle_collider.volume.intersects(&ball_collider.volume) {
        return;
    }

    let hit_direction =
        (ball_collider.volume.center() - paddle_collider.volume.center()).normalize();

    collision_event.send(PaddleCollisionEvent(
        ball_entity,
        paddle_entity,
        hit_direction,
    ));
}
