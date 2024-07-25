use bevy::{app::AppExit, math::bounding::BoundingVolume, prelude::*};

use crate::data::components::{
    Ball, CircleCollider, PaddleComponent, RectangleCollider, VelocityComponent,
};

pub const IS_DEBUG: bool = true;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if !IS_DEBUG {
            return;
        }
        app.add_systems(Update, (exit_game, draw_collider, draw_vector))
            .init_gizmo_group::<DebugGizmos>();
    }
}

pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DebugGizmos;

fn draw_collider(
    query: Query<AnyOf<(&RectangleCollider, &CircleCollider)>>,
    mut gizmos: Gizmos<DebugGizmos>,
) {
    for (rectangle, circle) in &query {
        if let Some(rectangle) = rectangle {
            gizmos.primitive_2d(rectangle.shape, rectangle.volume.center(), 0.0, Color::RED)
        }
        if let Some(circle) = circle {
            gizmos.primitive_2d(circle.shape, circle.volume.center(), 0.0, Color::GREEN)
        }
    }
}

fn draw_vector(
    mut gizmos: Gizmos<DebugGizmos>,
    ball_query: Query<(&Transform, &VelocityComponent), With<Ball>>,
    paddle_query: Query<&Transform, With<PaddleComponent>>,
) {
    let (ball, vel) = ball_query.single();
    let paddle = paddle_query.single();

    gizmos.line_2d(
        Vec2::new(ball.translation.x, ball.translation.y),
        Vec2::new(paddle.translation.x, paddle.translation.y),
        Color::PURPLE,
    );

    gizmos.line_2d(
        Vec2::new(ball.translation.x, ball.translation.y),
        Vec2::new(
            ball.translation.x + vel.value.x,
            ball.translation.y + vel.value.y,
        ),
        Color::BLUE,
    );
}
