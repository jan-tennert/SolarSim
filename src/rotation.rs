use bevy::app::{App, Plugin};
use bevy::prelude::{Resource, Query, Transform, Res, Quat, IntoSystemConfigs, Entity, in_state, Update, Without, With, Vec3};
use bevy::time::Time;

use crate::SimState;
use crate::body::{RotationSpeed, Diameter, Star, Moon, Planet, BodyChildren, AxialTilt};
use crate::physics::{update_position, Pause};
use crate::speed::Speed;


pub struct RotationPlugin;

impl Plugin for RotationPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (axial_tilt_planets).run_if(in_state(SimState::Simulation)));
    }

}

fn axial_tilt_planets(
    mut query: Query<(Entity, &mut AxialTilt, &mut Transform, With<Planet>, Without<Star>, Without<Moon>)>,
    parents_query: Query<(&Transform, &BodyChildren, With<Star>, Without<Planet>, Without<Moon>)>
) {
    for (entity, mut tilt, mut transform, _, _, _) in &mut query {
        if tilt.applied {
            continue;
        }
        
        let parent = parents_query.iter().find(|(_, children, _, _, _)| {
            children.0.contains(&entity)
        });
        if let Some((p_transform, _, _, _, _)) = parent {
            transform.look_at(p_transform.translation, Vec3::NEG_Z);
            tilt.applied = true;
        }
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