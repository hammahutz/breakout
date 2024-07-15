use crate::game::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct GameInput;

impl Plugin for GameInput {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .add_systems(Update, cursor_position);
    }
}

fn cursor_position(
    mut mouse_position: ResMut<MousePosition>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let window = windows.single();
    let (camera, carmera_transform) = cameras.single();

    if let Some(current_mouse_posision) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(carmera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_position.0 = current_mouse_posision;
    };
}
