use std::f32::consts::PI;

use crate::consts::*;

fn vector_from_cord(x: usize, y: usize) -> [f32; 3] {
    let u = (x as f32 + 0.5) / WIDTH as f32;
    let v = (y as f32 + 0.5) / HEIGHT as f32;

    let lambda: f32 = (u - 0.5) * 2. * PI;
    let phi: f32 = (0.5 - v) * PI;
    let vec = [
        phi.cos() * lambda.cos(),
        phi.cos() * lambda.sin(),
        phi.sin(),
    ];
    return vec;
}

pub fn my_f(point: [f32; 3], zenit: [f32; 3]) -> f32 {
    let my: f32 = point[0] * zenit[0] + point[1] * zenit[1] + point[2] * zenit[2];
    return my;
}

pub fn transmition_f(my: f32) -> f32 {
    let omega: f32 = 0.98;
    let tao: f32 = 0.3;
    let M_eff: f32 = 1.7; //Airmass
    let C_diff: f32 = (omega * tao * M_eff) / 2.;
    return C_diff + (-tao / my).exp();
}

pub fn flux(my: f32) -> f32 {
    let solar_constant: f32 = 1.3608;
    let ro: f32 = 0.05;
    let r: f32 = 0.08;
    return (solar_constant * my * transmition_f(my)) / (1.0 - ro * r);
}

pub fn flux_pp(xy_sun: [f32; 2]) -> Vec<Vec<f32>> {
    let senit: [f32; 3] = vector_from_cord(xy_sun[0] as usize, xy_sun[1] as usize);
    let mut heat_matrix = vec![vec![0.0_f32; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut point: [f32; 3] = vector_from_cord(0, 0);

            point = vector_from_cord(x, y);

            let my: f32 = my_f(point, senit);
            let flux_pp: f32 = flux(my);
            heat_matrix[y][x] = flux_pp;
        }
    }
    return heat_matrix;
}
