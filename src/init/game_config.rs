use std::path::PathBuf;

use bevy::{asset::LoadState, prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::{game_list::GAME_LIST, AppState};

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "6cfa2bd6-ed72-430b-90a1-73671d6d4b6f"]
pub struct GameConfig {
    pub name: String,
    pub path: String,
    pub button_color: Option<ButtonColor>,
    pub button_text_color: Option<Color>,
    pub sprite_width: f32,
    pub sprite_height: f32,
    pub map_width: u16,
    pub map_height: u16,
    pub bg_color: (f32, f32, f32, f32),
    pub fov: FieldOfView,
    pub objective: GameObjective,
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
pub struct ButtonColor {
    pub hovered: Color,
    pub clicked: Color,
    pub none: Color,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub enum FieldOfView {
    AllVisible,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub enum GameObjective {
    Escape(u16, u16),
}

#[derive(Default, Debug)]
pub struct ConfigHandles(pub Vec<Handle<GameConfig>>);

pub fn load_config(asset_server: Res<AssetServer>, mut configs: ResMut<ConfigHandles>) {
    for game in GAME_LIST {
        let mut path = PathBuf::from(game.clone());
        path.set_extension("config");
        let asset: Handle<GameConfig> = asset_server.load(path);
        configs.0.push(asset);
    }
}

pub fn check_config_loaded(
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
    handles: Res<ConfigHandles>,
) {
    match asset_server.get_group_load_state(handles.0.iter().map(|h| h.id)) {
        LoadState::Loaded => state.set(AppState::Menu).unwrap(),
        _ => (),
    }
}
