use bevy::prelude::*;

use super::{asset_loader::SceneAssets, collision::RectangleCollider, input::MousePosition};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_paddle)
            .add_systems(Update, move_paddle);
    }
}

#[derive(Component, Debug)]
pub struct Paddle;

fn spawn_paddle(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let rectangle = Rectangle::new(128.0, 32.0);

    commands.spawn((
        SpriteBundle {
            texture: scene_assets.paddle.image.clone(),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            sprite: Sprite {
                custom_size: Some(rectangle.size()),
                ..default()
            },
            ..default()
        },
        Paddle,
        RectangleCollider::new(rectangle),
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
