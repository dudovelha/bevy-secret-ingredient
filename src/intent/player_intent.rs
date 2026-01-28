use bevy::prelude::Vec3;

pub struct PlayerIntent {
    pub move_x: i8,
    pub move_y: i8,
    pub jump: bool,
}