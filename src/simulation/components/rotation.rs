use std::f32::consts::PI;

use crate::constants::DAY_IN_SECONDS;
use crate::simulation::components::body::{BodyRotation, BodyShape, RotationSpeed, Star};
use crate::simulation::components::speed::Speed;
use crate::simulation::integration::{paused, SubSteps};
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::scenario::setup::setup_scenario;
use crate::simulation::SimState;
use crate::utils::sim_state_type_simulation;
use bevy::app::{App, Plugin};
use bevy::prelude::{in_state, not, Children, IntoScheduleConfigs, Quat, Query, Res, ResMut, Transform, Update, With, Without};
use bevy::scene::SceneInstance;
use bevy::time::Time;

pub struct RotationPlugin;

impl Plugin for RotationPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (initial_rotation.after(setup_scenario)).run_if(in_state(SimState::Loading)))
            .add_systems(Update, (rotate_bodies).run_if(sim_state_type_simulation).run_if(not(paused)));
    }

}

pub fn initial_rotation(
    mut query: Query<(&mut BodyRotation, &BodyShape, &Children)>,
    mut scenes: Query<&mut Transform, (With<SceneInstance>, Without<Star>)>,
    mut loading_state: ResMut<LoadingState>,
) {
    if query.iter().all(|(tilt, _, _)| tilt.applied) {
        loading_state.tilted_bodies = true;
    }
    for (mut rotation, diameter, children) in &mut query {
        if rotation.applied || !diameter.applied {
            continue;
        }
        for child in children.iter() {
            if let Ok(mut transform) = scenes.get_mut(*child) {
                transform.rotation = Quat::from_mat3(&rotation.matrix);
                rotation.applied = true;
                break;
            }
        }
    }
}

fn rotate_bodies(
    query: Query<(&RotationSpeed, &BodyShape, &BodyRotation, &Children)>,
    mut scenes: Query<&mut Transform, With<SceneInstance>>,
    time: Res<Time>,
    speed: Res<Speed>,
    sub_steps: Res<SubSteps>,
) {
        for (rotation_speed, _diameter, tilt, children) in &query {
            if rotation_speed.0 == 0.0 || !tilt.applied {
                continue;
            }
            
            let speed_modifier = ((speed.0 as f32) * (sub_steps.0 as f32)) / DAY_IN_SECONDS;
            let rotation_duration = rotation_speed.0 * 60.0;
            let rotations_per_day = DAY_IN_SECONDS / (rotation_duration as f32);
            
            for child in children.iter() {
                if let Ok(mut transform) = scenes.get_mut(*child) {
                   transform.rotation = transform.rotation * Quat::from_rotation_y(2.0 * PI * (rotations_per_day * time.delta_secs() * speed_modifier));
                }
            }
            
        }

}