use bevy::core::Name;
use bevy::math::{Vec3, DVec3};
use bevy::prelude::{Bundle, Component, Reflect, Transform, Entity, Color, Quat, default, Visibility};

use crate::serialization::SerializedBody;

#[derive(Component, Clone, Default, Reflect)]
pub struct Mass(pub f64);

#[derive(Default, Component, Reflect, Clone)]
pub struct Velocity(pub DVec3);

#[derive(Default, Component, Reflect, Clone)]
pub struct Acceleration(pub DVec3);

#[derive(Component, Reflect, Clone, Default)]
pub struct Scale(pub f32);

#[derive(Component, Reflect, Clone, Default)]
pub struct RotationSpeed(pub f32);

#[derive(Component, Reflect, Clone, Default)]
pub struct StartingRotation(pub Quat);

#[derive(Component, Reflect, Clone, Default)]
pub struct ModelPath(pub String);

#[derive(Component, Reflect, Clone, Default)]
pub struct BodyChildren(pub Vec<Entity>);

#[derive(Component, Reflect, Clone)]
pub struct OrbitSettings {
    
    pub color: Color,
    pub max_points: i32,
    pub lines: Vec<Vec3>,
    pub draw_lines: bool
                         
}

impl Default for OrbitSettings {
    
    fn default() -> Self {
        OrbitSettings { color: Color::GREEN, max_points: 3000, lines: vec![], draw_lines: false }
    }
    
}

#[derive(Component, Reflect, Clone, Default)]
pub struct LightSource {
    pub intensity: f32,
    pub shadows_enabled: bool,
    pub range: f32,
    pub radius: f32,
    pub enabled: bool
}

#[derive(Component, Reflect, Clone, Default)]
pub struct SimPosition(pub DVec3);

#[derive(Component, Reflect, Clone, Default)]
pub struct Diameter {
    
    pub num: f64,
    pub applied: bool
    
}

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
    pub scale: Scale,
    pub name: Name,
    pub model_path: ModelPath,
    pub light: LightSource,
    pub orbit: OrbitSettings,
    pub rotation_speed: RotationSpeed,
    pub starting_rotation: StartingRotation,   
    pub diameter: Diameter,
                   
}

impl From<SerializedBody> for BodyBundle {
    
    fn from(value: SerializedBody) -> Self {
        BodyBundle {
            mass: Mass(value.data.mass),
            sim_position: SimPosition(DVec3::from(value.data.starting_position) * 1000.0),
            vel: Velocity(DVec3::from(value.data.starting_velocity) * 1000.0),
            name: Name::new(value.data.name),
            model_path: ModelPath(format!("models/{}#Scene0", value.data.model_path)),
            diameter: Diameter {
                num: value.data.diameter * 1000.0,
                ..default()
            },
           ..default()
        }
    }
    
}