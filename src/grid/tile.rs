use bevy::color::Color;

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