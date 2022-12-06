use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BiomeHandle {
    pub biomes: Vec<Biome>
}

pub struct Biome {
    pub tiles: Vec<Tile>,
    pub weight: u16
}

#[derive(Clone)]
pub struct Tile {
    pub(crate) tile_type: usize,
    pub weight: u16
}

impl Tile {
    pub fn set_weight(
        mut self,
        weight: u16
    ) -> Tile {
        self.weight = weight;

        self
    }
}