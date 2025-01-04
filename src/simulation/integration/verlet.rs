use std::time::Instant;

use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::Diagnostics;
use bevy::math::DVec3;
use bevy::prelude::{in_state, not, Entity, IntoSystemConfigs, Mut, Query, Res, Time, Transform};
use bevy::reflect::List;

use crate::constants::G;
use crate::simulation::components::body::{Acceleration, Mass, OrbitSettings, SimPosition, Velocity};
use crate::simulation::components::speed::Speed;
use crate::simulation::integration::{paused, IntegrationType, SimulationStep, SubSteps, NBODY_STEPS, NBODY_STEP_TIME, NBODY_TOTAL_TIME};
use crate::utils::sim_state_type_simulation;

pub struct VerletIntegrationPlugin;

impl Plugin for VerletIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (apply_physics).before(SimulationStep).run_if(sim_state_type_simulation).run_if(in_state(IntegrationType::Verlet)).run_if(not(paused)));
    }
}

fn apply_physics(
    mut query: Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    time: Res<Time>,
    speed: Res<Speed>,
    sub_steps: Res<SubSteps>,
    mut diagnostics: Diagnostics,
) {
    let start = Instant::now();
    let count = query.iter().count();
    let delta = time.delta_secs_f64();
    let timestep = delta * speed.0;
    for _ in 0..sub_steps.0-1 {
        step(&mut query, count, timestep);
    }
    let start_step = Instant::now();
    step(&mut query, count, timestep);
    diagnostics.add_measurement(&NBODY_STEP_TIME, || start_step.elapsed().as_nanos() as f64);
    diagnostics.add_measurement(&NBODY_TOTAL_TIME, || start.elapsed().as_nanos() as f64);
    diagnostics.add_measurement(&NBODY_STEPS, || (sub_steps.0 as f64 / delta));
}

fn step(
    query: &mut Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    count: usize,
    timestep: f64,
) {
    update_acceleration(query, count);
    calculate_half_vel_and_pos(query, timestep);
    update_acceleration(query, count);
    final_velocity(query, timestep);
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

