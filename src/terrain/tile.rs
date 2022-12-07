use crate::terrain::biome::BiomeType;

#[derive(Copy, Clone)]
pub struct Tile {
    pub tile: usize,
    pub biome: BiomeType
}

impl Tile {
    pub fn default() -> Tile {
        Tile {
            tile: 0,
            biome: BiomeType::PLAINS
        }
    }
}