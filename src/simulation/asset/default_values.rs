use crate::simulation::asset::serialization::{SerializedFixedBodyFrame, SerializedMat3};
use anise::structure::planetocentric::ellipsoid::Ellipsoid;
use bevy::prelude::Mat3;

pub fn default_id() -> i32 {
    -1
}

pub fn default_frame() -> SerializedFixedBodyFrame {
    SerializedFixedBodyFrame {
        target_id: default_id(),
        orientation_id: default_id()
    }
}

pub fn default_spk() -> Vec<String> {
    Vec::new()
}

pub fn default_ellipsoid() -> Ellipsoid {
    Ellipsoid::from_sphere(1.0)
}

pub fn default_rot_matrix() -> SerializedMat3 {
    SerializedMat3::from(Mat3::IDENTITY)
}

pub fn default_color() -> String {
    "#ffffff".to_string()
}