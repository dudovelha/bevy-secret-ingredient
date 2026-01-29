use crate::grid::tile::TileType;
use bevy::prelude::{Handle, ResMut, Resource, StandardMaterial};
use std::collections::HashMap;
use bevy::asset::Assets;
use bevy::color::Color;

#[derive(Default, Resource)]
pub struct WorldAssets {
    pub grid_assets: HashMap<TileType, Handle<StandardMaterial>>
}

impl WorldAssets {
    pub(crate) fn load_assets(&mut self, materials: &mut ResMut<Assets<StandardMaterial>>) {
        load_tile_materials(self, materials);
    }
}

pub fn load_tile_materials(world_assets: &mut WorldAssets, materials: &mut ResMut<Assets<StandardMaterial>>) {
    world_assets.grid_assets
        .insert(TileType::Ground, materials.add(Color::srgb(0.3, 0.7, 0.3)));
    world_assets.grid_assets
        .insert(TileType::Stone, materials.add(Color::srgb(0.5, 0.5, 0.5)));
    world_assets.grid_assets
        .insert(TileType::Dirt, materials.add(Color::srgb(0.6, 0.4, 0.2)));
}