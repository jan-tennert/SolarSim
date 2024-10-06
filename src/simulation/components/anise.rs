use crate::simulation::components::horizons::AniseMetadata;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::ui::editor_body_panel::EditorPanelState;
use crate::simulation::ui::toast::{error_toast, success_toast, ToastContainer};
use crate::simulation::{SimState, SimStateType};
use anise::constants::frames::{IAU_EARTH_FRAME, JUPITER_BARYCENTER_J2000, SSB_J2000};
use anise::constants::orientations::{IAU_EARTH, J2000};
use anise::errors::AlmanacError;
use anise::math::cartesian::CartesianState;
use anise::math::Vector3;
use anise::prelude::{Almanac, Epoch, Frame, SPK};
use anise::structure::planetocentric::ellipsoid::Ellipsoid;
use bevy::app::Plugin;
use bevy::math::DVec3;
use bevy::prelude::{in_state, IntoSystemConfigs, Local, OnEnter, Quat, Query, Res, ResMut, Resource, State, Update};
use bevy_async_task::{AsyncTaskRunner, AsyncTaskStatus};
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
    mut bodies: Query<&mut AniseMetadata>,
    almanac: Res<AlmanacHolder>,
    mut e_state: ResMut<EditorPanelState>,
    scenario: Res<ScenarioData>,
    mut toasts: ResMut<ToastContainer>
) {
    // Define an Epoch in the dynamical barycentric time scale
    let epoch = Epoch::from_unix_milliseconds(scenario.starting_time_millis as f64);
    let mut metadata = selected_entity.entity.map(|e| bodies.get_mut(e).ok()).flatten().unwrap();
    let state = almanac.0
        .translate(
            Frame::new(metadata.ephemeris_id, J2000), // Target
            SSB_J2000, // Observer
            epoch,
            None,
        );
    if let Ok(s) = state {
        toasts.0.add(success_toast(&format!("Retrieved data for {}", metadata.ephemeris_id)));
        e_state.new_velocity = vector3_to_dvec3(s.velocity_km_s);
        e_state.new_position = vector3_to_dvec3(s.radius_km);
    } else {
        toasts.0.add(error_toast(format!("Error: {:?}", state.unwrap_err()).as_str()));
    }

    let fixed_frame = Frame::new(metadata.target_id, metadata.orientation_id);
    let full_frame = almanac.0.frame_from_uid(fixed_frame);

    if let Ok(f) = full_frame {
        e_state.ellipsoid = f.shape.unwrap_or(e_state.ellipsoid);
    } else {
        toasts.0.add(error_toast(format!("Error: {:?}", full_frame.unwrap_err()).as_str()));
    }

    let dcm = almanac.0.rotation_to_parent(fixed_frame, epoch);

    if let Ok(d) = dcm {
        e_state.rotation_matrix = matrix3_to_mat3(d.rot_mat);
    } else {
        toasts.0.add(error_toast(format!("Error: {:?}", dcm.unwrap_err()).as_str()));
    }
}

fn matrix3_to_mat3(m: anise::math::Matrix3) -> bevy::math::Mat3 {
    bevy::math::Mat3::from_cols(
        bevy::math::Vec3::new(m.data.0[0][0] as f32, m.data.0[0][1] as f32, m.data.0[0][2] as f32),
        bevy::math::Vec3::new(m.data.0[1][0] as f32, m.data.0[1][1] as f32, m.data.0[1][2] as f32),
        bevy::math::Vec3::new(m.data.0[2][0] as f32, m.data.0[2][1] as f32, m.data.0[2][2] as f32)
    )
}

fn vector3_to_dvec3(v: Vector3) -> DVec3 {
    DVec3::new(v.x, v.y, v.z)
}

fn spk_file_loading(
    mut almanac: ResMut<AlmanacHolder>,
    mut toasts: ResMut<ToastContainer>,
    mut scenario_data: ResMut<ScenarioData>,
    mut loading_state: ResMut<LoadingState>,
    mut task_executor: AsyncTaskRunner<Result<Almanac, AlmanacError>>,
    mut to_load: Local<Option<Vec<String>>>
) {
    if loading_state.loaded_spice_files || !loading_state.spawned_bodies {
        return;
    }
    if to_load.is_none() {
        *to_load = Some(scenario_data.spice_files.clone());
        loading_state.spice_total = to_load.as_ref().unwrap().len() as i32;
    }
    let to_load_v = to_load.as_mut().unwrap();
    match task_executor.poll() {
        AsyncTaskStatus::Idle => {
            if !to_load_v.is_empty() {
                println!("Loading SPICE file: {}", to_load_v.last().unwrap());
                let path = to_load_v.pop().unwrap();
                task_executor.start(load_spice_file(path, almanac.0.clone()));
            } else {
                loading_state.loaded_spice_files = true;
                *to_load = None;
            }
        }
        AsyncTaskStatus::Pending => {
            // <Insert loading screen>
        }
        AsyncTaskStatus::Finished(r) => {
            if let Ok(v) = r {
                almanac.0 = v;
            } else if let Err(e) = r {
                toasts.0.add(error_toast(format!("Couldn't load SPICE file: {}", e).as_str()));
            }
            loading_state.spice_loaded += 1;
        }
    }
}

async fn load_spice_file(
    path: String,
    almanac: Almanac
) -> Result<Almanac, AlmanacError> {
    almanac.load(format!("data/{}", path).as_str())
}

pub fn load_spice_files(
    paths: Vec<String>,
    almanac: &mut AlmanacHolder,
    toasts: &mut ToastContainer
) {
    let mut missing_paths = Vec::new();
    for path in paths {
        if let Ok(a) = almanac.0.load(format!("data/{}", path).as_str()) {
            almanac.0 = a;
        } else {
            missing_paths.push(path);
        }
    }
    if !missing_paths.is_empty() {
        toasts.0.add(error_toast(format!("Couldn't load the following SPICE files: {}", missing_paths.join(", ")).as_str()));
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