// use bevy::prelude::*;
//
// trait Vec2Ext {
//     fn from_rect(rectangle: Rectangle) -> Vec2;
// }
//
// impl Vec2Ext for Vec2 {
//     fn from_rect(rectangle: Rectangle) {
//         Vec2::new(rectangle.)
//     }
// }
//
#[derive(Debug)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct Settings {
    wall_thicknse: u32,
}
