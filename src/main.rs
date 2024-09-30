#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::{App, PluginGroup};
use bevy::asset::io::AssetSourceBuilder;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{default, AppExtStates, AssetApp, States, SubStates};
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::{PresentMode, Window, WindowPlugin};
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;
use reqwest::blocking::Client;
use crate::menu::MenuPlugin;
use crate::setup::SetupPlugin;
use crate::simulation::asset::SCENARIO_ASSET_SOURCE;
use crate::simulation::asset::serialization::SerializationPlugin;
use crate::simulation::components::editor::EditorPlugin;
use crate::simulation::SimulationPlugin;

mod constants;
mod setup;
mod menu;
mod simulation;
mod utils;

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
        .register_asset_source(
            SCENARIO_ASSET_SOURCE,
            AssetSourceBuilder::platform_default(SCENARIO_ASSET_SOURCE, None),
        )
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
    //    .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(EditorPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(SerializationPlugin)
        .add_plugins(BillboardPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}