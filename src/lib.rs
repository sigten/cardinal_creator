use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Init,
    Menu,
    Credits,
    LoadGame,
    PlayGame,
    GameEnd,
}

pub struct CardinalCreatorPlugin;

impl Plugin for CardinalCreatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Init);
        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
