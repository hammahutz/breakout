use bevy::{app::AppExit, prelude::*};

use crate::collision::Collider;

pub const IS_DEBUG: bool = true;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if !IS_DEBUG {
            return;
        }
        app.add_systems(Update, (exit_game, draw_collider_shapes))
            .init_gizmo_group::<DebugGizmos>();
    }
}

fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DebugGizmos;

fn draw_collider_shapes(mut query: Query<&Collider>, mut gizmos: Gizmos<DebugGizmos>) {
    gizmos.line_2d(Vec2::new(-100.0, 0.0), Vec2::new(100.0, 0.0), Color::GREEN);
    gizmos.rect_2d(, rotation, size, color)
}
