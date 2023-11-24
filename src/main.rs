#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use apsis::ApsisPlugin;
use bevy::app::{App, PluginGroup};
use bevy::DefaultPlugins;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{default, States, NonSend, Query, Entity, Startup};
use bevy::render::RenderPlugin;
use bevy::render::settings::{RenderCreation, WgpuSettings, Backends};
use bevy::window::{PresentMode, Window, WindowPlugin};
use bevy::winit::WinitWindows;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;

use camera::PanOrbitCameraPlugin;
use debug::DebugPlugin;
use diameter::DiameterPlugin;
use input::InputPlugin;
use loading::LoadingPlugin;
use lock_on::LockOnPlugin;
use orbit_lines::OrbitLinePlugin;
use reset::ResetPlugin;
use rotation::RotationPlugin;
use serialization::SerializationPlugin;
use skybox::SkyboxPlugin;
use speed::SpeedPlugin;
use star_renderer::StarRendererPlugin;
use ui::UIPlugin;
use winit::window::Icon;

use crate::billboard::BodyBillboardPlugin;
use crate::menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::selection::SelectionPlugin;
use crate::setup::SetupPlugin;

mod body;
mod constants;
mod setup;
mod physics;
mod egui_input_block;
mod speed;
mod selection;
mod menu;
mod skybox;
mod diameter;
mod ui;
mod orbit_lines;
mod reset;
mod rotation;
mod serialization;
mod lock_on;
mod input;
mod camera;
mod loading;
mod star_renderer;
mod billboard;
mod apsis;
mod unit;
mod debug;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SimState {
    #[default]
    Menu,
    Loading,
    Simulation,
    Reset,
    ExitToMainMenu
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: Query<(Entity, &Window)>,
    w_windows: NonSend<WinitWindows>,
) {
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
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
            })  
        )
        .add_plugins(EguiPlugin)
     //   .add_plugins(WorldInspectorPlugin::new())
  //      .add_plugins(DefaultPickingPlugins)
        .add_plugins(LockOnPlugin)
        .add_plugins(SerializationPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(BodyBillboardPlugin)
        .add_plugins(BillboardPlugin)
        .add_plugins(ApsisPlugin)
        .add_plugins(DebugPlugin)
        //     .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(SelectionPlugin)
        .add_plugins(SkyboxPlugin)
        .add_plugins(StarRendererPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(SpeedPlugin)
        .add_plugins(ResetPlugin)
        .add_plugins(OrbitLinePlugin)
        .add_plugins(RotationPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DiameterPlugin)
    //    .add_plugins(ScreenDiagnosticsPlugin::default())
  //      .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_state::<SimState>()
        .add_systems(Startup, set_window_icon)
        .run();
}