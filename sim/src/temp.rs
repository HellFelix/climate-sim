use std::f32::consts::PI;

use crate::{
    consts::*,
    planet::{Planet, PlanetRenderTexture},
    rk4::heat_eq_step_spherical,
};
use bevy::prelude::*;
use ndarray::Array2;

#[derive(Component)]
// We use a vector because an array of this size would overflow the thread stack.
pub struct TempMap(Array2<f32>); // T(x, y)
impl TempMap {
    pub fn new(init_temp: Array2<f32>) -> Self {
        Self(init_temp)
    }

    pub fn apply_heat_eq(&mut self) {
        self.0 = heat_eq_step_spherical(&self.0, SIM_DT);
        let first_avg = self.0.column(0).iter().map(|t| *t).sum::<f32>() / WIDTH as f32;
        self.0.column_mut(0).iter_mut().for_each(|t| *t = first_avg);

        let last_avg = self.0.column(HEIGHT - 1).iter().map(|t| *t).sum::<f32>() / WIDTH as f32;
        self.0
            .column_mut(HEIGHT - 1)
            .iter_mut()
            .for_each(|t| *t = last_avg);

        info!("{}", self.0.column(0));
    }

    pub fn set_at(&mut self, x: usize, y: usize, t: f32) {
        self.0[[x, y]] = t;
    }

    pub fn add_heat(&mut self, rhs: Array2<f32>) {
        //self.0 = rhs;
        self.0.scaled_add(1., &rhs);
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

    pub fn radiate_black_body(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let t = self.0[[x, y]];
                self.0[[x, y]] -= t.powi(4) * EPS * SIGMA * DA / C;
            }
        }
    }

    pub fn get_heat_texture(&self) -> Vec<u8> {
        let mut colors = Vec::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let heat_color = heat_color(self.0[[x, y]], 0., 200.);
                colors.append(&mut heat_color.to_vec());
            }
        }

        colors
    }
}

pub fn apply_heat_eq(mut temp_query: Query<&mut TempMap>) {
    let mut sim_steps = SPEEDUP;

    let mut temp = temp_query.single_mut().unwrap();
    while sim_steps >= 1 {
        temp.apply_heat_eq();
        sim_steps -= 1;
    }
}

pub fn apply_temp_image(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    render_tex: Res<PlanetRenderTexture>,
    planet_query: Query<&mut MeshMaterial3d<StandardMaterial>, With<Planet>>,
    temp_query: Query<&TempMap>,
) {
    let temp = temp_query.single().unwrap();

    let image = images.get_mut(&render_tex.0).unwrap();
    if let Some(ref mut data) = image.data {
        let new_data = temp.get_heat_texture();
        *data = new_data;
    }

    let planet_mesh = planet_query.single().unwrap();
    let mesh = materials.get_mut(&planet_mesh.0).unwrap();
    if let Some(ref mut base_color_texture) = mesh.base_color_texture {
        *base_color_texture = render_tex.0.clone();
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
