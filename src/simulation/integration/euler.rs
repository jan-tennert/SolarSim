use std::time::Instant;

use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::Diagnostics;
use bevy::math::{DVec3, Vec3};
use bevy::prelude::{in_state, not, Entity, IntoSystemConfigs, Mut, Query, Res, ResMut, Time, Transform};
use bevy::reflect::List;

use crate::constants::G;
use crate::simulation::components::body::{Acceleration, Mass, OrbitSettings, SimPosition, Velocity};
use crate::simulation::components::motion_line::OrbitOffset;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::components::speed::Speed;
use crate::simulation::integration::{paused, IntegrationType, SimulationStep, SubSteps, NBODY_STEPS, NBODY_STEP_TIME, NBODY_TOTAL_TIME};
use crate::utils::sim_state_type_simulation;

pub struct EulerIntegrationPlugin;

impl Plugin for EulerIntegrationPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (apply_physics).in_set(SimulationStep).run_if(sim_state_type_simulation).run_if(in_state(IntegrationType::Euler)).run_if(not(paused)));
    }

}

fn apply_physics(
    mut query: Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    time: Res<Time>,
    speed: Res<Speed>,
    selected_entity: Res<SelectedEntity>,
    mut orbit_offset: ResMut<OrbitOffset>,
    sub_steps: Res<SubSteps>,
    mut diagnostics: Diagnostics,
    scale: Res<SimulationScale>,
) {
    let count = query.iter().count();
    let delta = time.delta_seconds() as f64;
    #[cfg(not(target_arch = "wasm32"))]
    let start = Instant::now();
    for _ in 0..sub_steps.0 - 1 {
        update_acceleration(&mut query, count);
        update_velocity_and_positions(&mut query, delta, &speed, &selected_entity, &mut orbit_offset, false, &scale);
    }
    let start_step = Instant::now();
    update_acceleration(&mut query, count);
    update_velocity_and_positions(&mut query, delta, &speed, &selected_entity, &mut orbit_offset, true, &scale);
    diagnostics.add_measurement(&NBODY_STEP_TIME, || start_step.elapsed().as_nanos() as f64);
    diagnostics.add_measurement(&NBODY_TOTAL_TIME, || start.elapsed().as_nanos() as f64);
    diagnostics.add_measurement(&NBODY_STEPS, || (sub_steps.0 as f64 / delta));
}

fn update_acceleration(
    query: &mut Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    count: usize
) {
    let mut other_bodies: Vec<(Entity, &Mass, Mut<Acceleration>, Mut<SimPosition>)> = Vec::with_capacity(count);
    for (entity, mass, mut acc, _, _, sim_pos, _) in query.iter_mut() {
        acc.0 = DVec3::ZERO;
        for (other_entity, other_mass, ref mut other_acc, other_sim_pos) in other_bodies.iter_mut() {
            let distance = other_sim_pos.current - sim_pos.current;
            let r_sq = distance.length_squared();
            let force_direction = distance.normalize(); // Calculate the direction vector
            let force_magnitude = G * mass.0 * other_mass.0 / r_sq;
            let force = force_direction * force_magnitude;
            acc.0 += force;
            other_acc.0 -= force;
        }
        other_bodies.push((entity, mass, acc, sim_pos));
    }
}

fn update_velocity_and_positions(
    query: &mut Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    delta_time: f64,
    speed: &Res<Speed>,
    selected_entity: &Res<SelectedEntity>,
    orbit_offset: &mut ResMut<OrbitOffset>,
    last_step: bool,
    scale: &SimulationScale,
) {
    let offset = match selected_entity.entity { //if orbit_offset.enabled is true, we calculate the new position of the selected entity first and then move it to 0,0,0 and add the actual position to all other bodies
        Some(selected) => {
            if !last_step {
                DVec3::ZERO
            } else if let Ok((_, mass, mut acc, mut orbit_s, mut vel, mut sim_pos, mut transform)) = query.get_mut(selected) {
                if last_step {
                    orbit_s.force_direction = acc.0.normalize();
                }
                acc.0 /= mass.0; //actually apply the force to the body
                vel.0 += acc.0 * delta_time * speed.0;
                sim_pos.current += vel.0 * delta_time * speed.0; //this is the same step as below, but we are doing this first for the offset
                let raw_translation = scale.m_to_unit_dvec(sim_pos.current);
                transform.translation = Vec3::ZERO; //the selected entity will always be at 0,0,0
                -raw_translation 
            } else {
                DVec3::ZERO 
            }
        }
        None => DVec3::ZERO,
    };
    for (entity, mass, mut acc, mut orbit_s, mut vel, mut sim_pos, mut transform) in query.iter_mut() {
        if last_step {
            if let Some(s_entity) = selected_entity.entity {
                if s_entity == entity {
                    continue;
                }
            }
        }
        if last_step {
            orbit_s.force_direction = acc.0.normalize();
        }
        acc.0 /= mass.0; //actually apply the force to the body
        vel.0 += acc.0 * delta_time * speed.0;
        sim_pos.current += vel.0 * delta_time * speed.0;
        if last_step {
            let pos_without_offset = scale.m_to_unit_dvec(sim_pos.current);
            transform.translation = (pos_without_offset + offset).as_vec3(); //apply offset
        }
    }
    if last_step {
        orbit_offset.value = offset.as_vec3();   
    }
}