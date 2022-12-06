use bevy::prelude::*;
use crate::terrain::biome::{Biome, BiomeHandle, Tile};

pub const TEXTURE_DIMENSION: f32 = 8.0;
pub const TEXTURES: usize = 64;
pub const AIR: usize = TEXTURES - 1;

pub const DIRT:  usize = 0;
pub const GRASS: usize = 1;
pub const STONE: usize = 2;
pub const WATER: usize = TEXTURE_DIMENSION as usize;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BiomeHandle {
            biomes: Vec::new()
        }).add_startup_system(init_biomes);
    }
}

fn init_biomes(
    mut biome_handle: ResMut<BiomeHandle>
) {
    let grass_tile: Tile = Tile {
        tile_type: GRASS,
        weight: 10
    };

    let dirt_tile: Tile = Tile {
        tile_type: DIRT,
        weight: 5
    };

    let stone_tile: Tile = Tile {
        tile_type: STONE,
        weight: 5
    };

    // Biome 0: PLAINS BIOME
    let plains_biome: Biome = Biome::new(20)
        .add_tile(grass_tile.clone())
        .add_tile(dirt_tile.clone())
        .add_tile(stone_tile.clone());

    // Biome 1: DESERT BIOME
    let desert_biome: Biome = Biome::new(20)
        .add_tile(grass_tile.clone().set_weight(2))
        .add_tile(dirt_tile.clone())
        .add_tile(stone_tile.clone().set_weight(40));

    // Biome 2: GRASSLANDS BIOME
    // TODO: add biomes

    // Biome 3: BEACH BIOME


    biome_handle.biomes.extend(vec![plains_biome, desert_biome]);
}