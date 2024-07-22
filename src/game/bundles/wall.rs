use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct WallBundle {
    pub sprite: SpriteBundle,
    pub block: Block,
    pub collider: RectangleCollider,
}

pub enum WallSide {
    Right,
    Top,
    Left,
    Bottom,
}

impl WallBundle {
    pub fn new(
        side: WallSide,
        texture: Handle<Image>,
        wall_thickness: f32,
        level_width: f32,
        level_height: f32,
    ) -> Self {
        let block = WallBundle::get_block(side, wall_thickness, level_width, level_height);

        let sprite = SpriteBundle {
            texture,
            transform: Transform::from_xyz(block.position.x, block.position.y, 0.0),
            sprite: Sprite {
                custom_size: Some(block.dimension),
                ..default()
            },
            ..default()
        };

        let collider = RectangleCollider::new(Rectangle::from_size(block.dimension));

        WallBundle {
            sprite,
            block,
            collider,
        }
    }

    fn get_block(
        side: WallSide,
        wall_thickness: f32,
        level_width: f32,
        level_height: f32,
    ) -> Block {
        // let level_width = window.resolution.width();
        // let level_height = window.resolution.height();

        let position_x: f32;
        let position_y: f32;
        let dimension_x: f32;
        let dimension_y: f32;

        match side {
            WallSide::Right => {
                position_x = -level_width / 2.0;
                position_y = 0.0;
                dimension_x = wall_thickness;
                dimension_y = level_height;
            }
            WallSide::Top => {
                position_x = 0.0;
                position_y = level_height / 2.0;
                dimension_x = level_width;
                dimension_y = wall_thickness;
            }
            WallSide::Left => {
                position_x = level_width / 2.0;
                position_y = 0.0;
                dimension_x = wall_thickness;
                dimension_y = level_height
            }
            WallSide::Bottom => {
                position_x = 0.0;
                position_y = -level_height / 2.0;
                dimension_x = level_width;
                dimension_y = wall_thickness;
            }
        };

        Block::new(
            Vec2::new(position_x, position_y),
            Vec2::new(dimension_x, dimension_y),
        )
    }
}

#[derive(Bundle)]
pub struct BlockBundle {
    pub sprite: SpriteBundle,
    pub block: Block,
    pub collider: RectangleCollider,
    pub health: HealthComponent,
}
