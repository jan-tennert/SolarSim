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

fn coordinate_field(ui: &mut Ui, name: &str, state: &mut f64) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(egui::DragValue::new(state));
    });
}