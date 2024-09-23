use crate::simulation::components::SimComponentPlugin;
use crate::simulation::input::SimInputPlugin;
use crate::simulation::loading::LoadingPlugin;
use crate::simulation::render::SimRenderPlugin;
use crate::simulation::ui::InterfacePlugin;
use bevy::app::{App, Plugin};
use bevy::prelude::{AppExtStates, States, SubStates, *};

pub mod input;
pub mod render;
pub mod ui;
pub mod components;
pub mod loading;

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, Resource)]
pub enum SimStateType {
    #[default]
    None,
    Simulation,
    Editor
}

#[derive(States, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub enum SimState {
    #[default]
    Setup,
    Menu,
    ScenarioSelection,
    Loading,
    Loaded,
    Reset,
    ExitToMainMenu
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<SimStateType>()
            .init_state::<SimState>()
            .add_plugins(SimInputPlugin)
            .add_plugins(SimRenderPlugin)
            .add_plugins(InterfacePlugin)
            .add_plugins(SimComponentPlugin)
            .add_plugins(LoadingPlugin);
    }

}