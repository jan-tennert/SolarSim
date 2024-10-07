use bevy::prelude::{in_state, App, Camera, Entity, IntoSystemConfigs, Plugin, PreUpdate, Query, Res, Resource, Transform, Vec3, Without};

use crate::simulation::components::body::{BodyChildren, Mass};
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::SimState;

pub struct LockOnPlugin;

impl Plugin for LockOnPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .init_resource::<LockOn>()
        .add_systems(PreUpdate, (lock_on).run_if(in_state(SimState::Loaded)));
    }
    
}

#[derive(Resource, Default)]
pub struct LockOn {
    
    pub enabled: bool,
    
}

fn lock_on(
    lock_on: Res<LockOn>,
    mut query: Query<(Entity, &Transform, Option<&BodyChildren>), Without<Camera>>,
    mut camera: Query<(&Camera, &mut Transform), (Without<Mass>, Without<BodyChildren>)>,
    selected_entity: Res<SelectedEntity>
) {
    if !lock_on.enabled {
        return;
    }
    if let Some(s_entity) = selected_entity.entity {
        let mut parent: Option<&Transform> = None;
        for (_, transform, children) in query.iter_mut() {
            if let Some(children) = children {
                if children.0.contains(&s_entity) {
                    parent = Some(transform);
                }
            } 
        }
        if let Some(p_transform) = parent {
            let (_, mut c_transform) = camera.single_mut();   
            c_transform.look_at(p_transform.translation, Vec3::X);
        }
    }       
}