use bevy::{
    prelude::*,
};
use noise::{NoiseFn, Perlin};

use crate::terrain::meshing::ChunkTileMapBuilder;

pub const TEXTURE_DIMENSION: f32 = 8.0;
pub const TEXTURES: usize = 64;
pub const AIR: usize = TEXTURES - 1;

pub const DIRT:  usize = 0;
pub const GRASS: usize = 1;
pub const STONE: usize = 2;
pub const WATER: usize = TEXTURE_DIMENSION as usize;

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
        cord: Vec2,
        seed: u32
    ) -> Self {
        let mut prng = Perlin::new(seed);
        let mut blocks = [[AIR; CHUNK_SIZE]; CHUNK_SIZE];
        let coordinate = cord;

        let frequency = 0.1;
        let octaves = 5;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let point =
                    (Vec2::new(x as f32, y as f32) / CHUNK_SIZE as f32 + coordinate)
                    * frequency;

                let mut val = 0.0;
                let mut den = 0.0;

                for i in 0..octaves {
                    val += 1.0 / (i as f64).powi(i)
                        * prng.get(
                        [
                            point.x as f64 * (i as f64).powi(i),
                            point.y as f64 * (i as f64).powi(i)
                        ]
                    );

                    den += 1.0 / (i as f64).powi(i);
                }

                val *= (3.0 / den) - 0.1;

                if val > 0. {
                    blocks[x][y] = val as usize;
                } else {
                    blocks[x][y] = WATER;
                }
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