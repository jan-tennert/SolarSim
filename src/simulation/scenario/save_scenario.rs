use crate::simulation::asset::serialization::{SerializedBody, SerializedBodyData, SerializedFixedBodyFrame, SerializedLightSource, SerializedMat3, SerializedVec, SimulationData};
use crate::simulation::components::body::{BodyChildren, BodyRotation, BodyShape, LightSource, Mass, ModelPath, RotationSpeed, SimPosition, Star, Velocity};
use crate::simulation::components::horizons::AniseMetadata;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::speed::Speed;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::ui::scenario_selection::SelectedScenario;
use crate::simulation::ui::toast::{success_toast, ToastContainer};
use anise::structure::planetocentric::ellipsoid::Ellipsoid;
use bevy::app::Plugin;
use bevy::ecs::system::SystemParam;
use bevy::math::DVec3;
use bevy::prelude::Name;
use bevy::prelude::{Assets, Entity, Query, Res, ResMut};
use std::fs;

pub struct SaveScenarioPlugin;

impl Plugin for SaveScenarioPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        app
            ;
    }

}

#[derive(SystemParam)]
pub struct SystemPanelSet<'w, 's> {
    selected_scenario: ResMut<'w, SelectedScenario>,
    bodies_asset: ResMut<'w, Assets<SimulationData>>,
    scenario_data: ResMut<'w, ScenarioData>,
    bodies: Query<'w, 's, (Entity, &'static Mass, &'static SimPosition, &'static Velocity, &'static Name, &'static ModelPath, &'static BodyShape, &'static RotationSpeed, &'static BodyRotation, Option<&'static BodyChildren>, &'static AniseMetadata, &'static BodyRotation, Option<&'static Star>)>,
    lights: Query<'w, 's, &'static LightSource>,
    toasts: ResMut<'w, ToastContainer>,
    scale: Res<'w, SimulationScale>,
    speed: Res<'w, Speed>

}

pub fn save_scenario(
    mut system_panel_set: SystemPanelSet
) {
    let file_path = get_file_path(&system_panel_set);
    let bodies = collect_bodies(&system_panel_set);
    let scenario_data = &*system_panel_set.scenario_data;
    let simulation_data: SimulationData = SimulationData {
        bodies,
        starting_time_millis: scenario_data.starting_time_millis,
        title: scenario_data.title.clone(),
        description: scenario_data.description.clone(),
        scale: system_panel_set.scale.0,
        timestep: system_panel_set.speed.0 as i32,
        data_sets: scenario_data.spice_files.keys().cloned().collect(),
    };
    let serialized_data = serde_json::to_string_pretty(&simulation_data).unwrap();
    fs::write(format!("scenarios/{}", file_path), serialized_data).unwrap();
    system_panel_set.toasts.0.add(success_toast("Scenario saved"));
}

fn get_file_path<'s>(system_panel_set: &'s SystemPanelSet) -> &'s str {
    system_panel_set.selected_scenario.handle.path().unwrap().path().file_name().unwrap().to_str().unwrap()
}

fn collect_bodies(system_panel_set: &SystemPanelSet) -> Vec<SerializedBody> {
    let mut bodies = Vec::new();
    system_panel_set.bodies.iter().filter(|(_, _, _, _, _, _, _, _, _, _, _, _, star)| star.is_some()).for_each(|(entity, _, _, _, _, _, _ ,_, _, children, _, _, _)| {
        let mut data = find_body_data(system_panel_set, entity).map(|(data, _)| data).unwrap();
        let light_source = find_light_source(system_panel_set, entity);
        data.light_source = light_source;
        let planets = collect_planets(system_panel_set, children.unwrap().clone());
        bodies.push(SerializedBody { children: planets, data });
    });
    bodies
}

fn collect_planets(system_panel_set: &SystemPanelSet, children: BodyChildren) -> Vec<SerializedBody> {
    let mut planets = Vec::new();
    for planet_entity in children.0.clone() {
        if let Some((planet_data, planet_children)) = find_body_data(system_panel_set, planet_entity) {
            let moons = collect_moons(system_panel_set, planet_children.unwrap().clone());
            planets.push(SerializedBody { children: moons, data: planet_data.clone() });
        }
    }
    planets
}

fn collect_moons(system_panel_set: &SystemPanelSet, children: BodyChildren) -> Vec<SerializedBody> {
    let mut moons = Vec::new();
    for child_entity in children.0.clone() {
        if let Some(child_data) = find_body_data(system_panel_set, child_entity).map(|(data, _)| data) {
            moons.push(SerializedBody { children: Vec::new(), data: child_data.clone() });
        }
    }
    moons
}

fn find_body_data(system_panel_set: &SystemPanelSet, entity: Entity) -> Option<(SerializedBodyData, Option<BodyChildren>)> {
    system_panel_set.bodies.iter().find(|(e, _, _, _, _, _, _, _, _, _, _, _, _)| *e == entity)
        .map(|(_, m, p, v, n, mp, d, rs, _at, child, naif, rotation, _)| (
            create_serialized_body_data(m.0, p.current / 1000.0, v.0 / 1000.0, n.to_string(), mp.cleaned(), rs.0, None, naif.clone(), d.ellipsoid, *rotation),
            child.map(|c| c.clone())
        ))
}
fn create_serialized_body_data(
    mass: f64,
    position: DVec3,
    velocity: DVec3,
    name: String,
    model_path: String,
    rotation_speed: f64,
    light_source: Option<SerializedLightSource>,
    anise_metadata: AniseMetadata,
    ellipsoid: Ellipsoid,
    rotation: BodyRotation
) -> SerializedBodyData {
    SerializedBodyData {
        mass,
        starting_position: SerializedVec::from(position),
        starting_velocity: SerializedVec::from(velocity),
        name,
        model_path,
        rotation_speed,
        simulate: true,
        ellipsoid,
        light_source,
        naif_id: anise_metadata.ephemeris_id,
        fixed_body_frame: SerializedFixedBodyFrame {
            target_id: anise_metadata.target_id,
            orientation_id: anise_metadata.orientation_id
        },
        rotation_matrix: SerializedMat3::from(rotation.matrix)
    }
}

fn find_light_source(
    system_panel_set: &SystemPanelSet,
    entity: Entity,
) -> Option<SerializedLightSource> {
    system_panel_set.lights.iter().find(|s| s.parent == entity).map(|s| SerializedLightSource {
        intensity: s.intensity,
        range: s.range,
        color: s.color.to_srgba().to_hex(),
        imposter_color: s.imposter_color.to_srgba().to_hex(),
        enabled: s.enabled
    })
}