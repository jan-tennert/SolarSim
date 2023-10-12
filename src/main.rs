mod body;
mod constants;
mod setup;
mod bodies;
mod physics;
mod input;
mod speed;
mod fps;
mod selection;
mod menu;
mod skybox;
mod diameter;
mod pan_orbit;
mod ui;
mod orbit_lines;

use bevy::app::{App, PluginGroup};
use bevy::DefaultPlugins;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{default, States};
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::window::{WindowPlugin, Window, PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use orbit_lines::OrbitLinePlugin;
use pan_orbit::lib::PanOrbitCameraPlugin;
use skybox::SkyboxPlugin;
use speed::SpeedPlugin;
use ui::UIPlugin;
use crate::menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::selection::SelectionPlugin;
use crate::setup::SetupPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SimState {
    #[default]
    Menu,
    Simulation,
    Reset,
    ExitToMainMenu
}

fn main() {
    App::new()
     //   .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins) 
        .add_plugins(WorldInspectorPlugin::new())
  //      .add_plugins(DefaultPickingPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SelectionPlugin)
        .add_plugins(SkyboxPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(SpeedPlugin)
        .add_plugins(OrbitLinePlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
      //  .add_plugins(DiameterPlugin)
    //    .add_plugins(ScreenDiagnosticsPlugin::default())
  //      .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_state::<SimState>()
        .run();
}