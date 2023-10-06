use bevy::core::Name;
use bevy::math::Vec3;
use bevy::pbr::PointLight;
use bevy::prelude::{default, Transform};
use bevy::reflect::erased_serde::__private::serde::de::Unexpected::Option;
use crate::body::{BodyBundle, Mass, ModelPath, Diameter, Velocity, LightSource, SimPosition, Selectable, Parent};
use crate::constants::KM_TO_AU;

pub struct Bodies;

/*
Data from https://ssd.jpl.nasa.gov/horizons/app.html#/ on 2023-10-01
Mass in kg
Diameter in AU (AU = unit)
SimPosition in m
Vel in km/s
Acc in km/s^2
Transform Position in 0.1 AU (0.1 AU = 1 unit)
 */
impl Bodies {

    pub fn sun() -> BodyBundle {
        BodyBundle {
            mass: Mass(1_988_500e24),
            transform: Transform::from_translation(Vec3::ZERO),
            sim_position: SimPosition(Vec3::ZERO),
            vel: Default::default(),
            acc: Default::default(),
            diameter: Diameter(0.01),
            name: Name::new("Sun"),
            model_path: ModelPath("models/sun.glb#Scene0".to_string()),
            light: LightSource {
                intensity: 1000000.0,
                shadows_enabled: false,
                range: 300000.0,
                radius: 5.0,
                enabled: true,
            },
            selectable: Selectable(true),
            ..default()
        }
    }
    
    pub fn earth() -> BodyBundle {
        let sim_pos = Vec3::new(
            1.473588784571390E+08, 1.854315256927273E+07, 2.990429803438578E+04
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(5.97219e24),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                -4.226365231723641E+00, 2.941379349033467E+01, -2.828583292782128E-03   
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.00002),
            name: Name::new("Earth"),
            model_path: ModelPath("models/earth.glb#Scene0".to_string()),
            parent: Parent(Some("Sun".to_string())),
            ..default()
        }
    }
    
    pub fn moon() -> BodyBundle {
        let sim_pos = Vec3::new(
            1.476804491967800E+08, 1.872052246844263E+07, 3.243775744153466E+04 
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(7.348e22),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                -4.694794112410923E+00, 3.037390017058626E+01, 9.549595923954257E-02
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.000009),
            name: Name::new("Moon"),
            model_path: ModelPath("models/moon.glb#Scene0".to_string()),
            parent: Parent(Some("Earth".to_string())),
            ..default()
        }
    }

    pub fn all() -> Vec<BodyBundle> {
            vec![Bodies::sun(), Bodies::earth(), Bodies::moon()]
    }

}