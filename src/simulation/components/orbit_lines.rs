use bevy::{prelude::{in_state, App, Camera, Entity, Gizmos, IntoSystemConfigs, Plugin, PreUpdate, Query, Res, Resource, Transform, Vec3, With, Without}, time::Time};

use crate::{constants::M_TO_UNIT};
use crate::simulation::components::body::{BillboardVisible, BodyChildren, Diameter, Moon, OrbitSettings, Planet, SimPosition, Star};
use crate::simulation::components::camera::PanOrbitCamera;
use crate::simulation::components::physics::{apply_physics, Pause, SubSteps};
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::components::speed::Speed;
use crate::simulation::SimState;
use crate::simulation::ui::UiState;

pub struct OrbitLinePlugin;

impl Plugin for OrbitLinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OrbitOffset>()
            .add_systems(PreUpdate, (update_lines.after(apply_physics), draw_orbit_line.after(update_lines)).run_if(in_state(SimState::Loaded)));
    }
}

#[derive(Resource)]
pub struct OrbitOffset {
    
    pub value: Vec3,
    pub enabled: bool,
    
}

impl Default for OrbitOffset {
    
    fn default() -> Self {
        OrbitOffset {
            value: Vec3::ZERO,
            enabled: true,
        }
    }
    
}

const MULTIPLIER: f32 = 0.0001;
const PLANET_HIDE_MULTIPLIER: f32 = 10000.0;
const HIDE_MULTIPLIER: f32 = 100.0;

fn update_lines(
    mut planet_query: Query<(Entity, &mut OrbitSettings, &SimPosition, &BodyChildren, &Diameter, &BillboardVisible), (With<Planet>, Without<Moon>, Without<Star>)>,
    mut moon_query: Query<(Entity, &SimPosition, &mut OrbitSettings, &Diameter, &BillboardVisible), (With<Moon>, Without<Planet>, Without<Star>)>,
    camera: Query<&PanOrbitCamera, With<Camera>>,
    time: Res<Time>,
    speed: Res<Speed>,
    substeps: Res<SubSteps>,
    pause: Res<Pause>,
    selected_entity: Res<SelectedEntity>,
    ui_state: Res<UiState>
) {
    if pause.0 {
        return;
    }
    let cam = camera.single();
    for (entity, mut orbit, pos, _, diameter, billboard_visible) in &mut planet_query {
        if orbit.draw_lines {
            orbit.hide_lines = (cam.radius < diameter.num * PLANET_HIDE_MULTIPLIER && entity == selected_entity.entity.unwrap() || !billboard_visible.0) && ui_state.dyn_hide_orbit_lines;
            let speed = speed.0 as f32 * (substeps.0 as f32);
            let max_step = (orbit.period as f32 / speed) * MULTIPLIER;
            if orbit.step >= max_step {
                orbit.lines.push_back((pos.0 * M_TO_UNIT).as_vec3());
              //  insert_at_nearest_distance(&mut orbit.lines, (pos.0 * M_TO_UNIT).as_vec3());
                orbit.step = 0.0;
            } else {
                orbit.step += time.delta_seconds();
            }
        }
    }
    for (entity, pos, mut orbit, diameter, billboard_visible) in &mut moon_query {
        if orbit.draw_lines {
            if let Some((_, _, p_pos, _, _, _)) = planet_query.iter().find(|(_, _, _, children, _, _)| {
                children.0.contains(&entity)
            }) {
                orbit.hide_lines = (cam.radius < diameter.num * HIDE_MULTIPLIER && entity == selected_entity.entity.unwrap() || !billboard_visible.0) && ui_state.dyn_hide_orbit_lines;
                let speed = speed.0 as f32 * (substeps.0 as f32);
                let max_step = (orbit.period as f32 / speed) * MULTIPLIER;
                if orbit.step >= max_step {
                    let raw_p_pos = (p_pos.0 * M_TO_UNIT).as_vec3();
                    let raw_pos = (pos.0 * M_TO_UNIT).as_vec3();
                    orbit.lines.push_back(raw_pos - raw_p_pos);   
                    //insert_at_nearest_distance(&mut orbit.lines, raw_pos - raw_p_pos);
                    orbit.step = 0.0;
                } else {
                    orbit.step += time.delta_seconds();
                }
            }
        }
    }
}

fn draw_orbit_line(
    offset: Res<OrbitOffset>,
    planet_query: Query<(&OrbitSettings, &SimPosition, &BodyChildren, &Transform), (With<Planet>, Without<Moon>, Without<Star>)>,
    moon_query: Query<(Entity, &OrbitSettings, &Transform), (With<Moon>, Without<Planet>, Without<Star>)>,
    mut gizmos: Gizmos
) {
    for (orbit, _, _, transform) in &planet_query {
        if orbit.draw_lines && !orbit.hide_lines {
            draw_lines(orbit, offset.value, &mut gizmos, transform.translation)
        }
    }
    for (entity, orbit, transform) in &moon_query {
        if orbit.draw_lines && !orbit.hide_lines {
            if let Some((_, p_pos, _, _)) = planet_query.iter().find(|(_, _, children, _)| {
                children.0.contains(&entity)
            }) {
                let raw_p_pos = (p_pos.0 * M_TO_UNIT).as_vec3();
                draw_lines(orbit, offset.value + raw_p_pos, &mut gizmos, transform.translation)
            }
        }
    }
}

pub fn draw_lines(orbit: &OrbitSettings, offset: Vec3, gizmos: &mut Gizmos, current_pos: Vec3) {
    for (index, first) in orbit.lines.iter().enumerate() {
        if let Some(second) = orbit.lines.get(index + 1) {
            gizmos.line(*first + offset, *second + offset, orbit.color);
        }
    }
    if let Some(last) = orbit.lines.iter().last() {
        gizmos.line(*last + offset, current_pos, orbit.color)   
    }
}