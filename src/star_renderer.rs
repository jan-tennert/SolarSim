use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, Query, Transform, Resource, Entity, With, ResMut, IntoSystemConfigs, Camera, Without, Visibility};
use crate::SimState;
use crate::body::{Mass, Star};

pub struct StarRendererPlugin;

impl Plugin for StarRendererPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, change_sun_renderer.run_if(in_state(SimState::Simulation)));
    }

}

fn change_sun_renderer(
    camera: Query<(&Transform, With<Camera>, Without<Star>)>,
    mut stars: Query<(&Transform, &Visibility, &mut Star, Without<Camera>)>
) {
    let (c_transform, _, _) = camera.single();
    for (transform, visibility, mut star, _) in &mut stars {
        let distance = c_transform.translation.distance(transform.translation);
        if distance > 25_000.0 &&!star.use_imposter {
            star.use_imposter = true;
        }
    }
}