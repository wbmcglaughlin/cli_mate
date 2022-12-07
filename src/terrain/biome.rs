use bevy::prelude::Resource;
use crate::terrain::chunk::CHUNK_SIZE;

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

    pub fn get_biome(
        &self,
        biome_type: BiomeType
    ) -> &Biome {
        for biome in &self.biomes {
            if biome.biome_type == biome_type {
                return biome;
            }
        }

        panic!("Biome not found");
    }

    pub fn get_biome_from_rng(
        &self,
        rng: f32
    ) -> BiomeType {
        assert!(rng < 1.0 && rng >= 0.0);

        let mut sum: u16 = 0;
        let val: u16 = (rng * self.biomes_weight_sum as f32) as u16;

        for biome in &self.biomes {
            sum += biome.weight;
            if sum > val {
                return biome.biome_type;
            }
        }

        panic!("Tile weight calculation error.")
    }

    pub fn get_biome_type_array_from_rng(
        &self,
        rng_array: [[f32; CHUNK_SIZE]; CHUNK_SIZE]
    ) -> [[BiomeType; CHUNK_SIZE]; CHUNK_SIZE] {
        let mut biome_type_array = [[BiomeType::PLAINS; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                biome_type_array[x][y] = self.get_biome_from_rng(rng_array[x][y]);
            }
        }

        biome_type_array
    }
}

#[derive(Clone)]
pub struct Biome {
    tiles: Vec<TileType>,
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
        tile: TileType
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
            biome_type: BiomeType::PLAINS,
            weight: 0,
            tiles_weight_sum: 0
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BiomeType {
    PLAINS,
    DESERT,
    GRASSLAND,
    BEACH,
    OCEAN
}

#[derive(Clone)]
pub struct TileType {
    pub(crate) tile_type: usize,
    pub weight: u16
}

impl TileType {
    pub fn set_weight(
        mut self,
        weight: u16
    ) -> TileType {
        self.weight = weight;

        self
    }
}