use bevy::{
    prelude::*,
};
use crate::terrain::biome::BiomeHandle;
use crate::terrain::meshing::ChunkTileMapBuilder;
use crate::terrain::noise::get_noise;

pub const CHUNK_SIZE: usize = 16;

pub const TILE_SIZE: f32 = 1.0;
pub const CHUNK_SIDE_SIZE: f32 = TILE_SIZE * CHUNK_SIZE as f32;

#[derive(Component)]
pub struct Chunk {
    pub blocks: [[usize; CHUNK_SIZE]; CHUNK_SIZE],
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
        let mut blocks = [[0; CHUNK_SIZE]; CHUNK_SIZE];

        // Get noise map for terrain type
        let noise = get_noise(coordinate, seed, 0.7, 5);

        // Get noise map for biome type
        let biome_noise = get_noise(coordinate, seed, 0.1, 3);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                blocks[x][y] = biome_handle.get_biome_from_rng(biome_noise[x][y]).get_tile_from_rng(noise[x][y]);
            }
        }

        Chunk {
            blocks,
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
        self.blocks[x][y] = block;
    }

    pub fn generate_mesh (
        &mut self,
    ) -> Mesh {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                self.chunk_tile_map_builder.add_tile(
                    Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE),
                    self.blocks[x][y]);
            }
        }

        let mesh = self.chunk_tile_map_builder.build();

        mesh
    }

    pub fn clear_builder(
        &mut self,
    ) {
        self.chunk_tile_map_builder.clear();
    }
}