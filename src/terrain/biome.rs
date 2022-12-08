use bevy::prelude::Resource;
use crate::terrain::chunk::CHUNK_SIZE;
use crate::terrain::foliage::{Foliage, FoliageType};

#[derive(Resource)]
pub struct BiomeHandle {
    pub biomes: Vec<Biome>,
    pub biomes_weight_sum: u16
}

impl BiomeHandle {
    pub fn new() -> Self {
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

    /// Retrieve biome in biome handle of biome type.
    ///
    /// panics when biome is not found
    pub fn get_biome(
        &self,
        biome_type: BiomeType
    ) -> &Biome {
        for biome in &self.biomes {
            if biome.biome_type == biome_type {
                return biome;
            }
        }

        // Get biome should never be called when the biome does not exist.
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
    foliage: Vec<Foliage>,
    pub weight: u16,
    pub biome_type: BiomeType,
    pub foliage_density: f32,
    tiles_weight_sum: u16,
    foliage_weight_sum: u16
}

impl Biome {
    pub fn new(
        weight: u16,
        biome_type: BiomeType,
        foliage_density: f32
    ) -> Self {
        Biome {
            tiles: Vec::new(),
            foliage: Vec::new(),
            weight,
            biome_type,
            foliage_density,
            tiles_weight_sum: 0,
            foliage_weight_sum: 0
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

    pub fn add_foliage(
        mut self,
        foliage: Foliage
    ) -> Self {
        self.foliage.push(foliage.clone());
        self.foliage_weight_sum += foliage.weight;
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

    pub fn get_foliage_from_rng(
        &self,
        rng: f32,
        tile_type: usize
    ) -> FoliageType {
        assert!(rng < 1.0 && rng >= 0.0);

        if self.foliage_weight_sum > 0 {
            if rng < self.foliage_density {
                let mut sum: u16 = 0;
                let val: u16 = (rng * self.foliage_weight_sum as f32) as u16;

                for foliage in &self.foliage {
                    sum += foliage.weight;
                    if sum > val {
                        if foliage.spawns_on.contains(&tile_type) {
                            return foliage.foliage_type;
                        }
                    }
                }
            }
        }

        FoliageType::NONE
    }
}

impl Default for Biome {
    fn default() -> Self {
        Biome {
            tiles: Vec::new(),
            foliage: Vec::new(),
            biome_type: BiomeType::PLAINS,
            weight: 0,
            foliage_density: 0.0,
            tiles_weight_sum: 0,
            foliage_weight_sum: 0
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