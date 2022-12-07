#[derive(Clone)]
pub struct Foliage {
    pub foliage_type: FoliageType,
    pub weight: u16
}

#[derive(Clone, Copy)]
pub enum FoliageType {
    NONE,
    CACTUS
}

pub fn get_foliage_paths(foliage_type: FoliageType) -> &'static str {
    match foliage_type {
        FoliageType::CACTUS => {"foliage/cactus.png"}
        FoliageType::NONE => {""}
    }
}