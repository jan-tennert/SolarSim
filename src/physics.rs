use std::process::exit;
use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{in_state, IntoSystemConfigs, Mut, Query, Res, Resource, Time, Transform};
use crate::body::{Acceleration, Mass, SimPosition, Velocity};
use crate::constants::{G, KM_TO_AU};
use crate::SimState;
use crate::speed::Speed;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Pause>()
            .add_systems(Update, (update_acceleration, update_velocity.after(update_acceleration), update_position.after(update_velocity)).run_if(in_state(SimState::Simulation)));
    }
}

#[derive(Resource, Default)]
pub struct Pause(pub bool);

fn update_acceleration(
    mut query: Query<(&Mass, &mut Acceleration, &SimPosition)>,
) {
    let mut other_bodies: Vec<(&Mass, Mut<Acceleration>, &SimPosition)> = Vec::new();
    for (mass, mut acc, sim_pos) in query.iter_mut() {
        acc.0 = Vec3::ZERO;
        for (other_mass, ref mut other_acc, other_sim_pos) in other_bodies.iter_mut() {
            let r_sq = (sim_pos.0 - other_sim_pos.0).length_squared() as f64;
            let force_direction = (other_sim_pos.0 - sim_pos.0).normalize(); // Calculate the direction vector
            
            let force_magnitude = G * mass.0 * other_mass.0 / r_sq;
          //  println!("r_sq: {}", G * mass.0 * other_mass.0 / r_sq);            
            let force = force_direction * (force_magnitude as f32);
         //   println!("force: {}", force);            
            acc.0 += force;
            other_acc.0 -= force;
        }
        other_bodies.push((mass, acc, sim_pos));
    }
    for (mass, mut acc, _) in query.iter_mut() {
        acc.0 /= mass.0 as f32;
    }
}

fn update_velocity(
    mut query: Query<(&mut Velocity, &Acceleration)>,
    time: Res<Time>,
    speed: Res<Speed>
) {
    for (mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0 * time.delta_seconds() * speed.0;
    }
}

pub fn update_position(
    mut query: Query<(&mut SimPosition, &mut Transform, &Velocity)>,
    time: Res<Time>,
    speed: Res<Speed>
) {
    for (mut sim_pos, mut transform, vel) in query.iter_mut() {
        sim_pos.0 += vel.0 * time.delta_seconds() * speed.0;
        transform.translation = sim_pos.0 * KM_TO_AU;
    }
}