use bevy::prelude::*;

use load_menu::load_menu;
use crate::AppState;

mod load_menu;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(load_menu))
            /*.add_system_set(SystemSet::on_update(AppState::Menu).with_system(handle_interactions))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(remove_menu))*/;
    }
}
