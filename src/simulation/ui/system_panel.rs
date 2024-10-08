use crate::simulation::components::billboard::BillboardSettings;
use crate::simulation::components::body::{BodyChildren, Moon, Planet, Star};
use crate::simulation::components::editor::{CreateBodyState, CreateBodyType, EditorSystemType, EditorSystems};
use crate::simulation::components::motion_line::OrbitOffset;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::render::skybox::Cubemap;
use crate::simulation::ui::metadata::MetadataUiState;
use crate::simulation::ui::UiState;
use crate::simulation::{SimState, SimStateType};
use bevy::core::Name;
use bevy::core_pipeline::Skybox;
use bevy::ecs::system::SystemParam;
use bevy::input::ButtonInput;
use bevy::prelude::{AabbGizmoConfigGroup, Camera, Commands, Entity, GizmoConfigStore, KeyCode, NextState, Query, Res, ResMut, Visibility, With, Without};
use bevy_egui::egui::{Align, Layout, ScrollArea, Ui};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

#[derive(SystemParam)]
pub struct SystemPanelSet<'w, 's> {
    egui_context: EguiContexts<'w, 's>,
    star_query: Query<'w, 's, (
        &'static Name,
        &'static BodyChildren,
        Entity,
        &'static mut Visibility
    ), (
        With<Star>,
        Without<Planet>,
        Without<Planet>
    )>,
    planet_query: Query<'w, 's, (
        &'static Name,
        &'static BodyChildren,
        Entity,
        &'static mut Visibility
    ),(
        With<Planet>,
        Without<Star>,
        Without<Moon>
    )>,
    moon_query: Query<'w, 's, (
        &'static Name,
        Entity,
        &'static mut Visibility
    ),(
        With<Moon>,
        Without<Planet>,
        Without<Star>
    )>,
    //  mut camera: Query<&mut Camera>,
    state: ResMut<'w, NextState<SimState>>,
    selected_entity: ResMut<'w, SelectedEntity>,
    config: ResMut<'w, GizmoConfigStore>,
    camera: Query<'w, 's, (Entity, &'static mut Camera, &'static mut PanOrbitCamera, Option<&'static Skybox>)>,
    commands: Commands<'w, 's>,
    cubemap: ResMut<'w, Cubemap>,
    billboard: ResMut<'w, BillboardSettings>,
    ui_state: ResMut<'w, UiState>,
    orbit_offset: ResMut<'w, OrbitOffset>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    sim_state_type: Res<'w, SimStateType>,
    create_body_state: ResMut<'w, CreateBodyState>,
    systems: Res<'w, EditorSystems>,
    show_metadata: ResMut<'w, MetadataUiState>,
}

fn body_tree<R>(
    entity: Entity,
    ui: &mut Ui,
    mut selected: &mut bool,
    name: &Name,
    default_open: bool,
    add_body: impl FnOnce(&mut Ui) -> R,
    show_button: bool,
    body_type: CreateBodyType
) -> Option<CreateBodyState> {
    let id = ui.make_persistent_id(name.as_str());
    let mut new_create_body = None;
    egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, default_open)
        .show_header(ui, |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut selected, name.as_str());
                if show_button && ui.button("+").on_hover_text("Create child").clicked() {
                    new_create_body = Some(CreateBodyState {
                        parent: Some(entity),
                        body_type,
                    });
                }
            });
        })
        .body(add_body);
    new_create_body
}

