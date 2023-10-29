use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Camera, Children, Component, in_state, IntoSystemConfigs, Parent, Query, Transform, Visibility, With, Without};
use bevy::scene::SceneInstance;

use crate::body::Star;
use crate::camera::PanOrbitCamera;
use crate::SimState;

pub struct StarRendererPlugin;

impl Plugin for StarRendererPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, change_sun_renderer.run_if(in_state(SimState::Simulation)));
    }

}

#[derive(Component, Debug, Default)]
pub struct StarBillboard;

fn change_sun_renderer(
    camera: Query<(&Transform, &PanOrbitCamera, With<Camera>, Without<Star>, Without<StarBillboard>)>,
    mut stars: Query<(&Transform, &Children, &mut Star, Without<Camera>, Without<StarBillboard>)>,
    mut star_billboards: Query<(&mut Transform, &mut Visibility, &Parent, With<StarBillboard>, Without<Camera>, Without<Star>)>,
    mut scenes: Query<(&SceneInstance, &mut Visibility, Without<StarBillboard>, Without<Star>)>,
) {
    let (c_transform, pan_orbit, _, _, _) = camera.single();
    for (transform, children, mut star, _, _) in &mut stars {
        let distance = c_transform.translation.distance(transform.translation);
        if distance > 25_000.0 &&!star.use_imposter {
            star.use_imposter = true;
            for child in children.iter() {
                if let Ok((_, mut visibility, _ , _)) = scenes.get_mut(*child) {
                    *visibility = Visibility::Hidden;
                }
                if let Ok((_, mut visibility, _, _, _, _)) = star_billboards.get_mut(*child) {
                    *visibility = Visibility::Visible;
                }
            }
        } else if distance < 25_000.0 && star.use_imposter {
            star.use_imposter = false;
            for child in children.iter() {
                if let Ok((scene, mut visibility, _, _)) = scenes.get_mut(*child) {
                    *visibility = Visibility::Visible;
                }
                if let Ok((_, mut visibility, _, _, _, _)) = star_billboards.get_mut(*child) {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }

    for (mut b_transform, _, parent, _, _, _) in &mut star_billboards {
        let (transform, _, _, _, _) = stars.get(**parent).unwrap();
        let distance = c_transform.translation.distance(transform.translation);
        println!("{}", distance);
        b_transform.look_at(-c_transform.translation, Vec3::Y);
        b_transform.scale = Vec3::splat(distance / 6000.0);
    }
}