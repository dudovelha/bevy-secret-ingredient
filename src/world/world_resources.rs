use crate::grid::grid::{VoxelGrid, CELL_SIZE_F};
use crate::grid::grid_position::GridPosition;
use crate::grid::tile::{TileData, TileType};
use bevy::math::Vec3;
use std::fs;
use bevy::log;

pub(crate) struct WorldResources;

impl WorldResources {
    pub fn load_grid(grid: &mut VoxelGrid, path: &str) {
        if let Ok(level_data) = fs::read_to_string(path) {
            let level_data: Vec<Vec<Vec<u8>>> = serde_json::from_str(&level_data).unwrap();

            grid.size = level_data[0].len() as i32;
            grid.origin = Vec3::new(
                grid.size as f32 * 0.5 * CELL_SIZE_F,
                CELL_SIZE_F / 2.0,
                grid.size as f32 * 0.5 * CELL_SIZE_F,
            );
            for (y, levels) in level_data.iter().enumerate() {
                for (z, lines) in levels.iter().enumerate() {
                    for (x, tile) in lines.iter().enumerate() {
                        if *tile == 0 {
                            continue;
                        }
                        grid.set(
                            GridPosition {
                                x: x as i32,
                                y: y as i32,
                                z: z as i32,
                            },
                            TileData {
                                tile_type: match tile {
                                    1 => TileType::Ground,
                                    2 => TileType::Stone,
                                    3 => TileType::Dirt,
                                    _ => panic!("Invalid tile type"),
                                },
                            },
                        )
                    }
                }
            }
        } else {
            let error = "Failed to load level file";
            log::error!(error);
            panic!("{}", error);
        }
    }
}