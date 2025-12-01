use bevy::prelude::*;
use ndarray::{Array2, arr2};

use crate::consts::{DX, DY, HEIGHT, KAPPA, WIDTH};

pub fn heat_eq_step(T0: &Array2<f32>, h: f32) -> Array2<f32> {
    system_rk4_step(
        |T| {
            let mut res = arr2(&[[0.; HEIGHT]; WIDTH]);
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    res[[x, y]] = (T[[(x + 1) % WIDTH, y]] - 2. * T[[x, y]]
                        + T[[(x + WIDTH - 1) % WIDTH, y]])
                        / DX.powi(2)
                        + (T[[x, if y == HEIGHT - 1 { y } else { y + 1 }]] - 2. * T[[x, y]]
                            + T[[x, if y == 0 { 0 } else { y - 1 }]])
                            / DY.powi(2);
                }
            }
            KAPPA * res
        },
        T0,
        h,
    )
}

fn system_rk4_step(F: fn(&Array2<f32>) -> Array2<f32>, T0: &Array2<f32>, h: f32) -> Array2<f32> {
    let k1 = F(T0);
    let k2 = F(&(T0 + (h / 2.) * &k1));
    let k3 = F(&(T0 + (h / 2.) * &k2));
    let k4 = F(&(T0 + h * &k3));

    return T0 + (h / 6.) * (k1 + k2 + k3 + k4);
}
