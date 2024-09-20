#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::{App, PluginGroup};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{default, AppExtStates, States, SubStates};
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::{PresentMode, Window, WindowPlugin};
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;

use loading::LoadingPlugin;
use serialization::SerializationPlugin;

use crate::menu::MenuPlugin;
use crate::setup::SetupPlugin;
use crate::simulation::components::SimComponentPlugin;
use crate::simulation::input::SimInputPlugin;
use crate::simulation::render::SimRenderPlugin;
use crate::simulation::ui::InterfacePlugin;

mod constants;
mod setup;
mod menu;
mod serialization;
mod loading;
mod unit;
mod simulation;

#[derive(States, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub enum SimState {
    #[default]
    Setup,
    Menu,
    Loading,
    Simulation,
    Reset,
    ExitToMainMenu
}

/**
fn set_window_icon(
    // we have to use `NonSend` here
    windows: Query<(Entity, &Window)>,
    w_windows: NonSend<WinitWindows>,
) {
    if cfg!(windows) {
        if let Ok((id,_)) = windows.get_single() {
            let window = w_windows.get_window(id).unwrap();
            // here we use the `image` crate to load our icon data from a png file
            // this is not a very bevy-native solution, but it will do
            let (icon_rgba, icon_width, icon_height) = {
                let image = image::open("assets/images/icon.png")
                    .expect("Failed to open icon path")
                    .into_rgba8();
                let (width, height) = image.dimensions();
                let rgba = image.into_raw();
                (rgba, width, height)
            };
        
            let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
        
            window.set_window_icon(Some(icon));
        }
    }
}
**/

fn main() {
    App::new()
     //   .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Solar System Simulation (Jan Tennert)".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
              //      backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            })  
        )
        .add_plugins(EguiPlugin)
     //   .add_plugins(WorldInspectorPlugin::new())
  //      .add_plugins(DefaultPickingPlugins)
        .add_plugins(SerializationPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(BillboardPlugin)
        //     .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SimInputPlugin)
        .add_plugins(SimRenderPlugin)
        .add_plugins(InterfacePlugin)
        .add_plugins(SimComponentPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
    //    .add_plugins(ScreenDiagnosticsPlugin::default())
  //      .add_plugins(ScreenFrameDiagnosticsPlugin)
        .init_state::<SimState>()
       // .add_systems(Startup, set_window_icon)
        .run();
}