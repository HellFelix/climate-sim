use std::f32::consts::PI;

// Physical constants
pub const G: f32 = 1.;

// Heat equation consts
pub const DX: f32 = 1.;
pub const DY: f32 = 1.;
pub const DTHETA: f32 = 2. * PI * DX as f32 / WIDTH as f32;
pub const DPHI: f32 = PI * DY as f32 / HEIGHT as f32;
pub const KAPPA: f32 = 10.;
const CFL: f32 = 0.01;
pub const SIM_DT: f32 = CFL * DPHI * DTHETA / (KAPPA * 2.0);

// Simulations consts
pub const SPEEDUP: u32 = 10;

// Projection consts
pub const HEIGHT: usize = 51;
pub const WIDTH: usize = 161; // Should be about PI times larger than HEIGHT!
pub const HEIGHT_AS_F32: f32 = 51.;
pub const WIDTH_AS_F32: f32 = 161.;

// radius of the world used as a scaling factor in calculations.
pub fn world_r() -> f32 {
    (HEIGHT_AS_F32.powi(2) + WIDTH_AS_F32.powi(2)).sqrt()
}
