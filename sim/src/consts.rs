// Physical constants
pub const G: f32 = 1.;

// Heat equation consts
pub const DX: f32 = 1.;
pub const DY: f32 = 1.;
pub const KAPPA: f32 = 10.;
const CFL: f32 = 0.3;
pub const SIM_DT: f32 = CFL * DX * DX / (KAPPA * 2.0);

// Simulations consts
pub const SPEEDUP: u32 = 2;

// Projection consts
pub const HEIGHT: usize = 51;
pub const WIDTH: usize = 161; // Should be about PI times larger than HEIGHT!
