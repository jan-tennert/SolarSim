use crate::simulation::components::anise::AnisePlugin;
use crate::simulation::components::apsis::ApsisPlugin;
use crate::simulation::components::billboard::BodyBillboardPlugin;
use crate::simulation::components::direction::DirectionPlugin;
use crate::simulation::components::horizons::HorizonsPlugin;
use crate::simulation::components::lock_on::LockOnPlugin;
use crate::simulation::components::motion_line::MotionLinePlugin;
use crate::simulation::components::reset::ResetPlugin;
use crate::simulation::components::rotation::RotationPlugin;
use crate::simulation::components::scale::ScalePlugin;
use crate::simulation::components::selection::SelectionPlugin;
use crate::simulation::components::shape::DiameterPlugin;
use crate::simulation::components::speed::SpeedPlugin;
use crate::simulation::integration::IntegrationPlugin;
use bevy::app::Plugin;

pub mod apsis;
pub mod billboard;
pub mod camera;
pub mod shape;
pub mod direction;
pub mod lock_on;
pub mod body;
pub mod motion_line;
pub mod rotation;
pub mod speed;
pub mod selection;
pub mod reset;
pub mod editor;
pub mod horizons;
pub mod scale;
pub mod anise;
mod spacecraft;

pub struct SimComponentPlugin;

impl Plugin for SimComponentPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(ApsisPlugin)
            .add_plugins(BodyBillboardPlugin)
          //  .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(DiameterPlugin)
            .add_plugins(DirectionPlugin)
            .add_plugins(IntegrationPlugin)
            .add_plugins(LockOnPlugin)
            .add_plugins(ScalePlugin)
            .add_plugins(MotionLinePlugin)
            .add_plugins(ResetPlugin)
            .add_plugins(RotationPlugin)
            .add_plugins(SelectionPlugin)
            .add_plugins(SpeedPlugin)
            .add_plugins(HorizonsPlugin)
            .add_plugins(AnisePlugin);
    }

}