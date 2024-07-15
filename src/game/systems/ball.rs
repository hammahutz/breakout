use crate::game::prelude::*;
use bevy::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ball);
    }
}

fn spawn_ball(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: scene_assets.ball.image.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Ball,
        Velocity::new(Vec2::new(100.0, 100.0)),
        CircleCollider::new(Circle::new(8.0)),
    ));
}
