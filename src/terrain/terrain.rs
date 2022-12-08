use bevy::prelude::*;
use crate::terrain::biome::{Biome, BiomeHandle, BiomeType, TileType};
use crate::terrain::foliage::{Foliage, FoliageType};

pub const TEXTURE_DIMENSION: f32 = 8.0;

pub const DIRT:  usize = 0;
pub const GRASS: usize = 1;
pub const STONE: usize = 2;
pub const SAND: usize = 3;
pub const WATER: usize = TEXTURE_DIMENSION as usize;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            BiomeHandle::new()
        ).add_startup_system(init_biomes);
    }
}

fn init_biomes(
    mut biome_handle: ResMut<BiomeHandle>
) {

    // Tile Type Init
    let grass_tile: TileType = TileType {
        tile_type: GRASS,
        weight: 10
    };

    let dirt_tile: TileType = TileType {
        tile_type: DIRT,
        weight: 5
    };

    let stone_tile: TileType = TileType {
        tile_type: STONE,
        weight: 5
    };

    let sand_tile: TileType = TileType {
        tile_type: SAND,
        weight: 5
    };

    let water_tile: TileType = TileType {
        tile_type: WATER,
        weight: 6
    };

    // Foliage Init
    let cactus: Foliage = Foliage {
        foliage_type: FoliageType::CACTUS,
        weight: 10,
        scale: 1.0 / 16.0,
        spawns_on: vec![SAND]
    };

    let rose: Foliage = Foliage {
        foliage_type: FoliageType::ROSE,
        weight: 10,
        scale: 1.0 / 16.0,
        spawns_on: vec![GRASS]
    };

    let rock: Foliage = Foliage {
        foliage_type: FoliageType::ROCK,
        weight: 10,
        scale: 1.0 / 16.0,
        spawns_on: vec![STONE, DIRT, GRASS]
    };

    // Biome 0: PLAINS BIOME
    let plains_biome: Biome = Biome::new(20, BiomeType::PLAINS, 0.3)
        .add_tile(grass_tile.clone())
        .add_tile(dirt_tile.clone())
        .add_tile(stone_tile.clone())
        .add_tile(water_tile.clone())
        .add_foliage(rock.clone());

    // Biome 1: DESERT BIOME
    let desert_biome: Biome = Biome::new(10, BiomeType::DESERT, 0.3)
        .add_tile(dirt_tile.clone().set_weight(4))
        .add_tile(sand_tile.clone().set_weight(20))
        .add_foliage(cactus.clone());

    // Biome 2: GRASSLANDS BIOME
    let grassland_biome: Biome = Biome::new(10, BiomeType::GRASSLAND, 0.3)
        .add_tile(grass_tile.clone().set_weight(20))
        .add_tile(dirt_tile.clone().set_weight(4))
        .add_tile(stone_tile.clone().set_weight(4))
        .add_tile(water_tile.clone().set_weight(10))
        .add_foliage(rose.clone())
        .add_foliage(rock.clone());

    // Biome 3: BEACH BIOME
    let beach_biome: Biome = Biome::new(5, BiomeType::BEACH, 0.0)
        .add_tile(water_tile.clone().set_weight(4))
        .add_tile(sand_tile.clone().set_weight(10));

    let ocean_biome: Biome = Biome::new(40, BiomeType::OCEAN, 0.0)
        .add_tile(water_tile.clone().set_weight(30))
        .add_tile(sand_tile.clone().set_weight(4));

    // Add Biomes To Biome Handle
    biome_handle.add_biome(plains_biome);
    biome_handle.add_biome(desert_biome);
    biome_handle.add_biome(grassland_biome);
    biome_handle.add_biome(beach_biome);
    biome_handle.add_biome(ocean_biome);
}

