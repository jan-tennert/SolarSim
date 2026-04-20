use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub struct AniseMetadata {

    pub ephemeris_id: i32,
    //For constants and rotation
    pub target_id: i32,
    pub orientation_id: i32,

}

impl Default for AniseMetadata {

    fn default() -> Self {
        Self {
            ephemeris_id: -1,
            orientation_id: -1,
            target_id: -1
        }
    }

}