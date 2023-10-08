use std::time::Duration;

use bevy::{prelude::{Plugin, App, Resource, Res, ResMut, Query, Transform}, time::{Timer, TimerMode, Time}};

use crate::body::{OrbitLines, DrawOrbitLines};

pub struct OrbitLinePlugin;

impl Plugin for OrbitLinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OrbitTimer>()
            ;
    }
}

#[derive(Resource)]
struct OrbitTimer(pub Timer);

impl Default for OrbitTimer {
    
    fn default() -> Self {
        OrbitTimer(Timer::new(Duration::from_millis(500), TimerMode::Repeating))
    }
    
}

fn add_orbit_lines(
    mut timer: ResMut<OrbitTimer>,
    time: Res<Time>,
    bodies: Query<(&OrbitLines, &DrawOrbitLines, &Transform)>
) {
    timer.0.tick(time.delta());
    
    if timer.0.finished() {
        for (orbit_lines, draw, transform) in &bodies {
            if draw.0 {
                
            }
        }
    }
}