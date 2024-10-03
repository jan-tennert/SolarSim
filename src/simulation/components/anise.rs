use crate::simulation::components::horizons::NaifIdComponent;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::ui::editor_body_panel::EditorPanelState;
use crate::simulation::ui::toast::{error_toast, success_toast, ToastContainer};
use crate::simulation::SimState;
use anise::constants::frames::{JUPITER_BARYCENTER_J2000, SSB_J2000};
use anise::constants::orientations::J2000;
use anise::math::Vector3;
use anise::prelude::{Almanac, Epoch, Frame, SPK};
use bevy::app::Plugin;
use bevy::math::DVec3;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, Query, Res, ResMut, Resource, Update};
use reqwest::get;

pub struct AnisePlugin;

impl Plugin for AnisePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<AlmanacHolder>()
            .add_systems(Update, spk_file_loading.run_if(in_state(SimState::Loading)));
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
        );

    if let Ok(s) = state {
        toasts.0.add(success_toast(&format!("Retrieved data for {}", id.0)));
        e_state.new_velocity = vector3_to_dvec3(s.velocity_km_s);
        e_state.new_position = vector3_to_dvec3(s.radius_km);
    } else {
        println!("{:?}", state);
        toasts.0.add(error_toast(format!("Error: {:?}", state.unwrap_err()).as_str()));
    }
}

fn vector3_to_dvec3(v: Vector3) -> DVec3 {
    DVec3::new(v.x, v.y, v.z)
}

fn spk_file_loading(
    mut almanac: ResMut<AlmanacHolder>,
    mut toasts: ResMut<ToastContainer>,
    mut scenario_data: ResMut<ScenarioData>,
    mut loading_state: ResMut<LoadingState>
) {
    if loading_state.loaded_spk_files || !loading_state.spawned_bodies {
        return;
    }
    load_spk_files(scenario_data.spk_files.clone(), &mut almanac, &mut toasts);
    loading_state.loaded_spk_files = true;
}

pub fn load_spk_files(
    paths: Vec<String>,
    almanac: &mut AlmanacHolder,
    toasts: &mut ToastContainer
) {
    let mut missing_paths = Vec::new();
    for path in paths {
        let spk = SPK::load(format!("data/{}", path).as_str()).unwrap();
        if let Ok(a) = almanac.0.with_spk(spk) {
            almanac.0 = a;
           // get_target_frames(&almanac.0);
        } else {
            missing_paths.push(path);
        }
    }
    if !missing_paths.is_empty() {
        toasts.0.add(error_toast(format!("Couldn't load the following SPK files: {}", missing_paths.join(", ")).as_str()));
    }
}

/*fn get_target_frames(
    almanac: &Almanac
) -> Vec<Frame> {
    let mut frames = almanac.spk_data.iter().filter(|s| s.is_some()).map(|s| s.as_ref().unwrap()).map(|s| {
        s.data_summaries().unwrap().iter().map(|d| d.target_frame())
    }).flatten().collect::<Vec<Frame>>();
    frames.dedup_by_key(|f| f.ephemeris_id);
    println!("{:?}", frames);
    Vec::new()
}*/