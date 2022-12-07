use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BiomeHandle {
    pub biomes: Vec<Biome>,
    pub biomes_weight_sum: u16
}

impl BiomeHandle {
    pub fn new(

    ) -> Self {
        BiomeHandle {
            biomes: Vec::new(),
            biomes_weight_sum: 0
        }
    }

    pub fn add_biome(
        &mut self,
        biome: Biome
    ) {
        self.biomes.push(biome.clone());
        self.biomes_weight_sum += biome.weight;
    }

    pub fn get_biome_from_rng(
        &self,
        rng: f32
    ) -> &Biome {
        assert!(rng < 1.0 && rng >= 0.0);

        let mut sum: u16 = 0;
        let val: u16 = (rng * self.biomes_weight_sum as f32) as u16;

        for biome in &self.biomes {
            sum += biome.weight;
            if sum > val {
                return biome;
            }
        }

        panic!("Tile weight calculation error.")
    }
}

#[derive(Clone)]
pub struct Biome {
    tiles: Vec<Tile>,
    pub weight: u16,
    pub biome_type: BiomeType,
    tiles_weight_sum: u16
}

impl Biome {
    pub fn new(
        weight: u16,
        biome_type: BiomeType
    ) -> Self {
        Biome {
            tiles: Vec::new(),
            weight,
            biome_type,
            tiles_weight_sum: 0
        }
    }

    pub fn add_tile(
        mut self,
        tile: Tile
    ) -> Self {
        self.tiles.push(tile.clone());
        self.tiles_weight_sum += tile.weight.clone();
        self
    }

    pub fn get_tile_from_rng(
        &self,
        rng: f32
    ) -> usize {
        assert!(rng < 1.0 && rng >= 0.0);

        let mut sum: u16 = 0;
        let val: u16 = (rng * self.tiles_weight_sum as f32) as u16;

        for tile in &self.tiles {
            sum += tile.weight;
            if sum > val {
                return tile.tile_type;
            }
        }

        panic!("Tile weight calculation error.")
    }
}

impl Default for Biome {
    fn default() -> Self {
        Biome {
            tiles: Vec::new(),
            weight: 0,
            tiles_weight_sum: 0
        }
    }
}

pub enum BiomeType {
    PLAINS,
    DESERT,
    GRASSLAND,
    BEACH,
    OCEAN
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