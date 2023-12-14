use bevy::{app::{App, Plugin}, math::Vec3A, prelude::{Children, GlobalTransform, Handle, in_state, IntoSystemConfigs, Mesh, Query, Res, ResMut, Transform, Update, Vec3, With}, render::primitives::{Aabb, Sphere}, scene::{SceneInstance, SceneSpawner}};
use bevy::prelude::AssetServer;

use crate::{body::{Diameter, Scale}, constants::M_TO_UNIT, loading::LoadingState, SimState};
use crate::body::SceneHandle;

pub struct DiameterPlugin;

impl Plugin for DiameterPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, apply_real_diameter.run_if(in_state(SimState::Loading)));
    }
    
}

fn apply_real_diameter(
    mut bodies: Query<(&Children, &SceneHandle, &mut Diameter, &mut Transform, &mut Scale)>,
    scenes: Query<&SceneInstance>,
    meshes: Query<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
    spawner: Res<SceneSpawner>,
    mut loading_state: ResMut<LoadingState>,
    asset_server: Res<AssetServer>,
) {
    if !bodies.is_empty() && bodies.iter().all(|(_, _, diameter, _, _)| {
        diameter.applied
    }) {
        loading_state.scaled_bodies = true;
    }
    for (children, handle, mut diameter, mut transform, mut scale) in &mut bodies {
        if diameter.applied || asset_server.get_load_state(&handle.0) != Some(bevy::asset::LoadState::Loaded) {
            continue;
        }
        for children in children {
            if let Ok(scene) = scenes.get(*children) {
                if !spawner.instance_is_ready(**scene) {
                    continue;
                }
                let m = meshes.iter_many(spawner.iter_instance_entities(**scene));
                let mut min = Vec3A::splat(f32::MAX);
                let mut max = Vec3A::splat(f32::MIN);
                for (g_transform, maybe_abb) in m {
                    if let Some(aabb) = maybe_abb {
                        let sphere = Sphere {
                            center: Vec3A::from(g_transform.transform_point(Vec3::from(aabb.center))),
                            radius: g_transform.radius_vec3a(aabb.half_extents),
                        };
                        let aabb = Aabb::from(sphere);
                        min = min.min(aabb.min());
                        max = max.max(aabb.max());
                    }
                }
                let aabb = Aabb::from_min_max(Vec3::from(min), Vec3::from(max));
                transform.scale = Vec3::splat((diameter.num * M_TO_UNIT / 2.0) as f32) / (Vec3::from(aabb.half_extents)); //not dividing by 2 for the diameter makes them to big which doesn't work with satellites very close to their planet
                scale.0 = transform.scale.x;
                diameter.applied = true;
                loading_state.scaled_bodies_count += 1;
            }
        }
    }
}