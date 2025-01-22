use crate::simulation::scenario::setup::camera::setup_camera;
pub use crate::simulation::scenario::setup::scenario::{setup_scenario, ScenarioData};
use crate::simulation::SimState;
use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, Startup, Update};

pub mod camera;
pub mod scenario;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ScenarioData>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, setup_scenario.run_if(in_state(SimState::Loading)));
    }
}
