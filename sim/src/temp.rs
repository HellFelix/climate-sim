use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{consts::*, planet::PlanetRenderTexture};

#[derive(Component)]
// We use a vector because an array of this size would overflow the thread stack.
pub struct TempMap(Vec<Vec<f32>>); // T(x, y)
impl TempMap {
    pub fn new(init_temp: Vec<Vec<f32>>) -> Self {
        Self(init_temp)
    }

    // fn temp_at(&self, phi: f32, theta: f32) -> f32 {
    //     self.0[(phi * WIDTH as f32) as usize][(theta * HEIGHT as f32) as usize]
    // }

    pub fn get_heat_texture(&mut self, f: fn(f32, f32) -> f32) -> Vec<u8> {
        let mut colors = Vec::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let temp = f(
                    2. * PI * x as f32 / WIDTH as f32,
                    PI * y as f32 / HEIGHT as f32,
                );
                self.0[x][y] = temp;
                let heat_color = heat_color(temp, 0., 1000.);
                colors.append(&mut heat_color.to_vec());
            }
        }

        colors
    }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn heat_color(value: f32, min_temp: f32, max_temp: f32) -> [u8; 4] {
    let t = clamp((value - min_temp) / (max_temp - min_temp), 0.0, 1.0);

    let r = clamp(255.0 * (1.5 - (4.0 * t - 3.0).abs()), 0.0, 255.0) as u8;
    let g = clamp(255.0 * (1.5 - (4.0 * t - 2.0).abs()), 0.0, 255.0) as u8;
    let b = clamp(255.0 * (1.5 - (4.0 * t - 1.0).abs()), 0.0, 255.0) as u8;

    [r as u8, g as u8, b as u8, 255]
}
