/*
pub const DEFAULT_CAM_RADIUS: f32 = 150.0;

/// Tags an entity as capable of panning and orbiting.
#[derive(Component, Reflect)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: DEFAULT_CAM_RADIUS,
            upside_down: false,
        }
    }
}

pub struct PanOrbitCameraPlugin;

impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<PanOrbitCamera>()
        .add_systems(Update, pan_orbit_camera.after(apply_physics).run_if(in_state(SimState::Loaded)));
        //.add_system_to_stage(CoreStage::PostUpdate, pan_orbit_camera);
    }  
}

// Time constant for smooth transitions (lower = slower, higher = faster)
const SMOOTH_FACTOR: f32 = 0.1;
const ZOOM_SENSITIVITY: f32 = 5.0;
const ORBIT_SENSITIVITY: f32 = 200.0;

pub fn pan_orbit_camera(
    time: Res<Time>, // Add Time resource to calculate frame deltas
    mut windows: Query<&mut Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<ButtonInput<MouseButton>>, // Fix for Input instead of ButtonInput
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
    mut lock_on: ResMut<LockOn>,
    mut egui_ctx: EguiContexts,
) {
    let orbit_button = MouseButton::Left;
    let pan_button = MouseButton::Right;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    // Mouse input for orbiting and panning
    if input_mouse.pressed(orbit_button) {
        lock_on.enabled = false;
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        for ev in ev_motion.read() {
            pan += ev.delta;
        }
    } else {
        ev_motion.clear();
    }

    // Check if within egui context to avoid conflicts
    if egui_ctx.try_ctx_mut().is_some() && !egui_ctx.ctx_mut().is_pointer_over_area() {
        for ev in ev_scroll.read() {
            #[cfg(not(target_family = "wasm"))]
            let multiplier = ZOOM_SENSITIVITY;
            #[cfg(target_family = "wasm")]
            let multiplier = 0.01;
            scroll += ev.y * multiplier;
        }
    }

    // Track if orbit button state changed this frame
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    // Main camera update loop
    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        // Handle orbit inversion if camera is upside down
        if orbit_button_changed {
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        // Define target position based on current focus and radius
        let target_position = pan_orbit.focus + transform.rotation * Vec3::new(0.0, 0.0, pan_orbit.radius);

        // SMOOTH ROTATION
        if rotation_move.length_squared() > 0.0 {
            let window = get_primary_window_size(&mut windows.single_mut());

            // Increase the speed by multiplying the delta by a higher factor
            let delta_x = (rotation_move.x * 1.) / window.x * std::f32::consts::PI * 2.0;
            let delta_y = (rotation_move.y * 1.)
                / window.y * std::f32::consts::PI;

            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            let target_rotation = yaw * transform.rotation * pitch;

            // Smooth rotation
            transform.rotation = transform.rotation.slerp(target_rotation, SMOOTH_FACTOR * time.delta_seconds());
        }

        // SMOOTH PANNING
        else if pan.length_squared() > 0.0 {
            let window = get_primary_window_size(&mut windows.single_mut());
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        }

        // SMOOTH ZOOM
        else if scroll.abs() > 0.0 {
            let target_radius = f32::max(pan_orbit.radius - scroll * pan_orbit.radius * 0.2, 0.000001);
            pan_orbit.radius = pan_orbit.radius.lerp(target_radius, SMOOTH_FACTOR);
        }

        // Calculate final target translation based on rotation and focus
        let final_position = pan_orbit.focus + transform.rotation * Vec3::new(0.0, 0.0, pan_orbit.radius);

        // Smoothly interpolate the camera's position
        transform.translation = transform.translation.lerp(final_position, SMOOTH_FACTOR);

        // Lock on to a specific target if enabled
        if lock_on.enabled {
            transform.look_at(Vec3::splat(0.0), Vec3::Y);
        }
    }
}

pub fn get_primary_window_size(window: &mut Window) -> Vec2 {
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}*/