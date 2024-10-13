use crate::simulation::components::horizons::AniseMetadata;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::ui::editor_body_panel::EditorPanelState;
use crate::simulation::ui::toast::{error_toast, success_toast, ToastContainer};
use crate::simulation::{SimState, SimStateType};
use anise::constants::frames::SSB_J2000;
use anise::constants::orientations::J2000;
use anise::math::Vector3;
use anise::naif::daf::DAF;
use anise::naif::spk::summary::SPKSummaryRecord;
use anise::prelude::{Almanac, Epoch, Frame, SPK};
use anise::structure::PlanetaryDataSet;
use bevy::app::Plugin;
use bevy::math::DVec3;
use bevy::prelude::{IntoSystemConfigs, Name, Query, Res, ResMut, Resource, State, Update};
use bevy_async_task::{AsyncTaskPool, AsyncTaskStatus};
use std::fs;

enum AlmanacType {
    SPK(DAF<SPKSummaryRecord>, String),
    PCA(PlanetaryDataSet, String)
}

struct Error(String);

pub struct AnisePlugin;

impl Plugin for AnisePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<AlmanacHolder>()
            .add_systems(Update, spk_file_loading.run_if(loading_or_editor));
    }
}

#[derive(Resource)]
pub struct AlmanacHolder(pub Almanac);

impl Default for AlmanacHolder {
    fn default() -> Self {
        Self(Almanac::default())
    }
}

fn loading_or_editor(state: Res<State<SimState>>, s_type: Res<SimStateType>) -> bool {
    *state == SimState::Loading || (*state == SimState::Loaded && *s_type == SimStateType::Editor)
}

pub fn retrieve_starting_data(
    selected_entity: Res<SelectedEntity>,
    mut bodies: Query<(&mut AniseMetadata, &Name)>,
    almanac: Res<AlmanacHolder>,
    mut e_state: ResMut<EditorPanelState>,
    scenario: Res<ScenarioData>,
    mut toasts: ResMut<ToastContainer>
) {
    // Define an Epoch in the dynamical barycentric time scale
    let epoch = Epoch::from_unix_milliseconds(scenario.starting_time_millis as f64);
    let (mut metadata, name) = selected_entity.entity.map(|e| bodies.get_mut(e).ok()).flatten().unwrap();
    let state = almanac.0
        .translate(
            Frame::new(metadata.ephemeris_id, J2000), // Target
            SSB_J2000, // Observer
            epoch,
            None,
        );
    if let Ok(s) = state {
        toasts.0.add(success_toast(&format!("Retrieved data for {}", name)));
        e_state.new_velocity = vector3_to_dvec3(s.velocity_km_s);
        e_state.new_position = vector3_to_dvec3(s.radius_km);
    } else {
        toasts.0.add(error_toast(format!("Couldn't retrieve position and velocity: {:?}", state.unwrap_err()).as_str()));
    }

    let fixed_frame = Frame::new(metadata.target_id, metadata.orientation_id);
    let full_frame = almanac.0.frame_from_uid(fixed_frame);

    if let Ok(f) = full_frame {
        e_state.ellipsoid = f.shape.unwrap_or(e_state.ellipsoid);
    } else {
        toasts.0.add(error_toast(format!("Couldn't retrieve shape: {:?}", full_frame.unwrap_err()).as_str()));
    }
    let dcm = almanac.0.rotate(
        fixed_frame,
        SSB_J2000,
        epoch,
    );
    if let Ok(d) = dcm {
        e_state.rotation_matrix = matrix3_to_mat3(d.rot_mat);
    } else {
        toasts.0.add(error_toast(format!("Couldn't retrieve rotation: {:?}", dcm.unwrap_err()).as_str()));
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
    mut task_pool: AsyncTaskPool<Result<AlmanacType, Error>>,
    sim_type: Res<SimStateType>
) {
    if loading_state.loaded_spice_files || !loading_state.spawned_bodies {
        return;
    }
    if *sim_type != SimStateType::Editor || scenario_data.spice_files.is_empty() || (loading_state.spice_loaded > 0 && loading_state.spice_loaded == loading_state.spice_total) {
        loading_state.loaded_spice_files = true;
        return;
    }
    if task_pool.is_idle() && loading_state.spice_total == 0 {
        loading_state.spice_total = scenario_data.spice_files.iter().count() as i32;
        almanac.0 = Almanac::default();
        for (path, mut loaded) in &mut scenario_data.spice_files {
            *loaded = false;
            if path.ends_with(".bsp") {
                task_pool.spawn(load_spk(path.clone()));
            } else if path.ends_with(".pca") {
                task_pool.spawn(load_pca(path.clone()));
            } else {
                toasts.0.add(error_toast(format!("Unsupported SPICE file type: {}", path).as_str()));
            }
        }
    }
    for status in task_pool.iter_poll() {
        if let AsyncTaskStatus::Finished(t) = status {
            match t {
                Ok(AlmanacType::SPK(daf, path)) => {
                    let spk = almanac.0.with_spk(daf);
                    if let Ok(s) = spk {
                        scenario_data.spice_files.insert(path, true);
                        almanac.0 = s;
                    } else if let Err(e) = spk {
                        toasts.0.add(error_toast(format!("Couldn't load SPICE file: {:?}", e).as_str()));
                    }
                }
                Ok(AlmanacType::PCA(set, path)) => {
                    almanac.0 = almanac.0.with_planetary_data(set);
                    scenario_data.spice_files.insert(path, true);
                }
                Err(e) => {
                    toasts.0.add(error_toast(format!("Couldn't load SPICE file: {:?}", e.0).as_str()));
                }
            }
            loading_state.spice_loaded += 1;
        }
    }
}

async fn load_spk(
    path: String,
) -> Result<AlmanacType, Error> {
    let real_path = format!("data/{}", path);
    let spk = SPK::load(real_path.as_str()).map_err(|e| Error(format!("{:?}", e)))?;
    Ok(AlmanacType::SPK(spk, path))
}

async fn load_pca(
    path: String
) -> Result<AlmanacType, Error> {
    let real_path = format!("data/{}", path);
    let data = fs::read(real_path.clone()).map_err(|e| Error(format!("{:?}", e)))?;
    let bytes: &[u8] = data.as_slice();
    let set = PlanetaryDataSet::from_bytes(bytes);
    Ok(AlmanacType::PCA(set, path))
}