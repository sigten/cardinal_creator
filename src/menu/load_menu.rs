use bevy::{prelude::*, sprite::Rect};

use crate::init::game_config::{ButtonColor, ConfigHandles, GameConfig};

#[derive(Component, Debug, Default)]
pub struct Menu;

#[derive(Component, Debug, Default, Clone)]
pub struct MenuButton {
    pub name: String,
    pub buttons: ButtonColor,
    pub game: Handle<GameConfig>,
}

pub fn load_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_handles: Res<ConfigHandles>,
    game_configs: Res<Assets<GameConfig>>,
) {
    let node = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Menu)
        .id();

    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    for handle in &game_handles.0 {
        let config = game_configs.get(handle).unwrap();

        let mut color = ButtonColor {
            hovered: Color::rgb(0.25, 0.25, 0.25),
            clicked: Color::rgb(0.35, 0.75, 0.35),
            none: Color::RED,
        };
        if let Some(button_color) = config.button_color {
            color = button_color.clone()
        }

        let menu_button = MenuButton {
            name: config.name.clone(),
            buttons: color.into(),
            game: handle.clone(),
        };
        commands
            .entity(node)
            .with_children(|parent| {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.), Val::Px(65.)),
                            margin: UiRect::all(Val::Px(6.)),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: UiColor(menu_button.buttons.none).clone(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            menu_button.name.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
            })
            .insert(menu_button);
    }
}
