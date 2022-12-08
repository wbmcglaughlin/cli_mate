use bevy::{
    prelude::*,
};
use bevy::sprite::{MaterialMesh2dBundle};
use bevy::utils::HashSet;
use crate::terrain::chunk::{Chunk, CHUNK_SIDE_SIZE, CHUNK_SIZE, ChunkCoordinate, TILE_SIZE};
use crate::entities::player::{Player};
use crate::terrain::biome::BiomeHandle;
use crate::terrain::foliage::{FoliageType, get_foliage_paths};

pub const VISIBLE_CHUNKS: i32 = 3;

pub struct ChunkHandlerPlugin;
impl Plugin for ChunkHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkHandler {
            chunks: Vec::new(),
            chunks_to_remesh: Vec::new()
        }).add_system(update_chunks)
            .add_system(remove_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkHandler {
    pub chunks: Vec<Chunk>,
    pub chunks_to_remesh: Vec<Vec2>
}

impl ChunkHandler {
    pub fn contains_chunk(
        &self,
        chunk_coordinate: Vec2
    ) -> bool {
        for chunk in &self.chunks {
            if chunk.coordinate == chunk_coordinate {
                return true;
            }
        }

        false
    }

    pub fn get_chunk(
        &mut self,
        chunk_coordinate: Vec2
    ) -> &mut Chunk {
        for chunk in self.chunks.iter_mut() {
            if chunk.coordinate == chunk_coordinate {
                return chunk;
            }
        }

        panic!("Chunk does not exist")
    }

    #[allow(dead_code)]
    pub fn get_chunk_xy(
        &mut self,
        coordinate: Vec2
    ) -> (&mut Chunk, usize, usize) {
        let chunk_coord = (coordinate / CHUNK_SIDE_SIZE).floor();
        let chunk = self.get_chunk(chunk_coord);

        // Get x and y array positions.
        let x = (coordinate.x - chunk.coordinate.x * CHUNK_SIDE_SIZE).floor() as usize;
        let y = (coordinate.y - chunk.coordinate.y * CHUNK_SIDE_SIZE).floor() as usize;

        (chunk, x, y)
    }

    #[allow(dead_code)]
    pub fn update_chunk(
        &mut self,
        chunk_coordinate: Vec2,
        x: usize,
        y: usize,
        new_block: usize
    ) {
        let chunk = self.get_chunk(chunk_coordinate);
        chunk.set_block(x, y, new_block);
    }

    #[allow(dead_code)]
    pub fn chunks_to_remesh(
        &mut self,
        chunk_coord: Vec2
    ) {
        self.chunks_to_remesh.push(chunk_coord);
    }
}

fn remove_chunks(
    mut commands: Commands,
    mut chunk_handler: ResMut<ChunkHandler>,
    mut player: Query<(&Transform, &mut Player), (With<Player>, Without<ChunkCoordinate>)>,
    mut chunks: Query<(Entity, &mut ChunkCoordinate), With<ChunkCoordinate>>
) {
    for (transform, mut player) in player.iter_mut() {
        if player.distance_moved > CHUNK_SIDE_SIZE / 2.0 {
            player.distance_moved = 0.;

            let mut chunks_to_remove: HashSet<Entity> = HashSet::new();

            for (chunk_entity, chunk_coordinate) in chunks.iter_mut() {
                let distance = transform.translation.distance_squared(chunk_coordinate.coordinate.extend(0.0) * CHUNK_SIDE_SIZE);

                if distance > CHUNK_SIDE_SIZE * CHUNK_SIDE_SIZE * VISIBLE_CHUNKS as f32 * VISIBLE_CHUNKS as f32 {
                    chunk_handler.chunks.retain(|chunk| (*chunk).coordinate != chunk_coordinate.coordinate);
                    chunks_to_remove.insert(chunk_entity);
                }
            }

            for chunk_ent in chunks_to_remove.iter() {
                commands.entity(*chunk_ent).despawn_recursive();
            }
        }
    }
}

pub fn update_chunks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<&Player, With<Player>>,
    chunks: Query<(Entity, &mut ChunkCoordinate), (With<ChunkCoordinate>, Without<Player>)>,
    mut chunk_handler: ResMut<ChunkHandler>,
    biome_handle: ResMut<BiomeHandle>
) {
    // Remesh Chunks
    for coord_to_remesh in chunk_handler.chunks_to_remesh.clone() {
        for (entity, chunk_coord) in chunks.iter() {
            if chunk_coord.coordinate == coord_to_remesh {
                // Get chunk to remesh
                let mut chunk = chunk_handler.get_chunk(coord_to_remesh);

                // De-spawn old chunk
                commands.entity(entity).despawn_recursive();

                // Spawn new chunk
                spawn_chunk(&mut commands,
                            &asset_server,
                            &mut meshes,
                            &mut materials,
                            &mut chunk);
            }
        }
    }

    chunk_handler.chunks_to_remesh.clear();

    // Generate Chunks
    for player in players.iter() {
        let player_coordinate = Vec2::new((player.pos.x / CHUNK_SIDE_SIZE).floor(), (player.pos.y / CHUNK_SIDE_SIZE).floor());

        for x in (-VISIBLE_CHUNKS+1)..VISIBLE_CHUNKS {
            for y in (-VISIBLE_CHUNKS+1)..VISIBLE_CHUNKS {
                let coord = player_coordinate + Vec2::new(x as f32, y as f32);
                if !chunk_handler.contains_chunk(coord) {
                    let mut chunk = Chunk::new(coord, 0, &biome_handle);

                    spawn_chunk(&mut commands,
                                &asset_server,
                                &mut meshes,
                                &mut materials,
                                &mut chunk);

                    chunk_handler.chunks.push(chunk);
                }
            }
        }
    }
}

pub fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    chunk: &mut Chunk
) {
    let mesh = chunk.generate_mesh();

    let chunk_entity = commands.spawn((ChunkCoordinate {
        coordinate: chunk.coordinate
    }, MaterialMesh2dBundle  {
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::from(asset_server.load("tiles/tiles.png"))),
        transform: Transform::from_xyz(
            chunk.coordinate.x * CHUNK_SIDE_SIZE,
            chunk.coordinate.y * CHUNK_SIDE_SIZE,
            0.0),
        ..Default::default()
    })).id();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            if chunk.foliage_type[x][y] != FoliageType::NONE {
                let foliage_entity = commands.spawn(SpriteBundle {
                    texture: asset_server.load(get_foliage_paths(chunk.foliage_type[x][y])),
                    transform: Transform::from_xyz(
                        x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                        y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                        1.0).with_scale(Vec3::splat(1.0 / 16.0)),
                    ..default()
                }).id();

                commands.entity(foliage_entity).set_parent(chunk_entity);
            }
        }
    }
}