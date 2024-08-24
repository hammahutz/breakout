use crate::data::{
    bundles::{BlockBundle, WallBundle, WallSide},
    components::{Block, HealthComponent, RectangleCollider},
    resources::{GameSettings, LevelResource, SceneAssets},
};

use bevy::prelude::*;

use super::load_level;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_wall, spawn_blocks.after(load_level)));
    }
}

fn spawn_blocks(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    level: Res<LevelResource>,
    window: Query<&Window>,
) {
    let texture = scene_assets.paddle.image.clone();
    let block_dimension = Vec2::new(100.0, 40.0);

    let window = window.single();
    let level_width = window.resolution.width();
    let level_height = window.resolution.height();

    let grid_start_position = Vec2::new(
        -level_width / 2.0 + block_dimension.x,
        level_height / 2.0 - block_dimension.y,
    );
    let row_column_gap: f32 = 10.0;

    for (y, row) in level.blocks.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            //0 is empty
            if *block == 0 {
                continue;
            }

            let position = Vec2::new(
                grid_start_position.x + (block_dimension.x + row_column_gap) * y as f32,
                grid_start_position.y - (block_dimension.y + row_column_gap) * x as f32,
            );

            commands.spawn(spawn_block(position, block_dimension, texture.clone()));
        }
    }
}

fn spawn_block(position: Vec2, dimension: Vec2, texture: Handle<Image>) -> BlockBundle {
    let sprite = SpriteBundle {
        texture,
        transform: Transform::from_xyz(position.x, position.y, 0.0),
        sprite: Sprite {
            custom_size: Some(dimension),
            ..default()
        },
        ..default()
    };

    let block = Block {
        position,
        dimension,
    };

    let collider = RectangleCollider::new(Rectangle::new(dimension.x, dimension.y));

    //TODO: Fix bug, collsision on spawn? Health 2 instead of 1 because of this. D:
    let health = HealthComponent { value: 3 };

    BlockBundle {
        sprite,
        block,
        collider,
        health,
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
