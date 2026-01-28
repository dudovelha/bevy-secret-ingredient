use crate::grid::grid::{GridPos, VoxelGrid};
use crate::grid::tile::{TileData, TileType};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::log;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Handle, ResMut, Resource};
use std::collections::HashMap;
use std::fs;

#[derive(Resource, Debug, Default)]
pub struct GridAssets {
    pub assets: HashMap<TileType, Handle<StandardMaterial>>,
}

impl GridAssets {
    pub fn get_tile_material(&self, tile_type: TileType) -> Option<&Handle<StandardMaterial>> {
        self.assets.get(&tile_type)
    }

    pub fn load_tile_materials(&mut self, materials: &mut ResMut<Assets<StandardMaterial>>) {
        self.assets
            .insert(TileType::Ground, materials.add(Color::srgb(0.3, 0.7, 0.3)));
        self.assets
            .insert(TileType::Stone, materials.add(Color::srgb(0.5, 0.5, 0.5)));
        self.assets
            .insert(TileType::Dirt, materials.add(Color::srgb(0.6, 0.4, 0.2)));
    }

    pub fn load_grid(&mut self, grid: &mut VoxelGrid) {
        let level_data = fs::read_to_string("assets/grid/level1.json").unwrap();
        let level_data: Vec<Vec<Vec<u8>>> = serde_json::from_str(&level_data).unwrap();

        grid.size = level_data[0].len() as i32;
        for (y, levels) in level_data.iter().enumerate() {
            for (z, lines) in levels.iter().enumerate() {
                for (x, tile) in lines.iter().enumerate() {
                    if *tile == 0 {
                        continue;
                    }
                    grid.set(
                        GridPos {
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
    }
}
