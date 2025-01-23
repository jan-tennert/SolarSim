use crate::simulation::ui::toast::{success_toast, ToastContainer};
use bevy::math::DVec3;
use bevy_egui::egui;
use bevy_egui::egui::Ui;

pub fn vector_field(ui: &mut Ui, name: &str, state: &mut DVec3) {
    ui.vertical(|ui| {
        ui.label(name);
        coordinate_field(ui, "X", &mut state.x);
        coordinate_field(ui, "Y", &mut state.y);
        coordinate_field(ui, "Z", &mut state.z);
    });
}

pub fn copy_value_button(ui: &mut Ui, value: &mut f64, container: &mut ToastContainer) {
    if ui.button("Copy value").on_hover_text("Copy the value to the clipboard").clicked() {
        ui.output_mut(|o| o.copied_text = value.to_string());
        container.0.add(success_toast("Value copied to clipboard"));
    }
}

pub fn body_property_field(ui: &mut Ui, actual_value: &mut f64, value: &mut f64) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if ui.button(":5").clicked() {
                *value /= 5.0;
            }
            if ui.button(":2").clicked() {
                *value /= 2.0;
            }
            if ui.button("x2").clicked() {
                *value *= 2.0;
            }
            if ui.button("x5").clicked() {
                *value *= 5.0;
            }
        });
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(value).max_decimals(20));
            if ui.button("Set").clicked() {
                *actual_value = *value;
            }
        })
    });
}

pub fn body_multiplier_field(ui: &mut Ui, actual_value: &mut DVec3, multiplier: &mut f64) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if ui.button(":5").clicked() {
                *multiplier /= 5.0;
            }
            if ui.button(":2").clicked() {
                *multiplier /= 2.0;
            }
            if ui.button("x2").clicked() {
                *multiplier *= 2.0;
            }
            if ui.button("x5").clicked() {
                *multiplier *= 5.0;
            }
        });
        ui.horizontal(|ui| {
            ui.label("Multiplier");
            ui.add(egui::DragValue::new(multiplier).max_decimals(20));
            if ui.button("Set").clicked() {
                *actual_value *= *multiplier;
            }
        })
    });
}

fn coordinate_field(ui: &mut Ui, name: &str, state: &mut f64) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(egui::DragValue::new(state).max_decimals(20));
    });
}