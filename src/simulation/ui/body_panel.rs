use bevy::asset::AssetServer;
use crate::simulation::components::apsis::ApsisBody;
use crate::simulation::components::body::{AxialTilt, BodyChildren, BodyParent, Diameter, Mass, ModelPath, OrbitSettings, RotationSpeed, Scale, SceneEntity, SceneHandle, SimPosition, Velocity};
use crate::constants::{G, M_TO_AU, M_TO_UNIT};
use crate::simulation::components::selection::SelectedEntity;
use crate::unit::{format_length, format_seconds};
use bevy::core::Name;
use bevy::math::DVec3;
use bevy::prelude::{BuildChildren, Camera, Commands, DespawnRecursiveExt, Entity, Handle, Mut, Query, Res, ResMut, Resource, Scene, Srgba, Transform, Vec3, With, Without};
use bevy_egui::egui::{Align, Layout, RichText, ScrollArea};
use bevy_egui::{egui, EguiContexts};
use crate::setup::spawn_scene;
use crate::simulation::components::editor::{EditorSystemType, EditorSystems};
use crate::simulation::ui::components::vector_field;
use crate::simulation::ui::UiState;

#[derive(Debug, Default, Clone, Resource)]
pub struct EditorPanelState {
    pub entity: Option<Entity>,
    pub new_name: String,
    pub new_position: DVec3,
    pub new_velocity: DVec3,
    pub new_mass: f64,
    pub new_diameter: f32,
    pub new_rotation_speed: f64,
    pub new_axial_tilt: f32,
    pub new_model_path: String,
    pub show_delete_confirm: bool,
}

impl EditorPanelState {

    pub fn new(entity: Entity, name: String, position: DVec3, velocity: DVec3, mass: f64, diameter: f32, rotation_speed: f64, axial_tilt: f32, model_path: String) -> Self {
        EditorPanelState {
            entity: Some(entity),
            new_name: name,
            new_position: position,
            new_velocity: velocity,
            new_mass: mass,
            new_diameter: diameter,
            new_rotation_speed: rotation_speed,
            new_axial_tilt: axial_tilt,
            new_model_path: model_path,
            show_delete_confirm: false,
        }
    }

}

