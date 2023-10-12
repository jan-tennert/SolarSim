use std::time::Duration;

use bevy::{prelude::{Plugin, App, Resource, Res, ResMut, Query, Transform, Update, IntoSystemConfigs, in_state, Mesh, Vec3, Material, Color, Commands, MaterialMeshBundle, default, Assets, Gizmos}, time::{Timer, TimerMode, Time}, render::{render_resource::{PrimitiveTopology, RenderPipelineDescriptor, SpecializedMeshPipelineError, PolygonMode, ShaderRef, AsBindGroup}, mesh::MeshVertexBufferLayout}, pbr::{MaterialPipeline, MaterialPipelineKey}, reflect::{TypePath, TypeUuid}};

use crate::{body::{OrbitLines, DrawOrbitLines, MaxOrbitPoints}, SimState, physics::update_position};

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
    mut bodies: Query<(&mut OrbitLines, &DrawOrbitLines, &MaxOrbitPoints, &Transform)>,
) {
    for (mut orbit_lines, draw, max_orbit_points, _) in &mut bodies {
        if draw.0 {
            let amount = orbit_lines.0.len() as i32;
            if amount > max_orbit_points.0 {
                while orbit_lines.0.len() as i32 >= max_orbit_points.0 {
                    orbit_lines.0.remove(0);
                }
            }
        }
    }
}
