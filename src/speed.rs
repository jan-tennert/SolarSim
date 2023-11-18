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
        
    pub fn small_step_up(&mut self) {
        self.0 *= 2.0; 
    }
        
    pub fn big_step_up(&mut self) {
        self.0 *= 10.0;
    }
        
    pub fn small_step_down(&mut self) {
        self.0 = f64::max(self.0 / 2.0, 1.0);
    }
        
    pub fn big_step_down(&mut self) {
        self.0 = f64::max(self.0 / 10.0, 1.0);
    }
        
}

impl Default for Speed {

    fn default() -> Self {
        Self(DEFAULT_TIMESTEP)
    }

}