#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub mod init;
use init::InitPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
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
        app.add_state(GameState::Init).add_plugin(InitPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
