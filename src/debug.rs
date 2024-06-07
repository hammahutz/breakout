use std::{borrow::Borrow, ops::Deref};

use bevy::{app::AppExit, prelude::*};

use crate::collision::{CircleCollider, RectangleCollider};

pub const IS_DEBUG: bool = true;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if !IS_DEBUG {
            return;
        }
        app.add_systems(Update, (draw_collider_rectangle, draw_collider_circle, exit_game))
            .init_gizmo_group::<DebugGizmos>();
    }
}

pub(crate) fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DebugGizmos;

fn draw_collider_rectangle(
    mut query: Query<(&RectangleCollider, &Transform)>,
    mut gizmos: Gizmos<DebugGizmos>,
) {
    for (rectangle, transfrom) in query.iter() {
        gizmos.primitive_2d(
            rectangle.primitive2d.clone(),
            transfrom.translation.xy(),
            0.0,
            Color::RED,
        );
    }
}
fn draw_collider_circle(
    mut query: Query<(&CircleCollider, &Transform)>,
    mut gizmos: Gizmos<DebugGizmos>,
) {
    for (cicle, transfrom) in query.iter() {
        gizmos.primitive_2d(
            cicle.primitive2d.clone(),
            transfrom.translation.xy(),
            0.0,
            Color::BLUE,
        );
    }
}
