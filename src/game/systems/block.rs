use crate::game::prelude::*;
use bevy::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_wall);
    }
}

fn spawn_wall(mut commands: Commands, scene_assets: Res<SceneAssets>, window: Query<&Window>) {
    let window = window.single();
    let wall_thicknes = 10.0;
    let screen_width = window.resolution.width();
    let screen_height = window.resolution.height();

    let left_wall = Block::new(
        Vec2::new(screen_width / 2.0, 0.0),
        Vec2::new(wall_thicknes, screen_height),
    );
    let right_wall = Block::new(
        Vec2::new(-screen_width / 2.0, 0.0),
        Vec2::new(wall_thicknes, screen_height),
    );
    let top_wall = Block::new(
        Vec2::new(0.0, screen_height / 2.0),
        Vec2::new(screen_width, 10.0),
    );

    commands.spawn((
        SpriteBundle {
            texture: scene_assets.paddle.image.clone(),
            transform: Transform::from_xyz(left_wall.position.x, left_wall.position.y, 0.0),
            sprite: Sprite {
                custom_size: Some(left_wall.dimension),
                ..default()
            },
            ..default()
        },
        left_wall,
        RectangleCollider::new(Rectangle::from_size(left_wall.dimension)),
    ));

    commands.spawn((
        SpriteBundle {
            texture: scene_assets.paddle.image.clone(),
            transform: Transform::from_xyz(right_wall.position.x, right_wall.position.y, 0.0),
            sprite: Sprite {
                custom_size: Some(right_wall.dimension),
                ..default()
            },
            ..default()
        },
        right_wall,
        RectangleCollider::new(Rectangle::from_size(right_wall.dimension)),
    ));

    commands.spawn((
        SpriteBundle {
            texture: scene_assets.paddle.image.clone(),
            transform: Transform::from_xyz(top_wall.position.x, top_wall.position.y, 0.0),
            sprite: Sprite {
                custom_size: Some(top_wall.dimension),
                ..default()
            },
            ..default()
        },
        top_wall,
        RectangleCollider::new(Rectangle::from_size(top_wall.dimension)),
    ));
}