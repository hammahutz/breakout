use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup, spawn_paddle);
    }
}

pub struct Paddle {
    pub model: SceneBundle,
}

fn spawn_paddle(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let spawn = commands.spawn((
            SceneBundle {
                scene: scene_assets.paddle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, -50.0),
                ..default()
            },
    ));
}
 
