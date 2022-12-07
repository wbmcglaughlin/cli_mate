use bevy::app::Plugin;
use crate::{App, ChunkHandlerPlugin, TerrainPlugin};

pub mod meshing;
pub mod chunk_handler;
pub mod chunk;
pub mod noise;
pub mod biome;
pub mod terrain;
pub mod foliage;
pub mod tile;

pub struct SurfacePlugin;
impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TerrainPlugin)
            .add_plugin(ChunkHandlerPlugin);
    }
}