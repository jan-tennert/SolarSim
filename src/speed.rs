use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

use crate::constants::DEFAULT_TIMESTEP;
use crate::unit::format_seconds;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<Speed>();
    }

}

#[derive(Resource, Debug)]
pub struct Speed(pub f64); //speed in seconds
    
impl Speed {
        
    pub fn format(&self, sub_steps: i32) -> String {
        let speed_in_seconds = self.0 * (sub_steps as f64);
            
        return format_seconds(speed_in_seconds);
    }
        
}

impl Default for Speed {

    fn default() -> Self {
        Self(DEFAULT_TIMESTEP)
    }

}