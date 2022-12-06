use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use crate::terrain::chunk::CHUNK_SIZE;

pub fn get_noise(
    coordinate: Vec2,
    seed: u32,
) -> [[f32; CHUNK_SIZE]; CHUNK_SIZE] {
    let prng = Perlin::new(seed);

    let frequency = 1.0;
    let octaves = 5;

    let mut noise = [[0.0; CHUNK_SIZE]; CHUNK_SIZE];

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let point =
                (Vec2::new(x as f32, y as f32) / CHUNK_SIZE as f32 + coordinate)
                    * frequency;

            let mut val = 0.0;
            let mut den = 0.0;

            for i in 0..octaves {
                let pow_val = (2.0 as f64).powi(i);

                val += 0.5 / pow_val
                    * (1.0 + prng.get(
                    [
                        point.x as f64 * pow_val,
                        point.y as f64 * pow_val
                    ])
                );

                den += 1.0 / pow_val;
            }

            val /= den;

            noise[x][y] = val.abs() as f32;
        }
    }

    noise
}