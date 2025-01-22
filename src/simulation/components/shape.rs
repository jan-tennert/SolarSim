use crate::simulation::components::body::BodyShape;
use crate::simulation::components::body::SceneHandle;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::ui::toast::{important_error_toast, ToastContainer};
use crate::simulation::SimState;
use bevy::asset::LoadState;
use bevy::ecs::query::QueryManyIter;
use bevy::prelude::{AssetServer, Entity, Mesh3d, Name, Resource};
use bevy::utils::HashMap;
use bevy::{app::{App, Plugin}, math::Vec3A, prelude::{in_state, Children, GlobalTransform, IntoSystemConfigs, Query, Res, ResMut, Transform, Update, Vec3, With}, render::primitives::{Aabb, Sphere}, scene::{SceneInstance, SceneSpawner}};

pub struct DiameterPlugin;

impl Plugin for DiameterPlugin {
    
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SavedAabbs>()
            .add_systems(Update, apply_real_diameter.run_if(in_state(SimState::Loading)));
    }
    
}

#[derive(Default, Resource)]
pub struct SavedAabbs(HashMap<String, Aabb>);

pub fn apply_real_diameter(
    mut bodies: Query<(&Children, &Name, &SceneHandle, &mut BodyShape, &mut Transform)>,
    scenes: Query<&SceneInstance>,
    meshes: Query<(&GlobalTransform, Option<&Aabb>), With<Mesh3d>>,
    spawner: Res<SceneSpawner>,
    mut loading_state: ResMut<LoadingState>,
    asset_server: Res<AssetServer>,
    s_scale: Res<SimulationScale>,
    mut toasts: ResMut<ToastContainer>,
    mut aabbs: ResMut<SavedAabbs>
) {
    if !bodies.is_empty() && bodies.iter().all(|(_, _, _, diameter, _)| {
        diameter.applied
    }) {
        loading_state.scaled_bodies = true;
    }
    for (children, name, handle, mut shape, mut transform) in &mut bodies {
        let load_state = asset_server.get_load_state(&handle.0).unwrap_or(LoadState::NotLoaded);
        if shape.applied || !load_state.is_loaded() {
            if !shape.applied {
                match asset_server.get_load_state(&handle.0) {
                    None => {}
                    Some(a) => {
                        match a {
                            LoadState::NotLoaded => {}
                            LoadState::Loading => {}
                            LoadState::Loaded => {}
                            LoadState::Failed(e) => {
                                toasts.0.add(important_error_toast(format!("Failed to load asset for body '{}': {}", name.as_str(), e).as_str()));
                                shape.applied = true;
                            }
                        }
                    }
                }
            }
            continue;
        }
        for children in children {
            if let Ok(scene) = scenes.get(*children) {
                if !spawner.instance_is_ready(**scene) {
                    continue;
                }
                let aabb = if let Some(aabb) = aabbs.0.get(&shape.path.clone()) {
                    *aabb
                } else {
                    let m = meshes.iter_many(spawner.iter_instance_entities(**scene));
                    let aabb = calculate_aabb(m);
                    aabbs.0.insert(shape.path.clone(), aabb);
                    aabb
                };
                let semi_major_radius_units = s_scale.m_to_unit_32((shape.ellipsoid.semi_major_equatorial_radius_km * 1000.0) as f32); // km to meters
                let semi_minor_radius_units = s_scale.m_to_unit_32((shape.ellipsoid.semi_minor_equatorial_radius_km * 1000.0) as f32);
                let polar_radius_units = s_scale.m_to_unit_32((shape.ellipsoid.polar_radius_km * 1000.0) as f32);
                transform.scale = Vec3::new(
                    semi_major_radius_units / (aabb.half_extents.x * 2.0),
                    polar_radius_units / (aabb.half_extents.y * 2.0),
                    semi_minor_radius_units / (aabb.half_extents.z * 2.0)
                );
                shape.applied = true;
                loading_state.scaled_bodies_count += 1;
            }
        }
    }
}

fn calculate_aabb(meshes: QueryManyIter<(&GlobalTransform, Option<&Aabb>), With<Mesh3d>, impl Iterator<Item=Entity> + Sized>) -> Aabb {
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