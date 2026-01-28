use crate::grid::tile::TileData;
use crate::physics::physics_world::PhysicsWorld;
use bevy::prelude::*;
use std::collections::HashMap;

pub const CELL_SIZE: i32 = 1;
pub const CELL_SIZE_F: f32 = CELL_SIZE as f32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Default, Resource)]
pub struct VoxelGrid {
    pub tiles: HashMap<GridPos, TileData>,
    pub size: i32,
}

impl GridPos {
    pub fn from_world(pos: Vec3) -> Self {
        Self {
            x: (pos.x / CELL_SIZE_F).floor() as i32,
            y: (pos.y / CELL_SIZE_F).floor() as i32,
            z: (pos.z / CELL_SIZE_F).floor() as i32,
        }
    }

    pub fn to_world(self) -> Vec3 {
        Vec3::new(
            (self.x * CELL_SIZE) as f32,
            (self.y * CELL_SIZE) as f32,
            (self.z * CELL_SIZE) as f32,
        )
    }
}

impl VoxelGrid {
    pub fn is_solid(&self, pos: GridPos) -> bool {
        self.tiles.contains_key(&pos)
    }

    pub fn get(&self, pos: GridPos) -> Option<&TileData> {
        self.tiles.get(&pos)
    }

    pub fn get_mut(&mut self, pos: GridPos) -> Option<&mut TileData> {
        self.tiles.get_mut(&pos)
    }

    pub fn set(&mut self, pos: GridPos, tile: TileData) {
        self.tiles.insert(pos, tile);
    }

    pub fn remove(&mut self, pos: GridPos) {
        self.tiles.remove(&pos);
    }
}

impl PhysicsWorld for VoxelGrid {
    fn is_aabb_blocked(&self, min: Vec3, max: Vec3) -> bool {
        let min_cell = GridPos::from_world(min);
        let max_cell = GridPos::from_world(max);

        for x in min_cell.x..=max_cell.x {
            for y in min_cell.y..=max_cell.y {
                for z in min_cell.z..=max_cell.z {
                    if self.is_solid(GridPos { x, y, z }) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
