use crate::game::prelude::*;
use bevy::{app::AppExit, math::bounding::BoundingVolume, prelude::*};

pub const IS_DEBUG: bool = true;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if !IS_DEBUG {
            return;
        }
        app.add_systems(Update, draw_collider)
            .add_systems(Update, exit_game)
            // .add_systems(Update, ( check_coordinates)
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
    //TODO: Fixa så att även vinklen fungerar
    for (rectangle, circle) in &query {
        if let Some(rectangle) = rectangle {
            gizmos.primitive_2d(rectangle.shape, rectangle.volume.center(), 0.0, Color::RED)
        }
        if let Some(circle) = circle {
            gizmos.primitive_2d(circle.shape, circle.volume.center(), 0.0, Color::GREEN)
        }
    }
}

// fn check_coordinates(mouse_position: Res<MousePosition>) {
//     println!("{:?}", mouse_position)
// }
