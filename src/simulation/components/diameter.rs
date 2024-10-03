use bevy::{app::{App, Plugin}, math::Vec3A, prelude::{in_state, Children, GlobalTransform, Handle, IntoSystemConfigs, Mesh, Query, Res, ResMut, Transform, Update, Vec3, With}, render::primitives::{Aabb, Sphere}, scene::{SceneInstance, SceneSpawner}};
use bevy::ecs::query::QueryManyIter;
use bevy::prelude::{AssetServer, Entity, Name};
use crate::simulation::SimState;
use crate::simulation::components::body::SceneHandle;
use crate::simulation::components::body::{Diameter, Scale};
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::scenario::loading::LoadingState;

pub struct DiameterPlugin;

impl Plugin for DiameterPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, apply_real_diameter.run_if(in_state(SimState::Loading)));
    }
    
}

pub fn apply_real_diameter(
    mut bodies: Query<(&Children, &Name, &SceneHandle, &mut Diameter, &mut Transform, &mut Scale)>,
    scenes: Query<&SceneInstance>,
    meshes: Query<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
    spawner: Res<SceneSpawner>,
    mut loading_state: ResMut<LoadingState>,
    asset_server: Res<AssetServer>,
    s_scale: Res<SimulationScale>
) {
    if !bodies.is_empty() && bodies.iter().all(|(_, _, _, diameter, _, _)| {
        diameter.applied
    }) {
        loading_state.scaled_bodies = true;
    }
    for (children, name, handle, mut diameter, mut transform, mut scale) in &mut bodies {
        if diameter.applied || asset_server.get_load_state(&handle.0) != Some(bevy::asset::LoadState::Loaded) {
            continue;
        }
        for children in children {
            if let Ok(scene) = scenes.get(*children) {
                if !spawner.instance_is_ready(**scene) {
                    continue;
                }
                let aabb = if let Some(aabb) = diameter.aabb {
                    aabb
                } else {
                    let m = meshes.iter_many(spawner.iter_instance_entities(**scene));
                    let aabb = calculate_aabb(m);
                    diameter.aabb = Some(aabb);
                    aabb
                };
                transform.scale = Vec3::splat(s_scale.m_to_unit_32(diameter.num) / 1.7) / (Vec3::from(aabb.half_extents)); //not dividing by 1.7 for the diameter makes them to big which doesn't work with satellites very close to their planet
                scale.0 = transform.scale.x;
                diameter.applied = true;
                loading_state.scaled_bodies_count += 1;
            }
        }
    }
}

fn calculate_aabb(meshes: QueryManyIter<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>, impl Iterator<Item=Entity> + Sized>) -> Aabb {
    let mut min = Vec3A::splat(f32::MAX);
    let mut max = Vec3A::splat(f32::MIN);
    for (g_transform, maybe_abb) in meshes {
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
    return Aabb::from_min_max(Vec3::from(min), Vec3::from(max));
}