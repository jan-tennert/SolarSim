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
            .add_systems(Update, (axial_tilt_planets, rotate_bodies).run_if(in_state(SimState::Simulation)));
    }

}

fn axial_tilt_planets(
    mut query: Query<(Entity, &mut AxialTilt, &Children, With<Planet>, Without<Star>, Without<Moon>)>,
    mut scenes: Query<(&mut Transform, With<SceneInstance>, Without<Star>)>,
    parents_query: Query<(&Transform, &BodyChildren, With<Star>, Without<Planet>, Without<Moon>, Without<SceneInstance>)>
) {
    for (entity, mut tilt, children, _, _, _) in &mut query {
        if tilt.applied {
            continue;
        }
        
        let parent = parents_query.iter().find(|(_, children, _, _, _, _)| {
            children.0.contains(&entity)
        });
        if let Some((p_transform, _, _, _, _, _)) = parent {
            for child in children.iter() {
                if let Ok((mut transform, _, _)) = scenes.get_mut(*child) {
                    let (u, w) = (transform.translation - p_transform.translation)
                        .normalize()
                        .any_orthonormal_pair();
                    //    let u_p = Quat::from_axis_angle(w, tilt.num.to_radians()).mul_vec3(u);
                    let tilted = Quat::from_axis_angle(Vec3::X, tilt.num.to_radians()) * Vec3::Z;
                    //        transform.rotate_axis(u_p, 0.0);
                    transform.rotate_x((90.0 as f32).to_radians());
                    //    transform.rotate_y(tilt.num.to_radians());
                    //    transform.rotate_x(tilt.num.to_radians());
                    tilt.applied = true;
                    tilt.axis = Some(tilted);
                }
            }
        }
    }
}

fn rotate_bodies(
    planet_query: Query<(&RotationSpeed, &AxialTilt, &Diameter, &Children, With<Planet>, Without<Star>, Without<Moon>)>,
    mut scenes: Query<(&mut Transform, With<SceneInstance>)>,
    time: Res<Time>,
    speed: Res<Speed>,
    sub_steps: Res<SubSteps>,
    pause: Res<Pause>,
) {     
    if !pause.0 {
        for (rotation_speed, axis, diameter, children, _, _, _) in &planet_query {
            if rotation_speed.0 == 0.0 || diameter.num == 0.0 || axis.axis.is_none() {
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