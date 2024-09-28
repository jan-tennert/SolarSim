use bevy::math::DVec3;
use crate::simulation::components::scale::SimulationScale;

pub fn scale_lumen(lumen: f32, scale: &SimulationScale) -> f32 {
    scale.squared().0 * lumen
}

pub fn unscale_lumen(lumen: f32, scale: &SimulationScale) -> f32 {
    lumen / scale.squared().0
}

pub fn scale_lumen_64(lumen: f64, scale: &SimulationScale) -> f64 {
    scale.squared().0 as f64 * lumen
}

pub fn km_to_m_dvec(km: DVec3) -> DVec3 {
    km * 1000.0
}

pub fn m_to_km_dvec(m: DVec3) -> DVec3 {
    m / 1000.0
}

pub fn km_to_m_f64(km: f64) -> f64 {
    km * 1000.0
}

pub fn m_to_km_f64(m: f64) -> f64 {
    m / 1000.0
}

pub fn km_to_m(km: f32) -> f32 {
    km * 1000.0
}

pub fn m_to_km(m: f32) -> f32 {
    m / 1000.0
}