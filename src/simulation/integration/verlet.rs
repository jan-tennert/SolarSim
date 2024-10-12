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

pub struct VerletIntegrationPlugin;

impl Plugin for VerletIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (apply_physics).in_set(SimulationStep).run_if(sim_state_type_simulation).run_if(in_state(IntegrationType::Verlet)).run_if(not(paused)));
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
    let start = Instant::now();
    let count = query.iter().count();
    let delta = time.delta_seconds() as f64;
    let timestep = delta * speed.0;
    for _ in 0..sub_steps.0-1 {
        step(&mut query, count, timestep, &mut orbit_offset, &selected_entity, &scale);
    }
    let start_step = Instant::now();
    step(&mut query, count, timestep, &mut orbit_offset, &selected_entity, &scale);
    diagnostics.add_measurement(&NBODY_STEP_TIME, || start_step.elapsed().as_nanos() as f64);
    diagnostics.add_measurement(&NBODY_TOTAL_TIME, || start.elapsed().as_nanos() as f64);
    diagnostics.add_measurement(&NBODY_STEPS, || (sub_steps.0 as f64 / delta));
}

fn step(
    query: &mut Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    count: usize,
    timestep: f64,
    orbit_offset: &mut OrbitOffset,
    selected_entity: &SelectedEntity,
    simulation_scale: &SimulationScale
) {
    update_acceleration(query, count);
    calculate_half_vel_and_pos(query, timestep);
    update_acceleration(query, count);
    final_velocity(query, timestep);
    update_positions(query, orbit_offset, selected_entity, simulation_scale);
}

fn update_acceleration(
    query: &mut Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    count: usize,
) {
    let mut other_bodies: Vec<(&Mass, Mut<Acceleration>, Mut<SimPosition>)> = Vec::with_capacity(count);
    query.iter_mut().for_each(|(_, mass, mut acc, _, _, sim_pos, _)|  {
        acc.0 = DVec3::ZERO;
        for (other_mass, ref mut other_acc, other_sim_pos) in other_bodies.iter_mut() {
            let distance = other_sim_pos.current - sim_pos.current;
            let r_sq = distance.length_squared();
            let force_direction = distance.normalize(); // Calculate the direction vector
            let force_magnitude = G * mass.0 * other_mass.0 / r_sq;
            let force = force_direction * force_magnitude;
            acc.0 += force;
            other_acc.0 -= force;
        }
        other_bodies.push((mass, acc, sim_pos));
    });
}

fn calculate_half_vel_and_pos(
    query: &mut Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    timestep: f64
) {
    for (_, mass, mut acc, _, mut vel, mut pos, _) in query.iter_mut() {
        acc.0 /= mass.0;
        vel.0 += 0.5 * acc.0 * timestep;
        pos.current += vel.0 * timestep;
    }
}

fn final_velocity(
    query: &mut Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    timestep: f64
) {
    for (_, mass, mut acc, _, mut vel, _, _) in query.iter_mut() {
        acc.0 /= mass.0;
        vel.0 += 0.5 * acc.0 * timestep;
    }
}

fn update_positions(
    query: &mut Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    orbit_offset: &mut OrbitOffset,
    selected_entity: &SelectedEntity,
    scale: &SimulationScale
) {
    let offset = match selected_entity.entity { //if orbit_offset.enabled is true, we calculate the new position of the selected entity first and then move it to 0,0,0 and add the actual position to all other bodies
        Some(selected) => {
            if let Ok((_, mass, mut acc, mut orbit_s, mut vel, mut sim_pos, mut transform)) = query.get_mut(selected) {
                if orbit_s.display_force {
                    orbit_s.force_direction = acc.0.normalize();
                }
                let raw_translation = scale.m_to_unit_dvec(sim_pos.current);
                transform.translation = Vec3::ZERO; //the selected entity will always be at 0,0,0
                -raw_translation
            } else {
                DVec3::ZERO
            }
        }
        None => DVec3::ZERO
    };
    for (entity, _, mut acc, mut orbit_s, mut vel, mut sim_pos, mut transform) in query.iter_mut() {
        if let Some(s_entity) = selected_entity.entity {
            if s_entity == entity {
                continue;
            }
        }
        if orbit_s.display_force {
            orbit_s.force_direction = acc.0.normalize();
        }
        let pos_without_offset = scale.m_to_unit_dvec(sim_pos.current);
        transform.translation = (pos_without_offset + offset).as_vec3(); //apply offset
    }
    orbit_offset.value = offset.as_vec3();
}