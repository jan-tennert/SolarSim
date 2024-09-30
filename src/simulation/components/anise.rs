use std::str::FromStr;
use anise::constants::celestial_objects::SOLAR_SYSTEM_BARYCENTER;
use anise::constants::frames::{EARTH_J2000, SSB_J2000, SUN_J2000};
use anise::constants::orientations::J2000;
use anise::math::Vector3;
use anise::prelude::{Almanac, Epoch, Frame, SPK};
use bevy::app::Plugin;
use bevy::math::DVec3;
use bevy::prelude::{Query, Res, ResMut, Resource};
use crate::setup::ScenarioData;
use crate::simulation::components::horizons::{HorizonsClient, NaifIdComponent};
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::ui::editor_body_panel::EditorPanelState;
use crate::simulation::ui::toast::{error_toast, success_toast, ToastContainer};

pub struct AnisePlugin;

impl Plugin for AnisePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<AlmanacHolder>();
    }
}

#[derive(Resource)]
pub struct AlmanacHolder(pub Almanac);

impl Default for AlmanacHolder {
    fn default() -> Self {
        Self(Almanac::default())
    }
}

pub fn retrieve_starting_data(
    selected_entity: Res<SelectedEntity>,
    bodies: Query<&NaifIdComponent>,
    almanac: Res<AlmanacHolder>,
    mut e_state: ResMut<EditorPanelState>,
    scenario: Res<ScenarioData>,
    mut toasts: ResMut<ToastContainer>
) {
    // Define an Epoch in the dynamical barycentric time scale
    let epoch = Epoch::from_unix_milliseconds(scenario.starting_time_millis as f64);
    let id = selected_entity.entity.map(|e| bodies.get(e).ok()).flatten().unwrap();
    let state = almanac.0
        .translate(
            Frame::new(id.0, J2000), // Target
            SSB_J2000, // Observer
            epoch,
            None,
        )
        .unwrap();

    toasts.0.add(success_toast(&format!("Retrieved data for {}", id.0)));
    e_state.new_velocity = vector3_to_dvec3(state.velocity_km_s);
    e_state.new_position = vector3_to_dvec3(state.radius_km);
}

fn vector3_to_dvec3(v: Vector3) -> DVec3 {
    DVec3::new(v.x, v.y, v.z)
}

pub fn load_spk_files(
    paths: Vec<String>,
    almanac: &mut ResMut<AlmanacHolder>,
    toasts: &mut ResMut<ToastContainer>
) {
    let mut missing_paths = Vec::new();
    for path in paths {
        let spk = SPK::load(format!("data/{}", path).as_str()).unwrap();
        if let Ok(a) = almanac.0.with_spk(spk) {
            almanac.0 = a;
        } else {
            missing_paths.push(path);
        }
    }
    if missing_paths.is_empty() {
        toasts.0.add(success_toast("Loaded all SPK files"));
    } else {
        toasts.0.add(error_toast(format!("Couldn't load the following SPK files: {}", missing_paths.join(", ")).as_str()));
    }
}
