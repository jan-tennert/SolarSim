use bevy::prelude::{DetectChangesMut, Query, ResMut, Resource};
use bevy_egui::EguiContexts;

#[derive(Resource, PartialEq, Eq, Default)]
pub struct EguiWantsFocus {
    // Must track previous value too, because when you click inside an egui window,
    // Context::wants_pointer_input() returns false once before returning true again, which
    // is undesirable.
    pub prev: bool,
    pub curr: bool,
}

pub fn check_egui_wants_focus(
    mut contexts: EguiContexts,
    mut wants_focus: ResMut<EguiWantsFocus>,
) {
    let ctx = contexts.ctx_mut();
    let new_wants_focus = ctx.wants_pointer_input() || ctx.wants_keyboard_input();
    let new_res = EguiWantsFocus {
        prev: wants_focus.curr,
        curr: new_wants_focus,
    };
    wants_focus.set_if_neq(new_res);
}
