use crate::simulation::components::billboard::BillboardSettings;
use crate::simulation::components::body::{BodyChildren, Moon, Planet, Star};
use crate::simulation::components::camera::PanOrbitCamera;
use crate::simulation::components::orbit_lines::OrbitOffset;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::render::skybox::Cubemap;
use crate::SimState;
use bevy::core::Name;
use bevy::core_pipeline::Skybox;
use bevy::input::ButtonInput;
use bevy::prelude::{AabbGizmoConfigGroup, Camera, Commands, Entity, GizmoConfigStore, KeyCode, NextState, Query, Res, ResMut, Vec3, Visibility, With, Without};
use bevy_egui::egui::{InnerResponse, Response, ScrollArea, Ui};
use bevy_egui::{egui, EguiContexts};
use crate::simulation::ui::UiState;

fn body_tree<R>(
    ui: &mut Ui,
    mut selected: &mut bool,
    name: &Name,
    default_open: bool,
    add_body: impl FnOnce(&mut Ui) -> R,
) -> (
    Response,
    InnerResponse<()>,
    Option<InnerResponse<R>>,
) {
    let id = ui.make_persistent_id(name.as_str());
    egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, default_open)
        .show_header(ui, |ui| {
            ui.toggle_value(&mut selected, name.as_str());
        })
        .body(add_body)
}

pub fn system_panel(
    mut egui_context: EguiContexts,
    mut star_query: Query<(
        &Name,
        &BodyChildren,
        Entity,
        &mut Visibility
    ), (
        With<Star>,
        Without<Planet>,
        Without<Planet>
    )>,
    mut planet_query: Query<(
        &Name,
        &BodyChildren,
        Entity,
        &mut Visibility
    ),(
        With<Planet>,
        Without<Star>,
        Without<Moon>
    )>,
    mut moon_query: Query<(
        &Name,
        Entity,
        &mut Visibility
    ),(
        With<Moon>,
        Without<Planet>,
        Without<Star>
    )>,
    //  mut camera: Query<&mut Camera>,
    mut state: ResMut<NextState<SimState>>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut config: ResMut<GizmoConfigStore>,
    mut camera: Query<(Entity, &mut Camera, &mut PanOrbitCamera, Option<&Skybox>)>,
    mut commands: Commands,
    mut cubemap: ResMut<Cubemap>,
    mut billboard: ResMut<BillboardSettings>,
    mut ui_state: ResMut<UiState>,
    mut orbit_offset: ResMut<OrbitOffset>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !ui_state.visible || egui_context.try_ctx_mut().is_none() {
        return;
    }
    if let Ok((entity, mut camera, mut pan, skybox)) = camera.get_single_mut() {
        let ctrl_hold = keys.pressed(KeyCode::ControlLeft);
        egui::SidePanel::left("system_panel")
            // .default_width(250.0)
            .resizable(true)
            .show(egui_context.ctx_mut(), |ui| {
                ScrollArea::vertical()
                    .auto_shrink(true)
                    .show(ui, |ui| {
                        ui.heading("Bodies");
                        for (s_name, s_children, s_entity, _) in &mut star_query {
                            let s_old_selected = selected_entity.entity == Some(s_entity);
                            let mut s_selected = s_old_selected;
                            body_tree(ui, &mut s_selected, s_name, true, |ui| {
                                for planet_child in &s_children.0 {
                                    if let Ok((p_name, p_children, p_entity, _)) = planet_query.get_mut(*planet_child) {
                                        let p_old_selected = selected_entity.entity == Some(p_entity);
                                        let mut p_selected = p_old_selected;
                                        body_tree(ui, &mut p_selected, p_name, false, |ui| {
                                            for moon_child in &p_children.0 {
                                                if let Ok((m_name, m_entity, _)) = moon_query.get_mut(*moon_child) {
                                                    let m_old_selected = selected_entity.entity == Some(m_entity);
                                                    let mut m_selected = m_old_selected;
                                                    ui.horizontal(|ui| {
                                                        ui.toggle_value(&mut m_selected, m_name.as_str());
                                                    });
                                                    if m_selected && !m_old_selected {
                                                        selected_entity.change_entity(m_entity, ctrl_hold)
                                                    }
                                                }
                                            }
                                        });
                                        if p_selected && !p_old_selected {
                                            selected_entity.change_entity(p_entity, ctrl_hold)
                                        }
                                    }
                                }
                            });
                            if s_selected && !s_old_selected {
                                selected_entity.change_entity(s_entity, ctrl_hold)
                            }
                        }
                        ui.heading("Options");
                        ui.checkbox(&mut camera.hdr, "HDR/Bloom");
                        let skybox_enabled = skybox.is_some();
                        let mut skybox_setting = skybox_enabled;
                        ui.checkbox(&mut skybox_setting, "Milky Way Skybox");

                        if skybox_enabled && !skybox_setting {
                            commands.entity(entity).remove::<Skybox>();
                            cubemap.activated = false;
                        } else if !skybox_enabled && skybox_setting {
                            commands.entity(entity).insert(Skybox { image: cubemap.image_handle.clone(), brightness: 1000.0 });
                            cubemap.activated = true;
                        }

                        ui.checkbox(&mut config.config_mut::<AabbGizmoConfigGroup>().1.draw_all, "Draw Outlines");
                        ui.checkbox(&mut billboard.show, "Show Body Names");
                        if ui.checkbox(&mut orbit_offset.enabled, "Offset body to zero").changed() {
                            if orbit_offset.enabled {
                                pan.focus = Vec3::ZERO;
                            }
                        }
                        ui.checkbox(&mut ui_state.dyn_hide_orbit_lines, "Dynamically hide orbit lines");
                        if ui.button("Open Debug Window").clicked() {
                            ui_state.show_debug = true;
                        }
                        ui.add_space(5.0);
                        if ui.button("Open Keybind Window").clicked() {
                            ui_state.show_keys = true;
                        }
                        ui.separator();
                        if ui.button("Back to Menu").clicked() {
                            let _ = state.set(SimState::ExitToMainMenu);
                        }
                    });
            });
    }
}