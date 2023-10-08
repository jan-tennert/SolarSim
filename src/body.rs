use bevy::core::Name;
use bevy::math::Vec3;
use bevy::pbr::PointLight;
use bevy::prelude::{Bundle, Component, Reflect, Transform, Entity};

#[derive(Component, Clone, Default, Reflect)]
pub struct Mass(pub f64);

#[derive(Default, Component, Reflect, Clone)]
pub struct Velocity(pub Vec3);

#[derive(Default, Component, Reflect, Clone)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Reflect, Clone, Default)]
pub struct Diameter(pub f32);

#[derive(Component, Reflect, Clone, Default)]
pub struct ModelPath(pub String);

#[derive(Component, Reflect, Clone, Default)]
pub struct BodyChildren(pub Vec<Entity>);

#[derive(Component, Reflect, Clone, Default)]
pub struct DrawOrbitLines(pub bool);

#[derive(Component, Reflect, Clone, Default)]
pub struct OrbitLines(pub Vec<(Vec3, Vec3)>);

#[derive(Component, Reflect, Clone, Default)]
pub struct LightSource {
    pub intensity: f32,
    pub shadows_enabled: bool,
    pub range: f32,
    pub radius: f32,
    pub enabled: bool
}

#[derive(Component, Reflect, Clone, Default)]
pub struct SimPosition(pub Vec3);

//Types:
#[derive(Component, Reflect, Clone, Default)]
pub struct Star;

#[derive(Component, Reflect, Clone, Default)]
pub struct Planet;

#[derive(Component, Reflect, Clone, Default)]
pub struct Moon;

#[derive(Bundle, Clone, Default)]
pub struct BodyBundle {

    pub mass: Mass,
    pub transform: Transform,
    pub sim_position: SimPosition,
    pub vel: Velocity,
    pub acc: Acceleration,
    pub diameter: Diameter,
    pub name: Name,
    pub model_path: ModelPath,
    pub light: LightSource,
    pub draw_orbit_lines: DrawOrbitLines,
    pub orbit_lines: OrbitLines
        
}