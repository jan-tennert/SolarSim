use std::f32::consts::PI;

use bevy::app::{App, Plugin};
use bevy::hierarchy::Children;
use bevy::math::Dir3;
use bevy::prelude::{in_state, IntoSystemConfigs, Quat, Query, Res, ResMut, Transform, Update, Vec3, With, Without};
use bevy::scene::SceneInstance;
use bevy::time::Time;

use crate::simulation::components::body::{BodyRotation, Diameter, RotationSpeed, Star};
use crate::constants::DAY_IN_SECONDS;
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::components::physics::{Pause, SubSteps};
use crate::simulation::scenario::setup::setup_scenario;
use crate::simulation::SimState;
use crate::simulation::components::speed::Speed;
use crate::utils::{sim_state_type_editor, sim_state_type_simulation};

pub struct RotationPlugin;

impl Plugin for RotationPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (axial_tilt.after(setup_scenario)).run_if(in_state(SimState::Loading)))
            .add_systems(Update, (rotate_bodies).run_if(sim_state_type_simulation));
    }

}

pub fn axial_tilt(
    mut query: Query<(&mut BodyRotation, &Diameter, &Children)>,
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
        println!("Applying axial tilt to body");
        for child in children.iter() {
            if let Ok(mut transform) = scenes.get_mut(*child) {
                /*transform.rotate_x(PI / 2.0);
                let tilted = Quat::from_axis_angle(Vec3::X, tilt.num.to_radians()) * Vec3::Z;
                transform.rotate_x(tilt.num.to_radians());
                tilt.axis = Some(tilted);*/
                transform.rotation = Quat::from_mat3(&rotation.matrix);
                rotation.axis = transform.translation * Vec3::Z;
                rotation.applied = true;
                break;
            }
        }
    }
}

fn rotate_bodies(
    query: Query<(&RotationSpeed, &Diameter, &BodyRotation, &Children)>,
    mut scenes: Query<&mut Transform, With<SceneInstance>>,
    time: Res<Time>,
    speed: Res<Speed>,
    sub_steps: Res<SubSteps>,
    pause: Res<Pause>,
) {     
    if !pause.0 {
        for (rotation_speed, diameter, tilt, children) in &query {
            if rotation_speed.0 == 0.0 || diameter.num == 0.0 || !tilt.applied {
                continue;
            }
            
            let speed_modifier = ((speed.0 as f32) * (sub_steps.0 as f32)) / DAY_IN_SECONDS;
            let rotation_duration = rotation_speed.0 * 60.0;
            let rotations_per_day = DAY_IN_SECONDS / (rotation_duration as f32);
            
            for child in children.iter() {
                if let Ok(mut transform) = scenes.get_mut(*child) {
                //    transform.rotate_z(2.0 * PI * (rotations_per_day * time.delta_seconds() * speed_modifier));
                    transform.rotate(Quat::from_axis_angle(tilt.axis, 2.0 * PI * (rotations_per_day * time.delta_seconds() * speed_modifier)));
                }
            }
            
        }
    }
}