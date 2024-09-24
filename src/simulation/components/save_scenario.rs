use std::fs;
use bevy::app::Plugin;
use bevy::core::Name;
use bevy::ecs::system::SystemParam;
use bevy::math::DVec3;
use bevy::prelude::{AssetServer, Assets, Entity, PointLight, Query, Res, ResMut, Visibility};
use crate::constants::M_TO_UNIT;
use crate::serialization::{SerializedBody, SerializedBodyData, SerializedLightSource, SerializedVec, SimulationData};
use crate::setup::StartingTime;
use crate::simulation::components::body::{AxialTilt, BodyChildren, Diameter, LightSource, Mass, ModelPath, RotationSpeed, SimPosition, Star, Velocity};
use crate::simulation::ui::scenario_selection::SelectedScenario;

pub struct SaveScenarioPlugin;

impl Plugin for SaveScenarioPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        app
            ;
    }

}

#[derive(SystemParam)]
pub struct SystemPanelSet<'w, 's> {
    assets: Res<'w, AssetServer>,
    selected_scenario: ResMut<'w, SelectedScenario>,
    bodies_asset: ResMut<'w, Assets<SimulationData>>,
    starting_time: ResMut<'w, StartingTime>,
    bodies: Query<'w, 's, (Entity, &'static Mass, &'static SimPosition, &'static Velocity, &'static Name, &'static ModelPath, &'static Diameter, &'static RotationSpeed, &'static AxialTilt, &'static BodyChildren, Option<&'static Star>)>,
    lights: Query<'w, 's, (&'static LightSource, &'static PointLight, &'static Visibility)>
}

pub fn save_scenario(
    mut system_panel_set: SystemPanelSet
) {
    let file_path = get_file_path(&system_panel_set);
    let bodies = collect_bodies(&system_panel_set);

}

fn get_file_path<'s>(system_panel_set: &'s SystemPanelSet) -> &'s str {
    system_panel_set.selected_scenario.handle.path().unwrap().path().file_name().unwrap().to_str().unwrap()
}

fn collect_bodies(system_panel_set: &SystemPanelSet) -> Vec<SerializedBody> {
    let mut bodies = Vec::new();
    for (entity, mass, position, velocity, name, model_path, diameter, rotation_speed, axial_tilt, children, star) in system_panel_set.bodies.iter() {
        if star.is_none() {
            continue;
        }
        let data = create_serialized_body_data(
            mass.0, position.0, velocity.0, name.to_string(), model_path.0.clone(),
            diameter.num as f64, rotation_speed.0, axial_tilt.num,
            find_light_source(system_panel_set, entity)
        );
        let planets = collect_planets(system_panel_set, children.clone());
        bodies.push(SerializedBody { children: planets, data });
    }
    bodies
}

fn collect_planets(system_panel_set: &SystemPanelSet, children: BodyChildren) -> Vec<SerializedBody> {
    let mut planets = Vec::new();
    for planet_entity in children.0.clone() {
        if let Some((planet_data, planet_children)) = find_body_data(system_panel_set, planet_entity) {
            let moons = collect_moons(system_panel_set, planet_children);
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

fn find_body_data(system_panel_set: &SystemPanelSet, entity: Entity) -> Option<(SerializedBodyData, BodyChildren)> {
    system_panel_set.bodies.iter().find(|(e, _, _, _, _, _, _, _, _, _, _)| *e == entity)
        .map(|(_, m, p, v, n, mp, d, rs, at, child, _)| (
            create_serialized_body_data(m.0, p.0, v.0, n.to_string(), mp.0.clone(), d.num as f64, rs.0, at.num, None),
            child.clone()
        ))
}
fn create_serialized_body_data(
    mass: f64,
    position: DVec3,
    velocity: DVec3,
    name: String,
    model_path: String,
    diameter: f64,
    rotation_speed: f64,
    axial_tilt: f32,
    light_source: Option<SerializedLightSource>
) -> SerializedBodyData {
    SerializedBodyData {
        mass,
        starting_position: SerializedVec::from(position),
        starting_velocity: SerializedVec::from(velocity),
        name,
        model_path,
        diameter: diameter / M_TO_UNIT,
        rotation_speed,
        axial_tilt,
        simulate: true,
        light_source
    }
}

fn find_light_source(
    system_panel_set: &SystemPanelSet,
    entity: Entity
) -> Option<SerializedLightSource> {
    system_panel_set.lights.iter().find(|(s, _, _)| s.0 == entity).map(|(_, light, visibility)| SerializedLightSource {
        intensity: light.intensity as f64,
        range: light.range as f64,
        color: light.color.to_srgba().to_hex(),
        enabled: visibility == &Visibility::Visible
    })
}