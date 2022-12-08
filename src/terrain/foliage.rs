use crate::terrain::biome::TileType;

#[derive(Clone)]
pub struct Foliage {
    pub foliage_type: FoliageType,
    pub weight: u16,
    pub scale: f32,
    pub spawns_on: Vec<usize>
}

#[derive(Clone, Copy, PartialEq)]
pub enum FoliageType {
    NONE,
    CACTUS,
    ROSE,
    ROCK,
}

pub fn get_foliage_paths(foliage_type: FoliageType) -> &'static str {
    match foliage_type {
        FoliageType::CACTUS => {"sprites/foliage/cactus.png"}
        FoliageType::NONE => {""}
        FoliageType::ROSE => {"sprites/foliage/rose.png"}
        FoliageType::ROCK => {"sprites/foliage/rock.png"}
    }
}