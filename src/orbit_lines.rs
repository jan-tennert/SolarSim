use std::time::Duration;

use bevy::{prelude::{Plugin, App, Resource, Res, ResMut, Query, Transform, Update, IntoSystemConfigs, in_state, Mesh, Vec3, Material, Color, Commands, MaterialMeshBundle, default, Assets, Gizmos}, time::{Timer, TimerMode, Time}, render::{render_resource::{PrimitiveTopology, RenderPipelineDescriptor, SpecializedMeshPipelineError, PolygonMode, ShaderRef, AsBindGroup}, mesh::MeshVertexBufferLayout}, pbr::{MaterialPipeline, MaterialPipelineKey}, reflect::{TypePath, TypeUuid}};

use crate::{body::OrbitSettings, SimState, physics::update_position};

pub struct OrbitLinePlugin;

impl Plugin for OrbitLinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OrbitTimer>()
            .add_systems(Update, (clean_orbit_lines.after(update_position)).run_if(in_state(SimState::Simulation)));
    }
}

#[derive(Resource)]
struct OrbitTimer(pub Timer);

impl Default for OrbitTimer {
    
    fn default() -> Self {
        OrbitTimer(Timer::new(Duration::from_millis(500), TimerMode::Repeating))
    }
    
}

fn clean_orbit_lines(
    mut bodies: Query<(&mut OrbitSettings, &Transform)>,
) {
    for (mut orbit, _) in &mut bodies {
        if orbit.draw_lines {
            let amount = orbit.lines.len() as i32;
            if amount > orbit.max_points {
                while orbit.lines.len() as i32 >= orbit.max_points {
                    orbit.lines.remove(0);
                }
            }
        }
    }
}
