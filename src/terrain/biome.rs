use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BiomeHandle {
    pub biomes: Vec<Biome>
}

pub struct Biome {
    tiles: Vec<Tile>,
    weight: u16
}

pub struct Tile {
    tile_type: u8,
    weight: u16
}