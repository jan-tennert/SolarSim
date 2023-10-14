use std::time::Duration;

use bevy::{prelude::{Plugin, App, Resource, Res, Entity, Query, Transform, Update, IntoSystemConfigs, in_state, Mesh, Vec3, Material, Color, Commands, MaterialMeshBundle, default, Assets, Gizmos, With, Without}, time::{Timer, TimerMode, Time}, render::{render_resource::{PrimitiveTopology, RenderPipelineDescriptor, SpecializedMeshPipelineError, PolygonMode, ShaderRef, AsBindGroup}, mesh::MeshVertexBufferLayout}, pbr::{MaterialPipeline, MaterialPipelineKey}, reflect::{TypePath, TypeUuid}};

use crate::{body::{OrbitSettings, Planet, Moon, Star, BodyChildren, SimPosition}, SimState, physics::update_position, selection::SelectedEntity, constants::KM_TO_AU};

pub struct OrbitLinePlugin;

impl Plugin for OrbitLinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OrbitOffset>()
            .add_systems(Update, (update_lines.after(update_position), draw_orbit_line.after(update_lines), clean_orbit_lines.after(draw_orbit_line)).run_if(in_state(SimState::Simulation)));
    }
}

#[derive(Resource, Default)]
pub struct OrbitOffset(pub Vec3);

fn update_lines(
    mut planet_query: Query<(&mut OrbitSettings, &SimPosition, &BodyChildren, With<Planet>, Without<Moon>, Without<Star>)>,
    mut moon_query: Query<(Entity, &SimPosition, &mut OrbitSettings, With<Moon>, Without<Planet>, Without<Star>)>,
) {
    for (mut orbit, pos, _, _, _, _) in &mut planet_query {
        if orbit.draw_lines {
            orbit.lines.push((pos.0 * KM_TO_AU).as_vec3());
        }
    }
    for (entity, pos, mut orbit, _, _, _) in &mut moon_query {
        if orbit.draw_lines {
            if let Some((_, p_pos, _, _, _, _)) = planet_query.iter().find(|(_, _, children, _, _, _)| {
                children.0.contains(&entity)
            }) {
                let raw_p_pos = (p_pos.0 * KM_TO_AU).as_vec3();
                let raw_pos = (pos.0 * KM_TO_AU).as_vec3();
                orbit.lines.push(raw_pos - raw_p_pos);
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
                let raw_p_pos = (p_pos.0 * KM_TO_AU).as_vec3();
                draw_lines(orbit, offset.0 + raw_p_pos, &mut gizmos)
            }
        }
    }
}

pub fn draw_lines(orbit: &OrbitSettings, offset: Vec3, gizmos: &mut Gizmos) {
    let points: Vec<&[Vec3]> = orbit.lines.chunks(2).collect();
    for outer in points {
        if let Some(first) = outer.get(0) {
            if let Some(second) = outer.get(1) {
                gizmos.line(*first + offset, *second + offset, orbit.color);
            }
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
