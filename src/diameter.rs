use bevy::{app::{App, Plugin}, prelude::{Query, Transform, Res, Entity, PreUpdate, Local, GizmoConfig, ResMut, AabbGizmo, GlobalTransform, PostUpdate, Update, With, Handle, Mesh, Vec3, Name, Children, in_state, IntoSystemConfigs, Visibility}, render::primitives::{Aabb, Sphere}, math::Vec3A, scene::{SceneSpawner, SceneInstance}};

use crate::{body::{Diameter, Scale}, SimState, constants::M_TO_UNIT, loading::LoadingState};

pub struct DiameterPlugin;

impl Plugin for DiameterPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, apply_real_diameter.run_if(in_state(SimState::Loading)));
    }
    
}

fn apply_real_diameter(
    mut bodies: Query<(&SceneInstance, &mut Diameter, &mut Visibility, &mut Transform, &mut Scale)>,
    meshes: Query<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
    spawner: Res<SceneSpawner>,
    mut loading_state: ResMut<LoadingState>
) {
    if !bodies.is_empty() && bodies.iter().all(|(_, diameter, _, _, _)| {
        diameter.applied
    }) {
        loading_state.scaled_bodies = true;
    }
    for (instance, mut diameter, mut visibility, mut transform, mut scale) in &mut bodies {
        if diameter.applied {
            continue;
        }
        let m = meshes.iter_many(spawner.iter_instance_entities(**instance));
        for (g_transform, maybe_abb) in m {
            if let Some(aabb) = maybe_abb {
                let sphere = Sphere {
                center: Vec3A::from(g_transform.transform_point(Vec3::from(aabb.center))),
                    radius: g_transform.radius_vec3a(aabb.half_extents),
                };
                let aabb = Aabb::from(sphere);
                transform.scale = Vec3::splat((diameter.num * M_TO_UNIT) as f32) / (Vec3::from(aabb.half_extents));
                scale.0 = transform.scale.x;
                diameter.applied = true;
            }
        }
    }
}