use bevy::{
    prelude::*,
};
use crate::terrain::biome::{BiomeHandle, BiomeType};
use crate::terrain::foliage::FoliageType;
use crate::terrain::meshing::ChunkTileMapBuilder;
use crate::terrain::noise::get_noise;
use crate::terrain::tile::Tile;

pub const CHUNK_SIZE: usize = 16;

pub const TILE_SIZE: f32 = 1.0;
pub const CHUNK_SIDE_SIZE: f32 = TILE_SIZE * CHUNK_SIZE as f32;

#[derive(Component)]
pub struct Chunk {
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    pub foliage_type: [[FoliageType; CHUNK_SIZE]; CHUNK_SIZE],
    pub coordinate: Vec2,
    chunk_tile_map_builder: ChunkTileMapBuilder,
}

#[derive(Component)]
pub struct ChunkCoordinate {
    pub coordinate: Vec2,
}

impl Chunk {
    pub fn new(
        coordinate: Vec2,
        seed: u32,
        biome_handle: &ResMut<BiomeHandle>
    ) -> Self {
        // Init blocks for chunk mesh layer
        let mut tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE] = [[Tile::default(); CHUNK_SIZE]; CHUNK_SIZE];

        // Get noise map for terrain type
        let noise = get_noise(coordinate, seed, 0.7, 5);

        // Get noise map for biome type
        let biome_noise = get_noise(coordinate, seed, 0.1, 3);
        let biome_type: [[BiomeType; CHUNK_SIZE]; CHUNK_SIZE] = biome_handle.get_biome_type_array_from_rng(biome_noise);

        // Foliage array
        let foliage_noise = get_noise(coordinate, seed, 5.0, 5);
        let mut foliage_type: [[FoliageType; CHUNK_SIZE]; CHUNK_SIZE] = [[FoliageType::NONE; CHUNK_SIZE]; CHUNK_SIZE];

        // Iterate over each tile in chunk
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let biome = biome_handle.get_biome(biome_type[x][y]);
                let tile_type = biome.get_tile_from_rng(noise[x][y]);

                // Set tile
                tiles[x][y] = Tile {
                    tile: tile_type,
                    biome: biome_type[x][y].clone()
                };

                // Set foliage
                foliage_type[x][y] = biome.get_foliage_from_rng(foliage_noise[x][y]);
            }
        }

        Chunk {
            tiles,
            foliage_type,
            coordinate,
            chunk_tile_map_builder: ChunkTileMapBuilder::default()
        }
    }

    #[allow(dead_code)]
    pub fn set_block(
        &mut self,
        x: usize,
        y: usize,
        block: usize
    ) {
        self.tiles[x][y] = Tile {
            tile: block,
            biome: BiomeType::PLAINS
        };
    }

    pub fn generate_mesh (
        &mut self,
    ) -> Mesh {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                self.chunk_tile_map_builder.add_tile(
                    Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE),
                    self.tiles[x][y].tile);
            }
        }

        let mesh = self.chunk_tile_map_builder.build();

        // Clear chunk builder
        self.clear_builder();

        mesh
    }

    pub fn clear_builder(
        &mut self,
    ) {
        self.chunk_tile_map_builder.clear();
    }
}