use bevy::app::Plugin;
use crate::simulation::components::anise::AnisePlugin;
use crate::simulation::components::apsis::ApsisPlugin;
use crate::simulation::components::billboard::BodyBillboardPlugin;
use crate::simulation::components::camera::PanOrbitCameraPlugin;
use crate::simulation::components::diameter::DiameterPlugin;
use crate::simulation::components::direction::DirectionPlugin;
use crate::simulation::components::horizons::HorizonsPlugin;
use crate::simulation::components::lock_on::LockOnPlugin;
use crate::simulation::components::orbit_lines::OrbitLinePlugin;
use crate::simulation::components::physics::PhysicsPlugin;
use crate::simulation::components::reset::ResetPlugin;
use crate::simulation::components::rotation::RotationPlugin;
use crate::simulation::components::scale::ScalePlugin;
use crate::simulation::components::selection::SelectionPlugin;
use crate::simulation::components::speed::SpeedPlugin;

pub mod apsis;
pub mod billboard;
pub mod camera;
pub mod diameter;
pub mod direction;
pub mod lock_on;
pub mod physics;
pub mod body;
pub mod orbit_lines;
pub mod rotation;
pub mod speed;
pub mod selection;
pub mod reset;
pub mod editor;
pub mod horizons;
pub mod scale;
pub mod anise;

pub struct SimComponentPlugin;

impl Plugin for SimComponentPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(ApsisPlugin)
            .add_plugins(BodyBillboardPlugin)
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(DiameterPlugin)
            .add_plugins(DirectionPlugin)
            .add_plugins(LockOnPlugin)
            .add_plugins(ScalePlugin)
            .add_plugins(OrbitLinePlugin)
            .add_plugins(PhysicsPlugin)
            .add_plugins(ResetPlugin)
            .add_plugins(RotationPlugin)
            .add_plugins(SelectionPlugin)
            .add_plugins(SpeedPlugin)
            .add_plugins(HorizonsPlugin)
            .add_plugins(AnisePlugin);
    }

}