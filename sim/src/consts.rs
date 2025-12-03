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

// Physics consts
pub const RHO: f32 = 0.05;
pub const R: f32 = 0.08;

pub const SOLAR_CONSTANT: f32 = 0.1; // 1.3608;
pub const OMEGA: f32 = 0.98; // spridning/(spridning + absorbtion)
pub const TAU: f32 = 0.3; // Optical Depth
pub const M_EFF: f32 = 1.7; // Airmass
pub const C_DIFF: f32 = OMEGA * TAU * M_EFF / 2.;
