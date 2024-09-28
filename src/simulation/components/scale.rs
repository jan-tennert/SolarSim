use bevy::app::Plugin;
use bevy::math::DVec3;
use bevy::prelude::{Resource, Vec3};

pub struct ScalePlugin;

impl Plugin for ScalePlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<SimulationScale>();
    }

}

#[derive(Resource)]
pub struct SimulationScale(pub f32);

impl Default for SimulationScale {

    fn default() -> Self {
        SimulationScale(0.0000001)
    }

}

impl SimulationScale {

    pub fn f64(&self) -> f64 {
        self.0 as f64
    }

    pub fn squared(&self) -> SimulationScale {
        SimulationScale(self.0 * self.0)
    }

    pub fn m_to_unit(&self, value: f64) -> f64 {
        value * self.0 as f64
    }

    pub fn unit_to_m(&self, value: f64) -> f64 {
        value / self.0 as f64
    }

    pub fn m_to_unit_32(&self, value: f32) -> f32 {
        value * self.0
    }

    pub fn unit_to_m_32(&self, value: f32) -> f32 {
        value / self.0
    }

    pub fn m_to_unit_vec(&self, value: Vec3) -> Vec3 {
        value * self.0
    }

    pub fn unit_to_m_vec(&self, value: Vec3) -> Vec3 {
        value / self.0
    }

    pub fn m_to_unit_dvec(&self, value: DVec3) -> DVec3 {
        value * self.0 as f64
    }

    pub fn unit_to_m_dvec(&self, value: DVec3) -> DVec3 {
        value / self.0 as f64
    }

}