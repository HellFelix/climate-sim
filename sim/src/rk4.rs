#![allow(non_snake_case)]

use std::f32::consts::PI;

use ndarray::{Array2, arr2};

use crate::consts::*;

// Head eq for cartesian coordinates
// pub fn heat_eq_step(T0: &Array2<f32>, h: f32) -> Array2<f32> {
//     system_rk4_step(
//         |T| {
//             let mut res = arr2(&[[0.; HEIGHT]; WIDTH]);
//             for x in 0..WIDTH {
//                 for y in 0..HEIGHT {
//                     res[[x, y]] = (T[[(x + 1) % WIDTH, y]] - 2. * T[[x, y]]
//                         + T[[(x + WIDTH - 1) % WIDTH, y]])
//                         / DX.powi(2)
//                         + (T[[x, if y == HEIGHT - 1 { y } else { y + 1 }]] - 2. * T[[x, y]]
//                             + T[[x, if y == 0 { 0 } else { y - 1 }]])
//                             / DY.powi(2);
//                 }
//             }
//             KAPPA * res
//         },
//         T0,
//         h,
//     )
// }

const R: f32 = 1.;

pub fn heat_eq_step_spherical(T0: &Array2<f32>, h: f32) -> Array2<f32> {
    system_rk4_step(
        |T| {
            let mut res = arr2(&[[0.; HEIGHT]; WIDTH]);
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let _phi = 2. * PI * x as f32 / WIDTH as f32;
                    let theta = PI * y as f32 / HEIGHT as f32;

                    let theta_plus_half = theta + DTHETA / 2.;
                    let theta_minus_half = theta - DTHETA / 2.;

                    let (is_upper, theta_break) = if y < HEIGHT / 2 {
                        (true, theta_plus_half)
                    } else {
                        (false, theta_minus_half)
                    };

                    // info!("First is {}", (theta.sin() * DTHETA.powi(2)));
                    // info!("Second is {}", theta.sin().powi(2));
                    // info!("Third is {}", DPHI.powi(2));

                    res[[x, y]] = (1. / R.powi(2))
                        * ((theta_break.cos() / theta_break.sin())
                            * (T[[x, if is_upper { y + 1 } else { y }]]
                                - T[[x, if is_upper { y } else { y - 1 }]])
                            / DTHETA
                            + (T[[x, if y == 0 { y } else { y - 1 }]] - 2. * T[[x, y]]
                                + T[[x, if y == HEIGHT - 1 { y } else { y + 1 }]])
                                / DTHETA.powi(2)
                            + theta_break.powi(-2)
                                * (T[[if x == 0 { WIDTH - 1 } else { x - 1 }, y]]
                                    - 2. * T[[x, y]]
                                    + T[[if x == WIDTH - 1 { 0 } else { x + 1 }, y]])
                                / DPHI.powi(2));
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

    T0 + (h / 6.) * (k1 + k2 + k3 + k4)
}
