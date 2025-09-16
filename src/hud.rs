use crate::map::TILE_SIZE;
use crate::player::Player;
use crate::prelude::GameState::Rendering;
use crate::prelude::GameSystems::Ui;
use crate::prelude::{Currency, Fuel, GameAssets, Health, Inventory, world_to_grid_position};
use bevy::math::Vec2;
use bevy::prelude::{
    App, AssetServer, BuildChildren, ChildBuild, Color, Commands, Component, DespawnRecursiveExt,
    Entity, FlexDirection, Image, ImageBundle, ImageNode, IntoSystemConfigs, JustifyContent, Node,
    OnEnter, Plugin, PositionType, Query, Res, Sprite, Text, TextColor, TextFont, TextLayout,
    TextUiWriter, TextureAtlas, Transform, Update, Val, With, info,
};
use bevy::text::JustifyText::{Left, Right};
use bevy::text::TextSpan;
use bevy::ui::AlignItems::Start;
use bevy::ui::Val::Px;
use bevy::ui::widget::NodeImageMode;
use bevy::ui::widget::NodeImageMode::Stretch;
use bevy::ui::{BackgroundColor, UiRect};
use bevy::utils::default;
use std::ops::Mul;

pub struct HUDPlugin;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudIntegrity;

#[derive(Component)]
struct HudDepthText;

#[derive(Component)]
struct HudFuelText;

#[derive(Component)]
struct HudInventoryText;
#[derive(Component)]
struct HudCurrencyText;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Rendering), init_hud)
            .add_systems(Update, update_hud.in_set(Ui));
    }
}

fn init_hud(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
    hud_query: Query<Entity, With<Hud>>,
) {
    let font = assets_server.load("fonts/FiraSans-Regular.ttf");

    let font_style = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..Default::default()
    };
    //Clear previous hud statistics, if any
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands
        .spawn((
            Hud,
            BackgroundColor(Color::NONE),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: Start,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                left: Px(10.0),
                top: Px(10.0),
                padding: UiRect::all(Px(10.0)),
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
        ))
        .with_children(|hud_children| {
            // Currency
            hud_children
                .spawn((
                    Text::new("Money: "),
                    font_style.clone(),
                    TextColor(Color::srgb(255.0, 215.0, 0.0)),
                    TextLayout::new_with_justify(Right),
                    HudCurrencyText,
                ))
                .with_child((TextSpan::default(), font_style.clone()));
            // Integrity stat
            hud_children.spawn((
                ImageNode::from_atlas_image(
                    game_assets.hud[0].texture.clone(),
                    TextureAtlas {
                        layout: game_assets.hud[0].texture_layout.clone(),
                        index: 0,
                    },
                )
                .with_mode(NodeImageMode::Auto),
                Node {
                    width: Val::Px(TILE_SIZE.mul(1.5)),
                    height: Val::Px(TILE_SIZE.mul(2.0)),
                    ..default()
                },
                HudIntegrity,
            ));
            // Depth stat
            hud_children
                .spawn((
                    Text::new("Depth: "),
                    font_style.clone(),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Left),
                    HudDepthText,
                ))
                .with_child((TextSpan::default(), font_style.clone()));
            // Velocity stat
            hud_children
                .spawn((
                    Text::new("Fuel: "),
                    font_style.clone(),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Left),
                    HudFuelText,
                ))
                .with_child((TextSpan::default(), font_style.clone()));
            // Inventory stat
            hud_children
                .spawn((
                    Text::new("Inventory: "),
                    font_style.clone(),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Left),
                    HudInventoryText,
                ))
                .with_child((TextSpan::default(), font_style.clone()));
        });
}

fn update_hud(
    mut hud_integrity: Query<(Entity, &mut ImageNode), With<HudIntegrity>>,
    hud_depth_text: Query<Entity, With<HudDepthText>>,
    hud_fuel_text: Query<Entity, With<HudFuelText>>,
    hud_inventory_text: Query<Entity, With<HudInventoryText>>,
    hud_currency_text: Query<Entity, With<HudCurrencyText>>,
    player: Query<(&Health, &Transform, &Fuel, &Inventory, &Currency), With<Player>>,
    mut text_writer: TextUiWriter,
) {
    // Updating hud stats
    if let Ok(player_stats) = player.get_single() {
        if let Ok(currency_text_entity) = hud_currency_text.get_single() {
            let currency_amount = player_stats.4.amount;
            *text_writer.text(currency_text_entity, 1) = format!("{}", currency_amount);
        }
        if let Ok((_, mut image_node)) = hud_integrity.get_single_mut() {
            let health = player_stats.0;

            // texture index on the base of health level
            let health_level_index = 10 - (health.current / 10.0).round() as usize;
            if let Some(texture_atlas) = &mut image_node.texture_atlas {
                texture_atlas.index = health_level_index;
            };
        }
        if let Ok(depth_text_entity) = hud_depth_text.get_single() {
            let position = world_to_grid_position(player_stats.1.translation.truncate());
            *text_writer.text(depth_text_entity, 1) = format!("{}", position.1);
        }
        if let Ok(fuel_text_entity) = hud_fuel_text.get_single() {
            let fuel = player_stats.2;
            *text_writer.text(fuel_text_entity, 1) = format!("{}", fuel.current.trunc());
        }
        if let Ok(inventory_text_entity) = hud_inventory_text.get_single() {
            let inventory = player_stats.3;
            *text_writer.text(inventory_text_entity, 1) = format!("{:?}", inventory.print_items());
        }
    }
}
