use bevy::{
    app::AppExit,
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
    math::bounding::BoundingVolume,
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};

use crate::data::components::{
    Ball, CircleCollider, HealthComponent, PaddleComponent, RectangleCollider, VelocityComponent,
};

use super::GameLoop;

pub const IS_DEBUG: bool = true;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if !IS_DEBUG {
            return;
        }
        app.add_systems(PostStartup, (spawn_health_text, spawn_specs_text))
            .add_systems(
                Update,
                (draw_collider, draw_vector, update_health_display)
                    .in_set(GameLoop::UpdateEntities),
            )
            .add_systems(Update, (exit_game, update_specs_text))
            .init_gizmo_group::<DebugGizmos>()
            .add_plugins((
                LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin,
                EntityCountDiagnosticsPlugin,
            ));
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

fn spawn_health_text(mut commands: Commands, query: Query<(Entity, &Transform, &HealthComponent)>) {
    println!("Före Loop");
    for (entity, transform, health) in &query {
        println!("I Loop");
        commands.entity(entity).insert(Text2dBundle {
            text: Text::from_section(
                // health_comp.value.to_string(),
                health.value.to_string(),
                TextStyle {
                    font_size: 20.0,
                    color: Color::CYAN,
                    ..default()
                },
            ),
            transform: *transform,
            ..default()
        });
    }
    println!("Efter loop");
}

fn update_health_display(mut query: Query<(&mut Text, &HealthComponent)>) {
    for (mut text, health) in query.iter_mut() {
        let value = health.value.to_string();
        let style = text.sections[0].style.clone();

        text.sections = vec![TextSection { value, style }];
    }
}

#[derive(Component)]
pub struct SpecText;

fn spawn_specs_text(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.single();
    let x = -window.resolution.width() / 2.0 + 50.0;
    let y = window.resolution.height() / 2.0 - 50.0;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                String::from("FPS:"),
                TextStyle {
                    font_size: 20.0,
                    color: Color::ORANGE,
                    ..default()
                },
            ),
            transform: Transform::from_xyz(x, y, 0.0),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
        SpecText,
    ));
}

fn update_specs_text(
    mut query: Query<&mut Text, With<SpecText>>,
    diagnostic: Res<DiagnosticsStore>,
) {
    let mut text = query.single_mut();
    let style = text.sections[0].style.clone();
    let mut values: Vec<String> = Vec::new();

    if let Some(fps_diagnostic) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_value) = fps_diagnostic.smoothed() {
            values.push(format!("FPS: {:.0}", fps_value));
        }
    }

    if let Some(frame_time_diagnostic) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_value) = frame_time_diagnostic.smoothed() {
            values.push(format!("Frame time: {:.2}", frame_time_value));
        }
    }

    if let Some(frame_count_diagnostic) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
        if let Some(frame_count_value) = frame_count_diagnostic.smoothed() {
            values.push(format!("Frame count: {:.0}", frame_count_value));
        }
    }

    if let Some(entity_count_diagnostic) =
        diagnostic.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
    {
        if let Some(entity_count_value) = entity_count_diagnostic.smoothed() {
            values.push(format!("Entitys count: {:.0}", entity_count_value));
        }
    }

    if !values.is_empty() {
        text.sections = values
            .into_iter()
            .map(|value| TextSection {
                value: format!("{}\n", value),
                style: style.clone(),
            })
            .collect();
    }
}
