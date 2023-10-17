use bevy::core::Name;
use bevy::math::{Vec3, DVec3};
use bevy::prelude::{default, Transform, Quat, EulerRot};
use crate::body::{BodyBundle, Mass, ModelPath, Scale, Velocity, LightSource, SimPosition, RotationSpeed, Diameter};
use crate::constants::M_TO_UNIT;

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
Vel in m/s
Acc in m/s^2
Transform Position in 0.1 AU (0.1 AU = 1 unit)
Radius in m
Rotation speed in km/h
 */
impl Bodies {

    pub fn sun() -> BodyBundle {
        BodyBundle {
            mass: Mass(1_988_500e24),
            transform: Transform::from_translation(Vec3::ZERO),
            sim_position: SimPosition(DVec3::ZERO),
            vel: Default::default(),
            acc: Default::default(),
            scale: Scale(0.01),
            name: Name::new("Sun"),
            diameter: Diameter {
                num: 696_000.0 * 2.0 * 1000.0,
                ..default()
            },
            model_path: ModelPath("models/sun.glb#Scene0".to_string()),
            light: LightSource {
                intensity: 1500000.0,
                shadows_enabled: false,
                range: 300000.0,
                radius: 10.0,
                enabled: true,
            },
            ..default()
        }
    }
    
    pub fn earth() -> BodyBundle {
        let sim_pos = DVec3::new(
            1.473588784571390E+08, 1.854315256927273E+07, 2.990429803438578E+04
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(5.97219e24),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -4.226365231723641E+00, 2.941379349033467E+01, -2.828583292782128E-03   
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            diameter: Diameter {
                num: 12_742.0 * 1000.0,
                ..default()
            },
            scale: Scale(0.00002),
            name: Name::new("Earth"),
            model_path: ModelPath("models/earth.glb#Scene0".to_string()),
       //     rotation_speed: RotationSpeed(1.574),
      //      starting_rotation: StartingRotation(Quat::from_euler(EulerRot::XYZ, 1.0, 0.0, 0.0)),
            ..default()
        }
    }
    
