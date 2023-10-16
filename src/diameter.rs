use bevy::{app::{App, Plugin}, prelude::{Query, Transform, Res, Entity, PreUpdate, Local, GizmoConfig, ResMut, AabbGizmo, GlobalTransform, PostUpdate, Update, With, Handle, Mesh, Vec3, Name, Children, in_state, IntoSystemConfigs}, render::primitives::{Aabb, Sphere}, math::Vec3A, scene::{SceneSpawner, SceneInstance}};

use crate::{body::Diameter, SimState, constants::M_TO_UNIT};
pub struct DiameterPlugin;

impl Plugin for DiameterPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, apply_real_diameter.run_if(in_state(SimState::Simulation)));
    }
    
}

fn apply_real_diameter(
    mut bodies: Query<(&SceneInstance, &mut Diameter, &mut Transform)>,
    meshes: Query<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
    spawner: Res<SceneSpawner>
) {
    for (instance, mut diameter, mut transform) in &mut bodies {
        if diameter.applied {
            continue;
        }
        for (g_transform, maybe_abb) in meshes.iter_many(spawner.iter_instance_entities(**instance)) {
            if let Some(aabb) = maybe_abb {
                let sphere = Sphere {
                center: Vec3A::from(g_transform.transform_point(Vec3::from(aabb.center))),
                    radius: g_transform.radius_vec3a(aabb.half_extents),
                };
                let aabb = Aabb::from(sphere);
                transform.scale = Vec3::splat((diameter.num * M_TO_UNIT) as f32) / (Vec3::from(aabb.half_extents));
                diameter.applied = true;
            }
        }
    }
}