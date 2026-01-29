use bevy::math::Vec3;
use crate::grid::grid::CELL_SIZE_F;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl GridPosition {
    pub fn from_world(pos: Vec3, origin: Vec3) -> Self {
        let p = pos + origin;
        let inv = 1.0 / CELL_SIZE_F;
        Self {
            x: ((p.x * inv) + 0.5).floor() as i32,
            y: ((p.y * inv) + 0.5).floor() as i32,
            z: ((p.z * inv) + 0.5).floor() as i32,
        }
    }

    pub fn to_world(self, origin: Vec3) -> Vec3 {
        Vec3::new(
            self.x as f32 * CELL_SIZE_F,
            self.y as f32 * CELL_SIZE_F,
            self.z as f32 * CELL_SIZE_F,
        ) - origin
    }
}