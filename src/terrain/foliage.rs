use bevy::{
    prelude::*,
};
use crate::terrain::biome::Biome;

pub struct ChunkFoliage {
    foliage: Vec<Foilage>
}

pub struct Foliage {
    foliage_type: FoliageType,
    biomes: Vec<Biome>
}

pub enum FoliageType {
    CACTUS
}

pub fn get_foliage_paths(foliage_type: FoliageType) -> &str {
    let mut path = "foliage/";

    match foliage_type {
        FoliageType::CACTUS => {path + "cactus.png"}
    }
}