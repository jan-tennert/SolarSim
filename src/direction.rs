use bevy::{app::{App, Plugin}, math::Vec3A, prelude::{Entity, Children, GlobalTransform, Handle, in_state, IntoSystemConfigs, Mesh, Query, Res, ResMut, Transform, Update, Vec3, With, Gizmos, Color, Without}, render::primitives::{Aabb, Sphere}, scene::{SceneInstance, SceneSpawner}};
use bevy::prelude::AssetServer;

use crate::{body::{Diameter, Scale, OrbitSettings, Velocity, Planet, Moon, BodyChildren}, constants::M_TO_UNIT, loading::LoadingState, SimState, arrows::ArrowGizmos, camera::{PanOrbitCamera, pan_orbit_camera}, physics::apply_physics};
use crate::body::SceneHandle;

pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (display_force_and_velocity.after(pan_orbit_camera)).run_if(in_state(SimState::Simulation)));
    }
    
}

fn display_force_and_velocity(
    planet_query: Query<(&Transform, &BodyChildren, &OrbitSettings, &Diameter, &Velocity), With<Planet>>,
    moon_query: Query<(Entity, &Transform, &OrbitSettings, &Diameter, &Velocity), With<Moon>>,
    mut gizmos: Gizmos
) {
    for (transform, _, orbit, diameter, velocity) in &planet_query {
        if orbit.display_force {
            gizmos.arrow(transform.translation, transform.translation + (orbit.force_direction.normalize() * (diameter.num * M_TO_UNIT)).as_vec3(), Color::BLUE);
        }
        if orbit.display_velocity {
            gizmos.arrow(transform.translation, transform.translation +(velocity.0.normalize() * (diameter.num * M_TO_UNIT)).as_vec3(), Color::RED);
        }
    }
    for (entity, transform, orbit, diameter, velocity) in &moon_query {
        if orbit.display_force {
            gizmos.arrow(transform.translation, transform.translation +(orbit.force_direction.normalize() * (diameter.num * M_TO_UNIT)).as_vec3(), Color::BLUE);
        }
        if orbit.display_velocity {
            if let Some((_, _, _, _, vel)) = planet_query.iter().find(|(_, ch, _, _, _)| ch.0.contains(&entity)) {
                gizmos.arrow(transform.translation, transform.translation +((velocity.0 - vel.0).normalize() * (diameter.num * M_TO_UNIT)).as_vec3(), Color::RED);                
            }
        }
    }
}