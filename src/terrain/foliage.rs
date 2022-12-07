use bevy::{
    prelude::*,
};

use crate::terrain::biome::BiomeType;

pub struct FoliagePlugin;
impl Plugin for FoliagePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoliageHandle::new())
            .add_startup_system(init_foliage);
    }
}

pub fn init_foliage(
    mut foliage_handle: ResMut<FoliageHandle>
) {
    let cactus: Foliage = Foliage {
        foliage_type: FoliageType::CACTUS,
        biomes: vec![BiomeType::DESERT]
    };

    foliage_handle.foliage_types.push(cactus);
}

#[derive(Resource)]
pub struct FoliageHandle {
    foliage_types: Vec<Foliage>
}

impl FoliageHandle {
    pub fn new() -> Self {
        FoliageHandle {
            foliage_types: Vec::new()
        }
    }

    pub fn add_foliage(
        &mut self,
        foliage: Foliage
    ) {
        self.foliage_types.push(foliage);
    }
}

pub struct ChunkFoliage {
    foliage: Vec<Foliage>
}

pub struct Foliage {
    pub foliage_type: FoliageType,
    pub biomes: Vec<BiomeType>
}

pub enum FoliageType {
    CACTUS
}

pub fn get_foliage_paths(foliage_type: FoliageType) -> &'static str {
    match foliage_type {
        FoliageType::CACTUS => {"foliage/cactus.png"}
    }
}