pub fn editor_body_panel(
    mut egui_context: EguiContexts,
    selected_entity: Res<SelectedEntity>,
    mut query: Query<(Entity, &mut Name, &mut SimPosition, &mut Velocity, &mut Mass, &mut Diameter, &mut RotationSpeed, &mut AxialTilt, &mut ModelPath, &mut SceneHandle), With<Mass>>,
    mut scene_query: Query<Entity, With<SceneEntity>>,
    mut state: ResMut<EditorPanelState>,
    mut commands: Commands,
    systems: Res<EditorSystems>,
    assets: Res<AssetServer>,
) {
    if egui_context.try_ctx_mut().is_none() {
        return;
    }
    if let Some(s_entity) = selected_entity.entity {
        if let Ok((entity, mut name, mut pos, mut vel, mut mass, mut diameter, mut rotation_speed, mut tilt, mut model_path, mut scene)) = query.get_mut(s_entity) {
            if(state.entity.is_none() || state.entity.unwrap() != s_entity) {
                *state = EditorPanelState::new(s_entity, name.to_string(), pos.0, vel.0, mass.0, (diameter.num / 1000.0) / M_TO_UNIT as f32, rotation_speed.0, tilt.num, model_path.cleaned());
            }
            egui::SidePanel::right("body_panel")
                //      .max_width(250.0)
                .resizable(true)
                .show(egui_context.ctx_mut(), |ui| {
                    ScrollArea::vertical()
                        .auto_shrink(true)
                        .show(ui, |ui| {
                            ui.heading("Body");
                            ui.horizontal(|ui| {
                                ui.label("Name");
                                ui.text_edit_singleline(&mut state.new_name);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Model Path");
                                ui.text_edit_singleline(&mut state.new_model_path);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Mass (kg)");
                                ui.add(egui::DragValue::new(&mut state.new_mass));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Diameter (km)");
                                ui.add(egui::DragValue::new(&mut state.new_diameter));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Rotation Speed (min/rotation)");
                                ui.add(egui::DragValue::new(&mut state.new_rotation_speed));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Axial Tilt (degrees)");
                                ui.add(egui::DragValue::new(&mut state.new_axial_tilt));
                            });
                            vector_field(ui, "Position (m)", &mut state.new_position);
                            vector_field(ui, "Velocity (m/s)", &mut state.new_velocity);
                            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("Apply").on_hover_text("Apply changes").clicked() {
                                        name.set(state.new_name.clone());
                                        pos.0 = state.new_position;
                                        vel.0 = state.new_velocity;
                                        mass.0 = state.new_mass;
                                        let new_diameter = state.new_diameter * M_TO_UNIT as f32 * 1000.0;
                                        diameter.applied = new_diameter == diameter.num;
                                        diameter.num = new_diameter;
                                        rotation_speed.0 = state.new_rotation_speed;
                                        let new_tilt = state.new_axial_tilt;
                                        tilt.applied = new_tilt == tilt.num;
                                        tilt.num = new_tilt;
                                        if model_path.cleaned() != state.new_model_path {
                                            *model_path = ModelPath::from_cleaned(state.new_model_path.as_str());
                                            let asset_handle: Handle<Scene> = assets.load(model_path.clone().0);
                                            commands.entity(scene_query.get(scene.1).unwrap()).despawn_recursive();
                                            scene.0 = asset_handle.clone();
                                            commands.entity(entity).with_children(|parent| {
                                                scene.1 = spawn_scene(
                                                    asset_handle,
                                                    state.new_name.as_str(),
                                                    parent
                                                );
                                            });

                                            diameter.aabb = None;
                                        }
                                        commands.run_system(systems.0[EditorSystemType::UPDATE_POSITIONS]);
                                        commands.run_system(systems.0[EditorSystemType::UPDATE_DIAMETER]);
                                        commands.run_system(systems.0[EditorSystemType::UPDATE_TILT]);
                                    }
                                    if ui.button("Reset").on_hover_text("Reset to original values").clicked() {
                                        *state = EditorPanelState::new(s_entity, name.to_string(), pos.0, vel.0, mass.0, (diameter.num / 1000.0) / M_TO_UNIT as f32, rotation_speed.0, tilt.num, model_path.cleaned());
                                    }
                                    if state.show_delete_confirm {
                                        ui.label("Are you sure?");
                                        if ui.button("Yes").on_hover_text("Delete selected body").clicked() {
                                            commands.entity(entity).despawn_recursive();
                                            state.show_delete_confirm = false;
                                        }
                                        if ui.button("No").on_hover_text("Cancel deletion").clicked() {
                                            state.show_delete_confirm = false;
                                        }
                                    } else {
                                        if ui.button("Delete").on_hover_text("Delete selected body").clicked() {
                                            state.show_delete_confirm = true;
                                        }
                                    }
                                });
                                ui.separator();
                            });
                        });
                });
        }
    } else {
        state.entity = None;
    }
}

