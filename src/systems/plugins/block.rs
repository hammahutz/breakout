use crate::data::{
    bundles::{WallBundle, WallSide},
    resources::{GameSettings, SceneAssets},
};
use bevy::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_wall);
    }
}

fn spawn_wall(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    game_settings: Res<GameSettings>,
    window: Query<&Window>,
) {
    let texture = scene_assets.paddle.image.clone();
    let wall_thickness = game_settings.wall_thickness;

    let window = window.single();
    let level_width = window.resolution.width();
    let level_height = window.resolution.height();

    commands.spawn(WallBundle::new(
        WallSide::Top,
        texture.clone(),
        wall_thickness,
        level_width,
        level_height,
    ));
    commands.spawn(WallBundle::new(
        WallSide::Left,
        texture.clone(),
        wall_thickness,
        level_width,
        level_height,
    ));
    commands.spawn(WallBundle::new(
        WallSide::Right,
        texture.clone(),
        wall_thickness,
        level_width,
        level_height,
    ));
}
