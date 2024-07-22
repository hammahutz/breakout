use crate::data::components::GameCamera;
use bevy::prelude::*;

pub struct CameraPlugins;

impl Plugin for CameraPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        GameCamera,
    ));
}
