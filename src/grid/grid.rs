use crate::grid::grid_position::GridPosition;
use crate::grid::tile::TileData;
use crate::objects::object::ObjectId;
use crate::physics::physics_world::PhysicsWorld;
use bevy::prelude::*;
use std::collections::HashMap;

pub const CELL_SIZE: i32 = 1;
pub const CELL_SIZE_F: f32 = CELL_SIZE as f32;

#[derive(Default, Resource)]
pub struct VoxelGrid {
    pub tiles: HashMap<GridPosition, TileData>,
    pub objects: HashMap<GridPosition, Vec<ObjectId>>,
    pub size: i32,
    pub origin: Vec3,
}

impl VoxelGrid {
    pub fn is_solid(&self, pos: GridPosition) -> bool {
        self.tiles.contains_key(&pos)
    }

    pub fn get(&self, pos: GridPosition) -> Option<&TileData> {
        self.tiles.get(&pos)
    }

    pub fn get_mut(&mut self, pos: GridPosition) -> Option<&mut TileData> {
        self.tiles.get_mut(&pos)
    }

    pub fn set(&mut self, pos: GridPosition, tile: TileData) {
        self.tiles.insert(pos, tile);
    }

    pub fn remove(&mut self, pos: GridPosition) {
        self.tiles.remove(&pos);
    }
}

impl PhysicsWorld for VoxelGrid {
    fn is_aabb_blocked(&self, min: Vec3, max: Vec3) -> bool {
        let min_cell = GridPosition::from_world(min, self.origin);
        let max_cell = GridPosition::from_world(max, self.origin);

        for x in min_cell.x..=max_cell.x {
            for y in min_cell.y..=max_cell.y {
                for z in min_cell.z..=max_cell.z {
                    if self.is_solid(GridPosition { x, y, z }) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
