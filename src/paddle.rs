use bevy::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::collision::Collider;
use crate::input::MousePosition;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_paddle)
            .add_systems(Update, move_paddle);
    }
}

#[derive(Component, Debug)]
pub struct Paddle;

fn spawn_paddle(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    assets: Res<Assets<Image>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: scene_assets.paddle.image.clone(),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 32.0)),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider::new(scene_assets.paddle.clone().size(assets)),
    ));
}

fn move_paddle(
    mouse_position_resource: Res<MousePosition>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
) {
    if let Ok(mut transform) = paddle_query.get_single_mut() {
        transform.translation.x = mouse_position_resource.0.x;
    }
}
