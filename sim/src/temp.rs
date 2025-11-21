use std::f32::consts::PI;

use crate::{consts::*, planet::PlanetRenderTexture, rk4::heat_eq_step};
use bevy::prelude::*;
use ndarray::Array2;

#[derive(Component)]
// We use a vector because an array of this size would overflow the thread stack.
pub struct TempMap(Array2<f32>); // T(x, y)
impl TempMap {
    pub fn new(init_temp: Array2<f32>) -> Self {
        info!("initial shape: {:?}", init_temp.shape());
        Self(init_temp)
    }

    pub fn apply_heat_eq(&mut self) {
        self.0 = heat_eq_step(&self.0, H);
    }

    // fn temp_at(&self, phi: f32, theta: f32) -> f32 {
    //     self.0[(phi * WIDTH as f32) as usize][(theta * HEIGHT as f32) as usize]
    // }
    pub fn set_heat(&mut self, f: fn(f32, f32) -> f32) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let temp = f(
                    2. * PI * x as f32 / WIDTH as f32,
                    PI * y as f32 / HEIGHT as f32,
                );
                self.0[[x, y]] = temp;
            }
        }
    }

    pub fn get_heat_texture(&mut self) -> Vec<u8> {
        let mut colors = Vec::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let heat_color = heat_color(self.0[[x, y]], 0., 1000.);
                colors.append(&mut heat_color.to_vec());
            }
        }

        colors
    }
}

pub fn apply_heat_eq(
    mut images: ResMut<Assets<Image>>,
    render_tex: Res<PlanetRenderTexture>,
    mut temp_query: Query<&mut TempMap>,
) {
    let mut temp = temp_query.single_mut().unwrap();
    temp.apply_heat_eq();

    let image = images.get_mut(&render_tex.0).unwrap();
    if let Some(ref mut data) = image.data {
        info!("temp is {:?}", temp.0);
        let new_data = temp.get_heat_texture();
        *data = new_data;
    }
}

/// Heat added from solar rays in each point where phi/theta are points
/// Ouput size [[f32; HEIGHT]; WIDTH]
fn heat_diff(phi: f32, theta: f32) -> Vec<Vec<f32>> {
    unimplemented!()
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
