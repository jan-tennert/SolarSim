use std::f32::consts::PI;

use bevy::app::{App, Plugin};
use bevy::hierarchy::Children;
use bevy::prelude::{Entity, in_state, IntoSystemConfigs, Quat, Query, Res, Transform, Update, Vec3, With, Without};
use bevy::scene::SceneInstance;
use bevy::time::Time;

use crate::body::{AxialTilt, BodyChildren, Diameter, Moon, Planet, RotationSpeed, Star};
use crate::constants::DAY_IN_SECONDS;
use crate::physics::{Pause, SubSteps};
use crate::SimState;
use crate::speed::Speed;

pub struct RotationPlugin;

impl Plugin for RotationPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (axial_tilt, rotate_bodies).run_if(in_state(SimState::Simulation)));
    }

}

fn axial_tilt(
    mut query: Query<(&mut AxialTilt, &Children)>,
    mut scenes: Query<(&mut Transform, With<SceneInstance>, Without<Star>)>,
) {
    for (mut tilt, children) in &mut query {
        if tilt.applied {
            continue;
        }
        for child in children.iter() {
            if let Ok((mut transform, _, _)) = scenes.get_mut(*child) {
                transform.rotate_x((90.0 as f32).to_radians());
                tilt.applied = true;
                break;
            }
        }
    }
}

fn rotate_bodies(
    query: Query<(&RotationSpeed, &Diameter, &Children)>,
    mut scenes: Query<(&mut Transform, With<SceneInstance>)>,
    time: Res<Time>,
    speed: Res<Speed>,
    sub_steps: Res<SubSteps>,
    pause: Res<Pause>,
) {     
    if !pause.0 {
        for (rotation_speed, diameter, children) in &query {
            if rotation_speed.0 == 0.0 || diameter.num == 0.0 {
                continue;
            }
            
            let speed_modifier = ((speed.0 as f32) * (sub_steps.0 as f32)) / DAY_IN_SECONDS;
            let rotation_duration = rotation_speed.0 * 60.0;
            let rotations_per_day = DAY_IN_SECONDS / (rotation_duration as f32);
            
           // transform.rotate_axis(axis.axis.unwrap(), 2.0 * PI * (rotations_per_day * time.delta_seconds() * speed_modifier));
            for child in children.iter() {
                if let Ok((mut transform, _)) = scenes.get_mut(*child) {
                    transform.rotate_z(2.0 * PI * (rotations_per_day * time.delta_seconds() * speed_modifier));
                }
            }
            
        }
    }
}