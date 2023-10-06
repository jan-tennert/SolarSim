use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

use crate::constants::DAY_IN_SECONDS;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<Speed>();
    }

}

    #[derive(Resource, Debug)]
    pub struct Speed(pub f32); //speed in seconds
    
    impl Speed {
        
        pub fn format(&self) -> String {
            let speed_in_seconds = self.0;
            
            if speed_in_seconds < 1.0 {
                return format!("{:.2} s/s", speed_in_seconds);
            } else if speed_in_seconds < 60.0 {
                return format!("{:.2} s/s", speed_in_seconds);
            } else if speed_in_seconds < 3600.0 {
                let minutes = speed_in_seconds / 60.0;
                return format!("{:.2} min/s", minutes);
            } else if speed_in_seconds < 86400.0 {
                let hours = speed_in_seconds / 3600.0;
                return format!("{:.2} hours/s", hours);
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
        Self(DAY_IN_SECONDS)
    }

}