pub fn system_panel(
    mut system_panel_set: SystemPanelSet,
    //  systems: Res<EditorSystems>,
    //  editor_systems: Res<EditorSystems>
) {
    let egui_context = &mut system_panel_set.egui_context;
    let ui_state = &mut system_panel_set.ui_state;
    if !ui_state.visible || egui_context.try_ctx_mut().is_none() {
        return;
    }
    let show_button = *system_panel_set.sim_state_type == SimStateType::Editor;
    if let Ok((entity, mut camera, mut pan, skybox)) = system_panel_set.camera.get_single_mut() {
        let ctrl_hold = system_panel_set.keys.pressed(KeyCode::ControlLeft);
        egui::SidePanel::left("system_panel")
            // .default_width(250.0)
            .resizable(true)
            .show(egui_context.ctx_mut(), |ui| {
                ScrollArea::vertical()
                    .auto_shrink(true)
                    .show(ui, |ui| {
                        ui.heading("Bodies");
                        for (s_name, s_children, s_entity, _) in &mut system_panel_set.star_query {
                            let s_old_selected = system_panel_set.selected_entity.entity == Some(s_entity);
                            let mut s_selected = s_old_selected;
                            let new_create_body = body_tree(s_entity, ui, &mut s_selected, s_name, true, |ui| {
                                for planet_child in &s_children.0 {
                                    if let Ok((p_name, p_children, p_entity, _)) = system_panel_set.planet_query.get_mut(*planet_child) {
                                        let p_old_selected = system_panel_set.selected_entity.entity == Some(p_entity);
                                        let mut p_selected = p_old_selected;
                                        let new_create_body = body_tree(p_entity, ui, &mut p_selected, p_name, false, |ui| {
                                            for moon_child in &p_children.0 {
                                                if let Ok((m_name, m_entity, _)) = system_panel_set.moon_query.get_mut(*moon_child) {
                                                    let m_old_selected = system_panel_set.selected_entity.entity == Some(m_entity);
                                                    let mut m_selected = m_old_selected;
                                                    ui.horizontal(|ui| {
                                                        ui.toggle_value(&mut m_selected, m_name.as_str());
                                                    });
                                                    if m_selected && !m_old_selected {
                                                        system_panel_set.selected_entity.change_entity(m_entity, ctrl_hold);
                                                    }
                                                }
                                            }
                                        }, show_button, CreateBodyType::Moon);
                                        if p_selected && !p_old_selected {
                                            system_panel_set.selected_entity.change_entity(p_entity, ctrl_hold)
                                        }
                                        if new_create_body.is_some() {
                                            *system_panel_set.create_body_state = new_create_body.unwrap();
                                            system_panel_set.commands.run_system(system_panel_set.systems.0[EditorSystemType::CREATE_BODY]);
                                        }
                                    }
                                }
                            }, show_button, CreateBodyType::Planet);
                            if s_selected && !s_old_selected {
                                system_panel_set.selected_entity.change_entity(s_entity, ctrl_hold)
                            }
                            if new_create_body.is_some() {
                                *system_panel_set.create_body_state = new_create_body.unwrap();
                                system_panel_set.commands.run_system(system_panel_set.systems.0[EditorSystemType::CREATE_BODY]);
                            }
                        }
                        if ui.button("+").on_hover_text("Create new major body").clicked() {
                            *system_panel_set.create_body_state = CreateBodyState {
                                parent: None,
                                body_type: CreateBodyType::Star,
                            };
                            system_panel_set.commands.run_system(system_panel_set.systems.0[EditorSystemType::CREATE_BODY]);
                        }
                        ui.separator();
                        ui.heading("Options");
                        ui.checkbox(&mut camera.hdr, "HDR/Bloom");
                        let skybox_enabled = skybox.is_some();
                        let mut skybox_setting = skybox_enabled;
                        ui.checkbox(&mut skybox_setting, "Milky Way Skybox");

                        if skybox_enabled && !skybox_setting {
                            system_panel_set.commands.entity(entity).remove::<Skybox>();
                            system_panel_set.cubemap.activated = false;
                        } else if !skybox_enabled && skybox_setting {
                            system_panel_set.commands.entity(entity).insert(Skybox { image: system_panel_set.cubemap.image_handle.clone(), brightness: 1000.0 });
                            system_panel_set.cubemap.activated = true;
                        }

                        ui.checkbox(&mut system_panel_set.config.config_mut::<AabbGizmoConfigGroup>().1.draw_all, "Draw Outlines");
                        ui.checkbox(&mut system_panel_set.billboard.show, "Show body names");
                        ui.checkbox(&mut system_panel_set.billboard.dynamic_hide, "Dynamically hide body names");
                        if *system_panel_set.sim_state_type == SimStateType::Simulation {
                            ui.checkbox(&mut ui_state.dyn_hide_orbit_lines, "Dynamically hide orbit lines");
                        }
                        if ui.button("Open Debug Window").clicked() {
                            ui_state.show_debug = true;
                        }
                        ui.add_space(5.0);
                        if ui.button("Open Keybind Window").clicked() {
                            ui_state.show_keys = true;
                        }
                        ui.add_space(5.0);
                        if *system_panel_set.sim_state_type == SimStateType::Editor && ui.button("Edit metadata").clicked() {
                            system_panel_set.show_metadata.show = true;
                        }
                        ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                            if ui.button("Back to Menu").clicked() {
                                system_panel_set.state.set(SimState::ExitToMainMenu);
                            }
                            ui.separator();
                        });
                    });
            });
    }

}