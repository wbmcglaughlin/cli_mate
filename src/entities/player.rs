use bevy::{
    prelude::*,
};

use crate::MainCamera;
use crate::entities::control::player_movement;

#[derive(Component, Deref, DerefMut)]
struct PlayerAnimationTimer(Timer);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(update_camera)
            .add_system(player_movement)
            .add_system(animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Player,
        &mut PlayerAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (player, mut timer,
        mut sprite, texture_atlas_handle) in &mut query {
        // Check if the entities is moving.
        if player.vel.length_squared() > 0. {
            // Check which direction entities is moving in.
            if player.vel.x < 0. {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }

            // Tick animation timer
            timer.tick(time.delta());
            if timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        } else {
            sprite.index = 0;
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load texture for entities
    let texture_handle = asset_server.load("sprites/player/player_walk.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle,
                                                Vec2::new(16.0, 16.0),
                                                3, 1,
                                                None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Set entities position
    let player_position = Vec2::new(0.0, 2.0);

    commands.spawn((
        Player {
            pos: player_position,
            vel: Vec2::default(),
            acc: Vec2::default(),
            distance_moved: 0.0,
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(
                player_position.extend(2.0))
                .with_scale(Vec3::splat(1. / 16.)),
            ..default()
        },
        PlayerAnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating))
    ));
}

#[derive(Component)]
pub struct Player {
    pub pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) acc: Vec2,

    pub distance_moved: f32,
}

impl Player {
    pub fn update(&mut self, dt: f32) {
        self.vel += dt * self.acc;

        self.pos += self.vel * dt;

        self.distance_moved += (dt * self.vel).length();

        self.vel -= self.vel * self.vel.length() * 0.9 * dt;

        if self.vel.length_squared() < 2.0 {
            self.vel = Vec2::default();
        }

        self.acc = Vec2::default();
    }

    pub fn add_acc(&mut self, acc: Vec2) {
        self.acc += acc;
    }
}

pub fn update_camera(
    transforms: Query<&mut Player, With<Player>>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>
) {
    for player in transforms.iter() {
        for mut camera in camera.iter_mut() {
            camera.translation.x = player.pos.x;
            camera.translation.y = player.pos.y;
        }
    }
}