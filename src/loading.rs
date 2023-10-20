use bevy::{app::{App, Plugin}, prelude::{Query, Transform, Res, Entity, PreUpdate, Local, GizmoConfig, ResMut, AabbGizmo, GlobalTransform, PostUpdate, Update, With, Handle, Mesh, Vec3, Name, Children, in_state, IntoSystemConfigs, Visibility, Resource, NextState}, render::primitives::{Aabb, Sphere}, math::Vec3A, scene::{SceneSpawner, SceneInstance}};

use crate::{SimState, body::Mass};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadingState>()
            .add_systems(Update, (loading_system).run_if(in_state(SimState::Loading)));
    }
}

#[derive(Resource, Default)]
pub struct LoadingState {
    
    pub loaded_bodies: bool,
    pub scaled_bodies: bool
             
}

impl LoadingState {
    
    pub fn reset(&mut self) {
        self.loaded_bodies = false;
        self.scaled_bodies = false;
    }
    
    pub fn is_done(&self) -> bool {
        return self.loaded_bodies && self.scaled_bodies
    }
    
}

fn loading_system(
    loading_state: ResMut<LoadingState>,
    mut sim_state: ResMut<NextState<SimState>>,
) {
    if loading_state.is_done() {
        sim_state.set(SimState::Simulation)
    }
}