use std::ops::Add;
use std::process::exit;
use bevy::app::{App, Plugin, Update};
use bevy::math::{Vec3, I64Vec3, Vec3A, DVec3};
use bevy::prelude::{in_state, IntoSystemConfigs, Mut, Query, Res, Resource, Time, Transform, Entity, GlobalTransform, BVec3, Gizmos, Color, ResMut};
use crate::body::{Acceleration, Mass, SimPosition, Velocity, OrbitSettings};
use crate::constants::{G, M_TO_UNIT};
use crate::SimState;
use crate::orbit_lines::{OrbitOffset, draw_lines};
use crate::selection::SelectedEntity;
use crate::speed::Speed;
use itertools::Itertools;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Pause>()
            .register_type::<Velocity>()
            .register_type::<Acceleration>()
            .register_type::<Mass>()
            .register_type::<SimPosition>()
            .register_type::<OrbitSettings>()
            .add_systems(Update, (update_acceleration, update_velocity.after(update_acceleration), update_position.after(update_velocity)).run_if(in_state(SimState::Simulation)));
    }
}

#[derive(Resource, Default)]
pub struct Pause(pub bool);

fn update_acceleration(
    mut query: Query<(&Mass, &mut Acceleration, &SimPosition)>,
    pause: Res<Pause>
) {
    if pause.0 {
        return;
    }
    let mut other_bodies: Vec<(&Mass, Mut<Acceleration>, &SimPosition)> = Vec::new();
    for (mass, mut acc, sim_pos) in query.iter_mut() {
        acc.0 = DVec3::ZERO;
        for (other_mass, ref mut other_acc, other_sim_pos) in other_bodies.iter_mut() {
            let r_sq = (sim_pos.0 - other_sim_pos.0).length_squared() as f64;
            let force_direction = DVec3::from((other_sim_pos.0 - sim_pos.0).normalize()); // Calculate the direction vector  
            
            let force_magnitude = G * mass.0 * other_mass.0 / r_sq;
          //  println!("r_sq: {}", G * mass.0 * other_mass.0 / r_sq);            
            let force = force_direction * force_magnitude;
         //   println!("force: {}", force);            
            acc.0 += force;
            other_acc.0 -= force;
        }
        other_bodies.push((mass, acc, sim_pos));
    }
    for (mass, mut acc, _) in query.iter_mut() {
        acc.0 /= mass.0;
    }
}

fn update_velocity(
    mut query: Query<(&mut Velocity, &Acceleration)>,
    time: Res<Time>,
    speed: Res<Speed>,
    pause: Res<Pause>
) {
    if pause.0 {
        return;
    }
    for (mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0 * time.delta_seconds() as f64 * speed.0;
    }
}

pub fn update_position(
    mut query: Query<(Entity, &mut SimPosition, &mut Transform, &Velocity)>,
    time: Res<Time>,
    speed: Res<Speed>,
    selected_entity: Res<SelectedEntity>,
    pause: Res<Pause>,
    mut orbit_offset: ResMut<OrbitOffset>,
) {
    if pause.0 {
        return;
    }
    let delta_time = time.delta_seconds() as f64;
    // Calculate the offset based on the selected entity's position
    let offset = match selected_entity.0 {
        Some(selected) => {
            if let Ok((_, mut sim_pos, mut transform, vel)) = query.get_mut(selected) {
                sim_pos.0 += vel.0 * delta_time * speed.0; //this is the same step as below, but we are doing this first for the offset
                let raw_translation = sim_pos.0 * M_TO_UNIT;
                transform.translation = Vec3::ZERO; //the selected entity will always be at 0,0,0
                -raw_translation 
            } else {
                DVec3::ZERO 
            }
        }
        None => DVec3::ZERO,
    };
    for (entity, mut sim_pos, mut transform, vel) in query.iter_mut() {
        if let Some(s_entity) = selected_entity.0 {
            if s_entity == entity {
                continue;
            }
        }
        sim_pos.0 += vel.0 * delta_time * speed.0;
        let pos_without_offset = sim_pos.0.as_vec3() * M_TO_UNIT as f32;
        transform.translation = pos_without_offset + offset.as_vec3(); //apply offset
    }
    orbit_offset.0 = offset.as_vec3();
}