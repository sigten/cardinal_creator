use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_pancam::PanCamPlugin;

use camera::create_camera;
use game_config::{check_config_loaded, load_config, ConfigHandles, GameConfig};

use crate::AppState;

pub mod camera;
pub mod game_config;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<GameConfig>::new(&["config"]))
            .add_plugin(PanCamPlugin::default())
            .insert_resource(ConfigHandles::default())
            .add_system_set(
                SystemSet::on_enter(AppState::Init)
                    .with_system(create_camera)
                    .with_system(load_config),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Init)
                    .with_system(print_config)
                    .with_system(check_config_loaded),
            );
    }
}

fn print_config(configs: Res<ConfigHandles>, assets: Res<Assets<GameConfig>>) {
    for handle in &configs.0 {
        if let Some(config) = assets.get(handle) {
            info!("{:?}", config)
        }
    }
}
