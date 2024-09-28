use bevy::color::Srgba;
use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{Camera, Commands, DespawnRecursiveExt, Entity, Mut, Query, Res, ResMut, Transform, Without};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{RichText, ScrollArea};
use crate::constants::{G, M_TO_AU};
use crate::simulation::components::apsis::ApsisBody;
use crate::simulation::components::body::{BodyChildren, BodyParent, Diameter, Mass, OrbitSettings, RotationSpeed, Scale, SimPosition, Velocity};
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::ui::UiState;
use crate::simulation::units::text_formatter::{format_length, format_seconds};

pub fn sim_body_panel(
    mut egui_context: EguiContexts,
    mut commands: Commands,
    mut query: Query<(&Name, Entity, &SimPosition, &mut Velocity, &RotationSpeed, &Diameter, &mut OrbitSettings, &mut Mass, &Scale, &mut Transform, Option<&mut ApsisBody>, Option<&BodyChildren>, Option<&BodyParent>)>,
    camera: Query<(&Camera, &Transform), Without<Velocity>>,
    selected_entity: Res<SelectedEntity>,
    mut ui_state: ResMut<UiState>,
    s_scale: Res<SimulationScale>,
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
                                let scaled_diameter = (diameter.num) * n_scale;
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
                            ui.label(format!("{}", format_length(s_scale.unit_to_m_32(c_distance_in_units as f32))));
                            ui.label(format!("{:.3} au", s_scale.unit_to_m(c_distance_in_units) * M_TO_AU as f64));

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

                            ui.horizontal(|ui| {
                                ui.label("Orbit Line Multiplier");
                                ui.add(
                                    egui::Slider::new(&mut orbit.orbit_line_multiplier, 0.1..=100.0)
                                        .clamp_to_range(true)
                                        .logarithmic(true),
                                );
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