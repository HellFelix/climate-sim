use std::f32::{INFINITY, consts::PI};

use crate::{consts::*, planet::Planet, temp::TempMap};

use bevy::prelude::*;
use ndarray::{Array2, arr2};

pub fn apply_heat_in(
    planet_query: Query<&Transform, With<Planet>>,
    mut temp_map_query: Query<&mut TempMap>,
) {
    let planet_transform = planet_query.single().unwrap();
    let mut temp_map = temp_map_query.single_mut().unwrap();

    // calculate from planet's frame of reference
    let center = planet_transform.translation;

    let origin_normal = (Vec3::ZERO - center).normalize();

    // Quaternion transformation into local reference frame
    let local_origin_normal = (planet_transform.rotation.conjugate() * origin_normal).normalize();

    // Calculation for coordinates in heat map plane
    // let theta = local_origin_normal.y.atan2(local_origin_normal.x);
    // let phi = (local_origin_normal.z / 1.).clamp(-1., 1.).acos();
    // let (x, y) = spherical_convert_nearest_coord(theta, phi);

    let flux = flux_pp(local_origin_normal);

    // info!("{flux}");
    // info!(
    //     "{}",
    //     flux.iter()
    //         .max_by(|a, b| a.partial_cmp(&b).unwrap())
    //         .unwrap()
    // );

    temp_map.add_heat(flux);
}

fn spherical_convert_nearest_coord(theta: f32, phi: f32) -> (usize, usize) {
    (
        (WIDTH as f32 * (if theta >= 0. { theta } else { 2. * PI + theta }) / (2. * PI)).floor()
            as usize,
        (HEIGHT as f32 * phi / PI).floor() as usize,
    )
}

// There's no way this should be > 1...
pub fn transmission_f(mu: f32) -> f32 {
    // Assuming mu is negative (clamped to 0)
    if mu > 0. {
        C_DIFF + (-TAU / mu).exp()
    } else {
        0.
    }
}

fn vector_from_coord(x: usize, y: usize) -> Vec3 {
    let r = 1.;
    let theta = (2. * PI / WIDTH as f32) * x as f32;
    let phi = (PI / HEIGHT as f32) * y as f32;

    Vec3 {
        x: r * phi.sin() * theta.cos(),
        y: r * phi.sin() * theta.sin(),
        z: r * phi.cos(),
    }
}

fn flux_pp(zenit: Vec3) -> Array2<f32> {
    let mut heat_matrix = arr2(&[[0.; HEIGHT]; WIDTH]);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let coord_vec = vector_from_coord(x, y);
            let mu = coord_vec.dot(zenit).clamp(0., INFINITY);
            let transmission = transmission_f(mu);
            let flux = (SOLAR_CONSTANT * mu * transmission) / (1.0 - RHO * R);
            heat_matrix[[x, y]] = flux;
        }
    }
    return heat_matrix;
}

pub fn apply_black_body_radiation(mut temp_map_query: Query<&mut TempMap>) {
    let mut temp_map = temp_map_query.single_mut().unwrap();
    temp_map.radiate_black_body();
}
