use bevy::prelude::*;
use crate::terrain::biome::BiomeHandle;

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
        });
    }
}

fn init_biomes(
    mut biome_handle: ResMut<BiomeHandle>
) {

}