    pub fn saturn() -> BodyBundle {
        let sim_pos = DVec3::new(
            1.317721699784666E+09, -6.263762138853518E+08, -4.157355925955266E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(5.6834e26),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                3.608323540191913E+00, 8.705880483493228E+00, -2.953903588682212E-01  
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00007),
            name: Name::new("Saturn"),
            model_path: ModelPath("models/saturn.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn saturn_titan() -> BodyBundle {
        let sim_pos = DVec3::new(
            1.317062395789841E+09, -6.254109541976979E+08, -4.200566301576936E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.3452e23),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -1.060852998165573E+00, 6.402666517530363E+00, 1.357634287951674E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00001),
            name: Name::new("Titan"),
            model_path: ModelPath("models/titan.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn saturn_rhea() -> BodyBundle {
        let sim_pos = DVec3::new(
            1.317198227126551E+09, -6.263121286545614E+08, -4.155952859529075E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(2.3064854e21),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                2.806558904291587E+00, 1.270148256713700E+00, 3.694364144037066E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00001),
            name: Name::new("Rhea"),
            model_path: ModelPath("models/rhea.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn jupiter() -> BodyBundle {
        let sim_pos = DVec3::new(
            5.911164050429280E+08, 4.486127736586710E+08, -1.508610682481316E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.8982e27),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -8.045068878300311E+00, 1.102381638213635E+01, 1.341531152888358E-01  
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00007),
            name: Name::new("Jupiter"),
            model_path: ModelPath("models/jupiter.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn jupiter_io() -> BodyBundle {
        let sim_pos = DVec3::new(
            5.910424467821088E+08, 4.481963687394117E+08, -1.510185010929203E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(8.931938e22),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                8.957736595686779E+00, 7.959026250920237E+00, 2.787009746093063E-01
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00001),
            name: Name::new("Io"),
            model_path: ModelPath("models/io.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn jupiter_europa() -> BodyBundle {
        let sim_pos = DVec3::new(
            5.917799042824603E+08, 4.486983281930672E+08, -1.506823606685701E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(4.799844e22),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -9.693151465294227E+00, 2.469741639214316E+01, 5.694296800460830E-01
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000008),
            name: Name::new("Europa"),
            model_path: ModelPath("models/europa.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn jupiter_ganymede() -> BodyBundle {
        let sim_pos = DVec3::new(
            5.920393829735433E+08, 4.480741128331422E+08, -1.509370908536822E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.4819e23),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -2.558462326557859E+00, 2.043120719962253E+01, 5.697972593813327E-01
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00001),
            name: Name::new("Ganymede"),
            model_path: ModelPath("models/ganymede.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn jupiter_callisto() -> BodyBundle {
        let sim_pos = DVec3::new(
            5.928184462926141E+08, 4.478344900349652E+08, -1.508781974881226E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.075938e23),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -4.643871215581399E+00, 1.853965996642426E+01, 4.153266498041814E-01
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00001),
            name: Name::new("Callisto"),
            model_path: ModelPath("models/callisto.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn venus() -> BodyBundle {
        let sim_pos = DVec3::new(
            8.476483460935698E+07, 6.527795533113867E+07, -4.030295749102697E+06
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(4.8675e24),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -2.133838684070412E+01, 2.768230884313838E+01, 1.611943339470342E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00001),
            name: Name::new("Venus"),
            model_path: ModelPath("models/venus.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn mercury() -> BodyBundle {
        let sim_pos = DVec3::new(
            -2.658235940349510E+07, 4.047607508223532E+07, 5.690109263829736E+06
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(3.3011e23),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -5.119740738494808E+01, -2.382829179403439E+01, 2.750476586235273E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000006),
            name: Name::new("Mercury"),
            model_path: ModelPath("models/mercury.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn uranus() -> BodyBundle {
        let sim_pos = DVec3::new(
            1.876848145196212E+09, 2.256742495428547E+09, -1.593333878791571E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(8.6810e25),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -5.285944969180821E+00, 4.037177487005098E+00, 8.328859774515029E-02
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00004),
            name: Name::new("Uranus"),
            model_path: ModelPath("models/uranus.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn neptune() -> BodyBundle {
        let sim_pos = DVec3::new(
            4.460737814330130E+09, -3.117194956197202E+08, -9.638308729856475E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.024e26),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                3.424898338191547E-01, 5.454448402599064E+00, -1.196973250551823E-01
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00005),
            name: Name::new("Neptune"),
            model_path: ModelPath("models/neptune.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn neptune_triton() -> BodyBundle {
        let sim_pos = DVec3::new(
            4.460435655534760E+09, -3.118210796191955E+08, -9.622740008927625E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(2.1390e22),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                6.549830743821887E-01, 8.816651890055235E+00, 2.683376921837763E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000005),
            name: Name::new("Triton"),
            model_path: ModelPath("models/triton.glb#Scene0".to_string()),
            ..default()
        }
    }
          
    pub fn luna() -> BodyBundle {
        let sim_pos = DVec3::new(
            1.476804491967800E+08, 1.872052246844263E+07, 3.243775744153466E+04 
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(7.348e22),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -4.694794112410923E+00, 3.037390017058626E+01, 9.549595923954257E-02
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000009),
            diameter: Diameter {
                num: 1738.1 * 1000.0,
                ..default()
            },
            name: Name::new("Luna"),
            model_path: ModelPath("models/moon.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn pluto() -> BodyBundle {
        let sim_pos = DVec3::new(
            2.534605027840262E+09, -4.550728311952005E+09, -2.462016025535650E+08
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.303e22),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                4.905505817681830E+00, 1.466573354685091E+00, -1.581250123789350E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000009),
            name: Name::new("Pluto"),
            model_path: ModelPath("models/pluto.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn iss() -> BodyBundle { //timestap doesn't work with
        let sim_pos = DVec3::new(
            1.473527157673001E+08, 1.854377197753885E+07, 3.268702643421665E+04
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(450_000_000.0),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -1.455626164296770E+00, 2.686558693275802E+01, 6.676360863324918E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000002),
            name: Name::new("ISS"),
            model_path: ModelPath("models/iss.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn charon_pluto() -> BodyBundle { //timestap doesn't work with
        let sim_pos = DVec3::new(
            2.534602841613384E+09, -4.550740270530462E+09, -2.462169722225490E+08
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.586e21),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                4.743470035507049E+00, 1.357540337784795E+00, -1.473381802316020E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000005),
            name: Name::new("Charon"),
            model_path: ModelPath("models/pluto.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn phobos_mars() -> BodyBundle { //timestap doesn't work with
        let sim_pos = DVec3::new(
            -2.046811201572424E+08, -1.250112401183025E+08, 2.405122029878475E+06
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.0659e16),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                1.317247125277010E+01, -1.655773129437739E+01, -3.519634910822811E-01
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000003),
            name: Name::new("Phobos"),
            model_path: ModelPath("models/phobos.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn mars() -> BodyBundle {
        let sim_pos = DVec3::new(
            -2.046893400400904E+08, -1.250136923437167E+08, 2.409131185058415E+06
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(641.71e21),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                1.357395490411145E+01, -1.860254221026088E+01, -7.224152414868863E-01  
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.00001),
            name: Name::new("Mars"),
            model_path: ModelPath("models/mars.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn ceres() -> BodyBundle { //timestap doesn't work with
        let sim_pos = DVec3::new(
            -2.762371221893816E+08, -2.903518150199021E+08, 4.151164079416633E+07
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(9.38392e20),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                1.207056566717051E+01, -1.370357563530193E+01, -2.655445328553542E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000003),
            name: Name::new("Ceres"),
            model_path: ModelPath("models/ceres.glb#Scene0".to_string()),
            ..default()
        }
    }
    
    pub fn eris() -> BodyBundle { //timestap doesn't work with
        let sim_pos = DVec3::new(
            1.280400740948511E+10, 5.796599006941406E+09, -2.733004417387743E+09
        ) * 1000.0; //convert it to m
        BodyBundle {
            mass: Mass(1.6466e22),
            sim_position: SimPosition(sim_pos),
            transform: Transform::from_translation(
                convert_vec(sim_pos * M_TO_UNIT)
            ),
            vel: Velocity(DVec3::new(
                -7.745567938606255E-01, 1.503854709856890E+00, 1.614258646777714E+00
            ) * 1000.0) , //convert it to m/s
            acc: Default::default(),
            scale: Scale(0.000003),
            name: Name::new("Eris"),
            model_path: ModelPath("models/eris.glb#Scene0".to_string()),
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
 /*                   BodyEntry {
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
                        children: vec![
                            BodyEntry {
                                bundle: Bodies::saturn_titan(),
                                children: vec![]
                            },
                            BodyEntry {
                                bundle: Bodies::saturn_rhea(),
                                children: vec![]
                            }
                        ]
                    },
                    BodyEntry {
                        bundle: Bodies::jupiter(),
                        children: vec![
                            BodyEntry {
                                bundle: Bodies::jupiter_io(),
                                children: vec![]
                            },
                            BodyEntry {
                                bundle: Bodies::jupiter_europa(),
                                children: vec![]
                            },
                            BodyEntry {
                                bundle: Bodies::jupiter_callisto(),
                                children: vec![]
                            },
                            BodyEntry {
                                bundle: Bodies::jupiter_ganymede(),
                                children: vec![]
                            }
                        ]
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
                        children: vec![
                            BodyEntry {
                                bundle: Bodies::neptune_triton(),
                                children: vec![]
                            }
                        ]
                    },
                    BodyEntry {
                        bundle: Bodies::pluto(),
                        children: vec![]
                    },
                    BodyEntry {
                        bundle: Bodies::ceres(),
                        children: vec![]
                    },
                    BodyEntry {
                        bundle: Bodies::eris(),
                        children: vec![]
                    },*/
                ]
            }
        ]
    }
    

}

fn convert_vec(vec: DVec3) -> Vec3 {
    return Vec3::new(vec.x as f32, vec.y as f32, vec.z as f32);
}