extern crate core;

mod terrain;
mod entities;

use bevy::{
    prelude::*,
};

use bevy::window::PresentMode;
use bevy_debug_text_overlay::OverlayPlugin;
use crate::entities::player::PlayerPlugin;
use crate::terrain::chunk_handler::ChunkHandlerPlugin;
use crate::terrain::SurfacePlugin;
use crate::terrain::terrain::TerrainPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "cli mate".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugin(OverlayPlugin { font_size: 22.0, ..default() })
        .add_plugin(SurfacePlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.03,
            ..default()
        },
        ..default()
    },MainCamera));
}