pub fn sim_body_panel(
    mut egui_context: EguiContexts,
    mut commands: Commands,
    mut query: Query<(&Name, Entity, &SimPosition, &mut Velocity, &RotationSpeed, &Diameter, &mut OrbitSettings, &mut Mass, &Scale, &mut Transform, Option<&mut ApsisBody>, Option<&BodyChildren>, Option<&BodyParent>)>,
    camera: Query<(&Camera, &Transform), Without<Velocity>>,
    selected_entity: Res<SelectedEntity>,
    mut ui_state: ResMut<UiState>,
) {
    if !ui_state.visible || egui_context.try_ctx_mut().is_none() {
        return;
    }
    if let Some(entity) = selected_entity.entity {
        let mut parent: Option<(&SimPosition, Mut<Velocity>, &Name, Mass)> = None;
        let mut selected: Option<(&Name, Entity, &SimPosition, Mut<Velocity>, &RotationSpeed, &Diameter, Mut<OrbitSettings>, Mut<Transform>, Mut<Mass>, Option<Mut<ApsisBody>>, &Scale, Option<&BodyChildren>)> = None;
        let mut s_children: Vec<(Entity, Mut<OrbitSettings>)> = vec![];
        for (name, b_entity, pos, mut velocity, rotation_speed, diameter, orbit, mass, scale, transform, apsis, children, maybe_parent) in query.iter_mut() {
            if children.is_some() && children.unwrap().0.contains(&entity) { //check for the parent of the selected entity
                parent = Some((pos, velocity, name, mass.clone()));
            } else {
                if b_entity == entity { //check for the selected entity
                    selected = Some((name, b_entity, pos, velocity, rotation_speed, diameter, orbit, transform, mass, apsis, scale, children));
                } else if let Some(parent_id) = maybe_parent { //check for potential children of the entity
                    if parent_id.0 == entity {
                        s_children.push((b_entity, orbit))
                    }
                }
            }
        }
        if let Some((name, entity, pos, mut velocity, rotation_speed, diameter, mut orbit, mut transform, mut mass, apsis, scale, _)) = selected {
            egui::SidePanel::right("body_panel")
                //      .max_width(250.0)
                .resizable(true)
                .show(egui_context.ctx_mut(), |ui| {
                    ScrollArea::vertical()
                        .auto_shrink(true)
                        .show(ui, |ui| {
                            ui.heading(name.as_str());

                            //Mass block
                            ui.label(RichText::new("Mass").size(16.0).underline());
                            ui.horizontal(|ui| {
                                let f_mass = mass.0 * 10e-24;
                                if ui.label(format!("{:.3} 10^24 kg", f_mass)).clicked() {
                                    ui_state.edit_mass = !ui_state.edit_mass;
                                }
                            });
                            if ui_state.edit_mass {
                                ui.horizontal(|ui| {
                                    if ui.button(":5").clicked() {
                                        mass.0 /= 5.0;
                                    }
                                    if ui.button(":2").clicked() {
                                        mass.0 /= 2.0;
                                    }
                                    if ui.button("x2").clicked() {
                                        mass.0 *= 2.0;
                                    }
                                    if ui.button("x5").clicked() {
                                        mass.0 *= 5.0;
                                    }
                                });
                            }
                            if scale.0 != 0.0 {
                                ui.label(
                                    RichText::new("Body Scale")
                                        .size(16.0)
                                        .underline(),
                                );
                                let mut n_scale = transform.scale.x / scale.0;
                                ui.horizontal(|ui| {
                                    ui.add(
                                        egui::Slider::new(&mut n_scale, 0.001..=100.0)
                                            .clamp_to_range(true)
                                            .logarithmic(true));
                                });
                                transform.scale = Vec3::splat(n_scale * scale.0);
                                ui.label(
                                    RichText::new("Equator Diameter")
                                        .size(16.0)
                                        .underline(),
                                );
                                let scaled_diameter = (diameter.num / M_TO_UNIT as f32) * n_scale;
                                ui.label(format!("{} km", scaled_diameter / 1000.0));
                            }

                            // Velocity Orbit Velocity around parent
                            let actual_velocity = match &parent {
                                Some((_, vel, _, _)) => (vel.0 - velocity.0).length() / 1000.0,
                                None => velocity.0.length() / 1000.0,
                            };
                            let velocity_prefix = if parent.is_some() {
                                "Orbital"
                            } else {
                                "Total"
                            };
                            ui.label(RichText::new(format!("{} Velocity", velocity_prefix)).size(16.0).underline());
                            ui.horizontal(|ui| {
                                ui.label(format!("{:.3} km/s", actual_velocity));
                                if actual_velocity < 10.0 {
                                    ui.label(format!("({:.3} km/h)", actual_velocity * 3600.0));
                                }
                            });

                            let mut new_apsis = None;
                            if let Some((_, _, _, p_mass)) = parent {
                                if let Some(apsis) = apsis {
                                    let distance = ((apsis.aphelion.distance + apsis.perihelion.distance) / 2.0) as f64;
                                    orbit.period = 2.0 * std::f64::consts::PI * f64::sqrt(f64::powf(distance, 3.0) / (G * (p_mass.0 + mass.0)));
                                    ui.label(RichText::new("Orbital Period").size(16.0).underline());
                                    ui.label(format!("{}", format_seconds(orbit.period)));
                                    new_apsis = Some(apsis);
                                }
                            }

                            ui.label(RichText::new("Rotation Period").size(16.0).underline());
                            ui.label(format!("{}", format_seconds(rotation_speed.0 * 60.0)));

                            ui.label(RichText::new("Distance to Camera").size(16.0).underline());
                            let (_, camera_pos) = camera.single();
                            let c_distance_in_units = camera_pos.translation.distance(transform.translation) as f64;
                            ui.label(format!("{}", format_length((c_distance_in_units / M_TO_UNIT) as f32)));
                            ui.label(format!("{:.3} au", c_distance_in_units / M_TO_UNIT * M_TO_AU as f64));

                            // Distance to parent
                            if let Some((parent_pos, _, p_name, _)) = parent {
                                ui.label(RichText::new(format!("Distance to {} (Center)", p_name)).size(16.0).underline());
                                let distance_in_m = parent_pos.0.distance(pos.0);
                                ui.label(format!("{}", format_length(distance_in_m as f32)));
                                ui.label(format!("{:.3} au", distance_in_m * (M_TO_AU as f64)));

                                if let Some(mut apsis) = new_apsis {
                                    //Apsis
                                    ui.label(RichText::new(format!("Periapsis ({})", p_name)).size(16.0).underline());
                                    ui.label(format!("{}", format_length(apsis.perihelion.distance)));
                                    ui.label(format!("{:.3} au", apsis.perihelion.distance * M_TO_AU));

                                    ui.label(RichText::new(format!("Apoapsis ({})", p_name)).size(16.0).underline());
                                    ui.label(format!("{}", format_length(apsis.aphelion.distance)));
                                    ui.label(format!("{:.3} au", apsis.aphelion.distance * M_TO_AU));
                                    if ui.button("Reset Apsides").clicked() {
                                        apsis.aphelion.distance = 0.0;
                                        apsis.perihelion.distance = 0.0;
                                    }

                                    let mut new_draw_lines = orbit.draw_lines;
                                    ui.checkbox(&mut new_draw_lines, "Draw Orbit lines");
                                    if new_draw_lines != orbit.draw_lines {
                                        orbit.draw_lines = new_draw_lines;
                                        if !new_draw_lines {
                                            orbit.lines.clear();
                                        }
                                    }
                                }
                            }

                            if s_children.iter().count() > 0 {
                                let old_draw_children_orbits = s_children.iter().all(|(_, orbit)| {
                                    orbit.draw_lines
                                });
                                let mut draw_children_orbits = old_draw_children_orbits;
                                ui.checkbox(&mut draw_children_orbits, "Draw Children Orbits");
                                if draw_children_orbits != old_draw_children_orbits {
                                    for (_, orbit) in s_children.iter_mut() {
                                        orbit.draw_lines = draw_children_orbits;
                                        if !draw_children_orbits {
                                            orbit.lines.clear();
                                        }
                                    }
                                }
                            }

                            ui.horizontal(|ui| {
                                ui.label("Orbit Color");
                                let orbit_color = orbit.color.to_srgba();
                                let mut rgb = [orbit_color.red, orbit_color.green, orbit_color.blue];
                                ui.color_edit_button_rgb(&mut rgb);
                                orbit.color = Srgba::rgb(rgb[0], rgb[1], rgb[2]).into();
                            });

                            ui.label(
                                RichText::new("Arrows")
                                    .size(16.0)
                                    .underline(),
                            );
                            ui.checkbox(&mut orbit.display_force, "Display force arrow");
                            ui.checkbox(&mut orbit.display_velocity, "Display velocity arrow");
                            ui.label(
                                RichText::new("Scale")
                                    .size(14.0)
                            );
                            ui.add(
                                egui::Slider::new(&mut orbit.arrow_scale, 1..=100000000)
                                    .clamp_to_range(true)
                                    .logarithmic(true)
                            );

                            //     ui.label("Max Orbit Points");
                            //      let mut old_length = orbit.lines.capacity();
                            //    ui.add(egui::DragValue::new(&mut old_length).speed(1.0));

                            //   if old_length != orbit.lines.capacity() {
                            //        orbit.lines.resize(old_length, Vec3::ZERO);
                            //    }
                            ui.separator();
                            if ui.button("Delete Children").clicked() {
                                for (entity, _) in s_children {
                                    commands.entity(entity).despawn_recursive();
                                }
                            }
                            if ui.button("Delete").clicked() {
                                commands.entity(entity).despawn_recursive();
                            }
                        });
                });
        }
    }
}
