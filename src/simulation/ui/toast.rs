use bevy::app::{App, Plugin, Update};
use bevy::prelude::{default, ResMut, Resource};
use bevy_egui::EguiContexts;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};

pub struct ToastPlugin;

impl Plugin for ToastPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<ToastContainer>()
            .add_systems(Update, show_toasts);
    }

}

#[derive(Resource)]
pub struct ToastContainer(pub Toasts);

impl Default for ToastContainer {
    fn default() -> Self {
        Self(Toasts::default())
    }
}

fn show_toasts(
    mut egui_context: EguiContexts,
    mut toasts: ResMut<ToastContainer>
) {
    toasts.0.show(egui_context.ctx_mut());
}

pub fn success_toast(text: &str) -> Toast {
    Toast {
        text: text.into(),
        kind: ToastKind::Success,
        options: ToastOptions::default()
            .duration_in_seconds(3.0),
        ..default()
    }
}

pub fn error_toast(text: &str) -> Toast {
    Toast {
        text: text.into(),
        kind: ToastKind::Error,
        options: ToastOptions::default()
            .duration_in_seconds(3.0),
        ..default()
    }
}

pub fn important_error_toast(text: &str) -> Toast {
    Toast {
        text: text.into(),
        kind: ToastKind::Error,
        options: ToastOptions::default()
            .duration_in_seconds(5.0),
        ..default()
    }
}