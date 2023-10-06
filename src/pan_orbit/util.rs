use bevy::input::Input;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{KeyCode, MouseButton, Res, Transform};
use bevy_easings::Lerp;

use super::lib::PanOrbitCamera;

const EPSILON: f32 = 0.001;

pub fn calculate_from_translation_and_focus(translation: Vec3, focus: Vec3) -> (f32, f32, f32) {
    let comp_vec = translation - focus;
    let mut radius = comp_vec.length();
    if radius == 0.0 {
        radius = 0.05; // Radius 0 causes problems
    }
    let alpha = if comp_vec.x == 0.0 && comp_vec.z >= 0.0 {
        0.0
    } else {
        (comp_vec.z / (comp_vec.x.powi(2) + comp_vec.z.powi(2)).sqrt()).acos()
    };
    let beta = (comp_vec.y / radius).asin();
    (alpha, beta, radius)
}

pub fn orbit_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
) -> bool {
    let is_pressed = pan_orbit
        .modifier_orbit
        .map_or(true, |modifier| key_input.pressed(modifier))
        && mouse_input.pressed(pan_orbit.button_orbit);

    is_pressed
        && pan_orbit
            .modifier_pan
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

pub fn orbit_just_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
) -> bool {
    let just_pressed = pan_orbit
        .modifier_orbit
        .map_or(true, |modifier| key_input.pressed(modifier))
        && (mouse_input.just_pressed(pan_orbit.button_orbit));

    just_pressed
        && pan_orbit
            .modifier_pan
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

pub fn orbit_just_released(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
) -> bool {
    let just_released = pan_orbit
        .modifier_orbit
        .map_or(true, |modifier| key_input.pressed(modifier))
        && (mouse_input.just_released(pan_orbit.button_orbit));

    just_released
        && pan_orbit
            .modifier_pan
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

pub fn pan_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
) -> bool {
    let is_pressed = pan_orbit
        .modifier_pan
        .map_or(true, |modifier| key_input.pressed(modifier))
        && mouse_input.pressed(pan_orbit.button_pan);

    is_pressed
        && pan_orbit
            .modifier_orbit
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

pub fn pan_just_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
) -> bool {
    let just_pressed = pan_orbit
        .modifier_pan
        .map_or(true, |modifier| key_input.pressed(modifier))
        && (mouse_input.just_pressed(pan_orbit.button_pan));

    just_pressed
        && pan_orbit
            .modifier_orbit
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

/// Update `transform` based on alpha, beta, and the camera's focus and radius
pub fn update_orbit_transform(
    alpha: f32,
    beta: f32,
    radius: f32,
    focus: Vec3,
    transform: &mut Transform,
) {
    let mut rotation = Quat::from_rotation_y(alpha);
    rotation *= Quat::from_rotation_x(-beta);
    transform.rotation = rotation;

    // Update the translation of the camera so we are always rotating 'around'
    // (orbiting) rather than rotating in place
    transform.translation = focus + transform.rotation * Vec3::new(0.0, 0.0, radius);
}

pub fn apply_limits(value: f32, upper_limit: Option<f32>, lower_limit: Option<f32>) -> f32 {
    let mut new_val = value;
    if let Some(zoom_upper) = upper_limit {
        new_val = f32::min(new_val, zoom_upper);
    }
    if let Some(zoom_lower) = lower_limit {
        new_val = f32::max(new_val, zoom_lower);
    }
    new_val
}

pub fn approx_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub fn lerp_and_snap_f32(from: f32, to: f32, smoothness: f32) -> f32 {
    let t = 1.0 - smoothness;
    let mut new_value = from.lerp(&to, &t);
    if smoothness < 1.0 && approx_equal(new_value, to) {
        new_value = to;
    }
    new_value
}

pub fn lerp_and_snap_vec3(from: Vec3, to: Vec3, smoothness: f32) -> Vec3 {
    let t = 1.0 - smoothness;
    let mut new_value = from.lerp(to, t);
    if smoothness < 1.0 && approx_equal((new_value - to).length(), 0.0) {
        new_value.x = to.x;
    }
    new_value
}
