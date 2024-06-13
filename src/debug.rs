use bevy::{app::AppExit, math::bounding::BoundingVolume, prelude::*};

use crate::collision::Volume;

pub const IS_DEBUG: bool = true;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if !IS_DEBUG {
            return;
        }
        app.add_systems(Update, (draw_collider, exit_game))
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

fn draw_collider(query: Query<&Volume>, mut gizmos: Gizmos<DebugGizmos>) {
    for volume in query.iter() {
        match volume {
            Volume::Aabb2d(a) => {
                gizmos.rect_2d(a.center(), 0.0, a.half_size().xy() * 2.0, Color::RED);
            }
            Volume::BoundingCircle(a) => {
                gizmos.circle_2d(a.center(), a.radius(), Color::GREEN);
            }
        }
    }
}
