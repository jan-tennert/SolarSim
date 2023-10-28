use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

use crate::constants::DEFAULT_TIMESTEP;

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
            
        if speed_in_seconds < 1.0 {
            return format!("{:.2} s", speed_in_seconds);
        } else if speed_in_seconds < 60.0 {
            return format!("{:.2} s", speed_in_seconds);
        } else if speed_in_seconds < 3600.0 {
            let minutes = speed_in_seconds / 60.0;
            return format!("{:.2} min", minutes);
        } else if speed_in_seconds < 86400.0 {
            let hours = speed_in_seconds / 3600.0;
            return format!("{:.2} hours", hours);
        } else if speed_in_seconds < 2592000.0 {
            let days = speed_in_seconds / 86400.0;
            return format!("{:.2} days", days);
        } else if speed_in_seconds < 31536000.0 {
            let months = speed_in_seconds / 2592000.0;
            return format!("{:.2} months", months);
        } else {
            let years = speed_in_seconds / 31536000.0;
            return format!("{:.2} years", years);
        }
    }
        
}

impl Default for Speed {

    fn default() -> Self {
        Self(DEFAULT_TIMESTEP)
    }

}