use bevy::{app::{App, Plugin}, prelude::{Query, Transform, OnEnter, Res,     Entity, IntoSystemConfigs, PreUpdate, in_state, Local, GizmoConfig, ResMut, AabbGizmo, GlobalTransform, PostUpdate, Update, With, Handle, Mesh, Vec3, Name}, render::primitives::{Aabb, Sphere}, math::Vec3A, scene::{SceneSpawner, SceneInstance}};

use crate::{body::Scale, SimState, setup::setup_planets};
pub struct DiameterPlugin;

impl Plugin for DiameterPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, apply_real_diameter.run_if(in_state(SimState::Simulation)));
    }
    
}

fn apply_real_diameter(
    mut setup: Local<bool>,
    mut bodies: Query<(&SceneInstance, &Scale, &mut Transform)>,
    meshes: Query<( &GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
    spawner: Res<SceneSpawner>
) {
    if !*setup {  
    for (instance, diameter, mut transform) in bodies.iter_mut() {
            let mut min = Vec3::splat(f32::MAX);
            let mut max = Vec3::splat(f32::MIN);

            for (g_transform, maybe_abb) in meshes.iter_many(spawner.iter_instance_entities(**instance)) {
                if let Some(_) = maybe_abb {
                    // Calculate the AABB based on the sphere representing the planet
                    let sphere_center = g_transform.translation();
                    let sphere_radius = diameter.0 * 0.5; // Half of the real diameter

                    let aabb_min = sphere_center - Vec3::splat(sphere_radius);
                    let aabb_max = sphere_center + Vec3::splat(sphere_radius);

                    min = min.min(aabb_min);
                    max = max.max(aabb_max);
                }
            }

            let extents = (max - min) * 0.5; // Half extents
            transform.scale = Vec3::splat(diameter.0) / extents;

            // Set setup to true to prevent repeated scaling
         //   *setup = true;
        }
    }
}