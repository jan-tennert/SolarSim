use bevy::app::{App, Plugin};
use bevy::prelude::{Resource, Query, Transform, Res, Quat, IntoSystemConfigs, Entity, in_state, Update, Without, With};
use bevy::time::Time;

use crate::SimState;
use crate::body::{RotationSpeed, Diameter, Star, Moon, Planet, BodyChildren};
use crate::physics::{update_position, Pause};
use crate::speed::Speed;


pub struct RotationPlugin;

impl Plugin for RotationPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (rotate_bodies.after(update_position)).run_if(in_state(SimState::Simulation)));
    }

}

fn rotate_bodies(
    mut planet_query: Query<(&RotationSpeed, &mut Transform, &Diameter, With<Planet>, Without<Star>, Without<Moon>)>,
    time: Res<Time>,
    speed: Res<Speed>,
    pause: Res<Pause>
) {     
    if !pause.0 {
        for (rotation_speed, mut transform, radius, _, _, _) in &mut planet_query {
        //    if rotation_speed.0 == 0.0 || radius.0 == 0.0 {
      //          continue;
     //       }
       //     let radians_per_second = rotation_speed.0 / (3.6 * radius.0);
      //      let actual_speed = radians_per_second * time.delta_seconds() * (speed.0 as f32);
       //     transform.rotate(Quat::from_rotation_z(actual_speed));
        }
    }
}