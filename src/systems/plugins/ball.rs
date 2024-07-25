use bevy::{ecs::event, prelude::*};

use crate::data::{
    bundles::BallBundle,
    components::{Ball, CircleCollider, DamageComponent, PaddleComponent, VelocityComponent},
    events::{CollisionEvent, PaddleCollisionEvent},
    resources::SceneAssets,
    util::CollisionSide,
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ball)
            .add_systems(Update, (recive_collsison, recive_paddle_collsison));
    }
}

fn spawn_ball(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(BallBundle {
        sprite: SpriteBundle {
            texture: scene_assets.ball.image.clone(),
            transform: Transform::from_xyz(100.0, 100.0, 0.0),
            ..default()
        },
        ball: Ball,
        velocity: VelocityComponent::new(Vec2::new(100.0, 100.0)),
        circle_collider: CircleCollider::new(Circle::new(8.0)),
        damage: DamageComponent { value: 1 },
    });
}

fn recive_collsison(
    mut collision_event: EventReader<CollisionEvent>,
    mut balls: Query<&mut VelocityComponent, With<Ball>>,
    paddle: Query<&PaddleComponent>,
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

fn recive_paddle_collsison(
    mut collision_event: EventReader<PaddleCollisionEvent>,
    mut balls: Query<&mut VelocityComponent, With<Ball>>,
    paddle: Query<&PaddleComponent>,
) {
    for event in collision_event.read() {
        if event.2.is_nan() {
            return;
        }

        if let Ok(mut ball) = balls.get_mut(event.0) {
            println!("HIT! {}", event.2);
            ball.value = event.2 * ball.value.length();
        }
    }
}
