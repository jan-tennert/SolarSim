pub const G: f64 = 6.67430e-11_f64; //gravitational constant
pub const DEF_M_TO_UNIT: f64 = 0.0000001;
pub const M_TO_AU: f32 = 6.684587e-12_f32;

pub const HOUR_IN_SECONDS: f32 = 60.0 * 60.0;
pub const DAY_IN_SECONDS: f32 = HOUR_IN_SECONDS * 24.0;

pub const DEFAULT_TIMESTEP: f64 = 60.0 * 15.0; //15mins
pub const DEFAULT_SUB_STEPS: i32 = 4 * 24; //DEFAULT_TIMESTEP * DEFAULT_SUB_STEPS = 1 day/s