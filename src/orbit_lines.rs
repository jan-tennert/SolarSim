use bevy::{prelude::{Plugin, App, Resource, Res, Entity, Query, Transform, Update, IntoSystemConfigs, in_state, Vec3, Material, Color, Commands, MaterialMeshBundle, default, Assets, Gizmos, With, Without, ResMut}, time::{Timer, TimerMode, Time}, render::{render_resource::{PrimitiveTopology, RenderPipelineDescriptor, SpecializedMeshPipelineError, PolygonMode, ShaderRef, AsBindGroup}, mesh::MeshVertexBufferLayout}, pbr::{MaterialPipeline, MaterialPipelineKey}, reflect::{TypePath, TypeUuid}};

use crate::{body::{OrbitSettings, Planet, Moon, Star, BodyChildren, SimPosition}, SimState, physics::update_position, selection::SelectedEntity, constants::M_TO_UNIT};

pub struct OrbitLinePlugin;

impl Plugin for OrbitLinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OrbitOffset>()
            .init_resource::<OffsetTimer>()
            .add_systems(Update, (update_lines.after(update_position), draw_orbit_line.after(update_lines), clean_orbit_lines.after(draw_orbit_line)).run_if(in_state(SimState::Simulation)));
    }
}

#[derive(Resource, Default)]
pub struct OrbitOffset(pub Vec3);

#[derive(Resource)]
struct OffsetTimer(Timer);

impl Default for OffsetTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.005, TimerMode::Repeating))
    }
}

fn update_lines(
    mut planet_query: Query<(&mut OrbitSettings, &SimPosition, &BodyChildren, With<Planet>, Without<Moon>, Without<Star>)>,
    mut moon_query: Query<(Entity, &SimPosition, &mut OrbitSettings, With<Moon>, Without<Planet>, Without<Star>)>,
    time: Res<Time>,
    mut timer: ResMut<OffsetTimer>
) {
    timer.0.tick(time.delta());
    
    if timer.0.finished() {
        for (mut orbit, pos, _, _, _, _) in &mut planet_query {
            if orbit.draw_lines {
                orbit.lines.push((pos.0 * M_TO_UNIT).as_vec3());
            }
        }
        for (entity, pos, mut orbit, _, _, _) in &mut moon_query {
            if orbit.draw_lines {
                if let Some((_, p_pos, _, _, _, _)) = planet_query.iter().find(|(_, _, children, _, _, _)| {
                    children.0.contains(&entity)
                }) {
                    let raw_p_pos = (p_pos.0 * M_TO_UNIT).as_vec3();
                    let raw_pos = (pos.0 * M_TO_UNIT).as_vec3();
                    orbit.lines.push(raw_pos - raw_p_pos);
                }
            }
        }
    }
}

fn draw_orbit_line(
    offset: Res<OrbitOffset>,
    planet_query: Query<(&OrbitSettings, &SimPosition, &BodyChildren, With<Planet>, Without<Moon>, Without<Star>)>,
    moon_query: Query<(Entity, &OrbitSettings, With<Moon>, Without<Planet>, Without<Star>)>,
    mut gizmos: Gizmos
) {
    for (orbit, _, _, _, _, _) in &planet_query {
        if orbit.draw_lines {
            draw_lines(orbit, offset.0, &mut gizmos)
        }
    }
    for (entity, orbit, _, _, _) in &moon_query {
        if orbit.draw_lines {
            if let Some((_, p_pos, _, _, _, _)) = planet_query.iter().find(|(_, _, children, _, _, _)| {
                children.0.contains(&entity)
            }) {
                let raw_p_pos = (p_pos.0 * M_TO_UNIT).as_vec3();
                draw_lines(orbit, offset.0 + raw_p_pos, &mut gizmos)
            }
        }
    }
}

pub fn draw_lines(orbit: &OrbitSettings, offset: Vec3, gizmos: &mut Gizmos) {
    for (index, first) in orbit.lines.iter().enumerate() {
        if let Some(second) = orbit.lines.get(index + 1) {
            gizmos.line(*first + offset, *second + offset, orbit.color);
        }
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
