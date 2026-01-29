use bevy::color::Color;
use bevy::prelude::Component;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TileType {
    Ground,
    Stone,
    Dirt,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileData {
    pub tile_type: TileType
}

#[derive(Default, Component)]
pub struct GridTile;