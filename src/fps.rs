use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin, DiagnosticsStore};
use bevy::prelude::{Res, ResMut, Resource, Time, Timer, TimerMode, IntoSystemConfigs, in_state};

use crate::SimState;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<Fps>()
            .init_resource::<FpsConfig>()
            .add_systems(Update, measure_fps.run_if(in_state(SimState::Simulation)));
    }

}

#[derive(Debug, Resource, Default)]
pub struct Fps(pub f64);

#[derive(Resource)]
struct FpsConfig(Timer);

impl Default for FpsConfig {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

fn measure_fps(
    mut timer: ResMut<FpsConfig>,
    time: Res<Time>,
    mut fps_res: ResMut<Fps>,
    //    mut state: ResMut<LogDiagnosticsState>,
    diagnostics: Res<DiagnosticsStore>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        if let Some(diagnostics) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps) = diagnostics.average() {
                fps_res.0 = fps;
            }
        }
    }
}