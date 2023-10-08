use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{default, Transform};
use crate::body::{BodyBundle, Mass, ModelPath, Diameter, Velocity, LightSource, SimPosition};
use crate::constants::KM_TO_AU;

pub struct Bodies;

pub struct BodyEntry {
    
    pub bundle: BodyBundle,
    pub children: Vec<BodyEntry>   
    
}    

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
                intensity: 10000000.0,
                shadows_enabled: false,
                range: 300000.0,
                radius: 10.0,
                enabled: true,
            },
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
            ..default()
        }
    }
    
    pub fn saturn() -> BodyBundle {
        let sim_pos = Vec3::new(
            1.317721699784666E+09, -6.263762138853518E+08, -4.157355925955266E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(5.6834e26),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                3.608323540191913E+00, 8.705880483493228E+00, -2.953903588682212E-01  
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.00007),
            name: Name::new("Saturn"),
            model_path: ModelPath("models/saturn.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn jupiter() -> BodyBundle {
        let sim_pos = Vec3::new(
            5.911164050429280E+08, 4.486127736586710E+08, -1.508610682481316E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(641.71e21),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                -8.045068878300311E+00, 1.102381638213635E+01, 1.341531152888358E-01  
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.00007),
            name: Name::new("Jupiter"),
            model_path: ModelPath("models/jupiter.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn venus() -> BodyBundle {
        let sim_pos = Vec3::new(
            8.476483460935698E+07, 6.527795533113867E+07, -4.030295749102697E+06
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(4.8675e24),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                -2.133838684070412E+01, 2.768230884313838E+01, 1.611943339470342E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.00001),
            name: Name::new("Venus"),
            model_path: ModelPath("models/venus.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn mercury() -> BodyBundle {
        let sim_pos = Vec3::new(
            -2.658235940349510E+07, 4.047607508223532E+07, 5.690109263829736E+06
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(3.3011e23),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                -5.119740738494808E+01, -2.382829179403439E+01, 2.750476586235273E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.000006),
            name: Name::new("Mercury"),
            model_path: ModelPath("models/mercury.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn uranus() -> BodyBundle {
        let sim_pos = Vec3::new(
            1.876848145196212E+09, 2.256742495428547E+09, -1.593333878791571E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(8.6810e25),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                -5.285944969180821E+00, 4.037177487005098E+00, 8.328859774515029E-02
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.00004),
            name: Name::new("Uranus"),
            model_path: ModelPath("models/uranus.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn neptune() -> BodyBundle {
        let sim_pos = Vec3::new(
            4.460737814330130E+09, -3.117194956197202E+08, -9.638308729856475E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.024e26),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                3.424898338191547E-01, 5.454448402599064E+00, -1.196973250551823E-01
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.00005),
            name: Name::new("Neptune"),
            model_path: ModelPath("models/neptune.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn luna() -> BodyBundle {
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
            name: Name::new("Luna"),
            model_path: ModelPath("models/moon.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn pluto() -> BodyBundle {
        let sim_pos = Vec3::new(
            2.534605027840262E+09, -4.550728311952005E+09, -2.462016025535650E+08
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.303e22),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                4.905505817681830E+00, 1.466573354685091E+00, -1.581250123789350E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.000009),
            name: Name::new("Pluto"),
            model_path: ModelPath("models/pluto.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn iss() -> BodyBundle { //timestap doesn't work with
        let sim_pos = Vec3::new(
            1.473527157673001E+08, 1.854377197753885E+07, 3.268702643421665E+04
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(450_000_000.0),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                -1.455626164296770E+00, 2.686558693275802E+01, 6.676360863324918E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.000002),
            name: Name::new("ISS"),
            model_path: ModelPath("models/iss.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn charon_pluto() -> BodyBundle { //timestap doesn't work with
        let sim_pos = Vec3::new(
            2.534602841613384E+09, -4.550740270530462E+09, -2.462169722225490E+08
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.586e21),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                4.743470035507049E+00, 1.357540337784795E+00, -1.473381802316020E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.000005),
            name: Name::new("Charon"),
            model_path: ModelPath("models/pluto.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn phobos_mars() -> BodyBundle { //timestap doesn't work with
        let sim_pos = Vec3::new(
            -2.046811201572424E+08, -1.250112401183025E+08, 2.405122029878475E+06
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.0659e16),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                1.317247125277010E+01, -1.655773129437739E+01, -3.519634910822811E-01
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.000003),
            name: Name::new("Phobos"),
            model_path: ModelPath("models/phobos.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn mars() -> BodyBundle {
        let sim_pos = Vec3::new(
            -2.046893400400904E+08, -1.250136923437167E+08, 2.409131185058415E+06
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(641.71e21),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                sim_pos * KM_TO_AU as f32
            ),
            vel: Velocity(Vec3::new(
                1.357395490411145E+01, -1.860254221026088E+01, -7.224152414868863E-01  
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter(0.00001),
            name: Name::new("Mars"),
            model_path: ModelPath("models/mars.glb#Scene0".to_string()),
            ..default()
        }
    }

    pub fn all() -> Vec<BodyEntry> { //this probably has to get improved but no idea how
        return vec![
            BodyEntry {
                bundle: Bodies::sun(),
                children: vec![
                    BodyEntry {
                        bundle: Bodies::earth(),
                        children: vec![
                            BodyEntry {
                                bundle: Bodies::luna(),
                                children: vec![]
                            }
                        ]
                    },
                    BodyEntry {
                        bundle: Bodies::mars(),
                        children: vec![
                            BodyEntry {
                                bundle: Bodies::phobos_mars(),
                                children: vec![]
                            }
                        ]
                    },
                    BodyEntry {
                        bundle: Bodies::saturn(),
                        children: vec![]
                    },
                    BodyEntry {
                        bundle: Bodies::jupiter(),
                        children: vec![]
                    },
                    BodyEntry {
                        bundle: Bodies::venus(),
                        children: vec![]
                    },
                    BodyEntry {
                        bundle: Bodies::mercury(),
                        children: vec![]
                    },
                    BodyEntry {
                        bundle: Bodies::uranus(),
                        children: vec![]
                    },
                    BodyEntry {
                        bundle: Bodies::neptune(),
                        children: vec![]
                    },
                    BodyEntry {
                        bundle: Bodies::pluto(),
                        children: vec![]
                    },
                ]
            }
        ]
    }

}