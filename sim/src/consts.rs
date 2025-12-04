use std::f32::consts::PI;

// Simulation consts
pub const UNIVERSAL_UPDATE_RATE: f64 = 0.01;
pub const SPEEDUP: u32 = 10;

// Control consts
pub const TRANSLATION_SPEED: f32 = 0.1;
pub const ROTATION_SPEED: f32 = 0.02;

// Planet consts
pub const PERIOD_TIME: f32 = 10.;
pub const N: f32 = 2. * PI / PERIOD_TIME;
pub const PER_TIME: f32 = 0.;
pub const E: f32 = 0.16;
pub const PLANET_DT: f32 = UNIVERSAL_UPDATE_RATE as f32;
pub const A: f32 = 10.;

// Heat equation consts
pub const DX: f32 = 1.;
pub const DY: f32 = 1.;
pub const DTHETA: f32 = 2. * PI * DX as f32 / WIDTH as f32;
pub const DPHI: f32 = PI * DY as f32 / HEIGHT as f32;
pub const KAPPA: f32 = 10.;
const CFL: f32 = 0.01;
pub const DIFFUSION_DT: f32 = CFL * DPHI * DTHETA / (KAPPA * 2.0);

// Projection consts
pub const HEIGHT: usize = 51;
pub const WIDTH: usize = 161; // Should be about PI times larger than HEIGHT!

// Flux consts
pub const RHO: f32 = 0.05;
pub const R: f32 = 0.08;

pub const SOLAR_CONSTANT: f32 = 0.1; // 1.3608;
pub const OMEGA: f32 = 0.98; // spridning/(spridning + absorbtion)
pub const TAU: f32 = 0.3; // Optical Depth
pub const M_EFF: f32 = 1.7; // Airmass
pub const C_DIFF: f32 = OMEGA * TAU * M_EFF / 2.;

// Black body radiation
pub const EPS: f32 = 1.;
pub const SIGMA: f32 = 1e-7;
pub const DA: f32 = DPHI * DTHETA; // This isn't quite right
pub const C: f32 = 1.;
