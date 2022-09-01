#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, render::texture::ImageSettings};

use init::InitPlugin;
use menu::MenuPlugin;

mod game_list;
mod init;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Init,
    Menu,
    ShowCredits,
    LoadGame,
    PlayGame,
    GameEnd,
}

pub struct CardinalCreatorPlugin;

impl Plugin for CardinalCreatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Init)
            .add_plugin(InitPlugin)
            .add_plugin(MenuPlugin)
            .insert_resource(ImageSettings::default_nearest());

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
