use crate::player::Player;
use crate::prelude::GameSystems::Ui;
use crate::prelude::{world_to_grid_position, Currency, Fuel, Health, Inventory};
use bevy::prelude::{App, AssetServer, BuildChildren, ChildBuild, Color, Commands, Component, Entity, FlexDirection, IntoSystemConfigs, JustifyContent, Node, OnEnter, Plugin, PositionType, Query, Res, Text, TextColor, TextFont, TextLayout, TextUiWriter, Transform, Update, Val, With};
use bevy::text::JustifyText::{Left, Right};
use bevy::text::TextSpan;
use bevy::ui::AlignItems::Start;
use bevy::ui::Val::Px;
use bevy::ui::{BackgroundColor, UiRect};
use bevy::utils::default;
use crate::prelude::GameState::Rendering;

pub struct HUDPlugin;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudIntegrityText;

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

fn init_hud(mut commands: Commands, assets_server: Res<AssetServer>) {
    let font = assets_server.load("fonts/FiraSans-Regular.ttf");

    let font_style = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..Default::default()
    };
    commands
        .spawn((
            Hud,
            BackgroundColor(Color::NONE),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: Start,
                flex_direction: FlexDirection::Column,
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
            hud_children
                .spawn((
                    Text::new("Integrity: "),
                    font_style.clone(),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Left),
                    HudIntegrityText,
                ))
                .with_child((TextSpan::default(), font_style.clone()));
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
    hud_integrity_text: Query<Entity, With<HudIntegrityText>>,
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
        if let Ok(integrity_text_entity) = hud_integrity_text.get_single() {
            let health = player_stats.0;
            *text_writer.text(integrity_text_entity, 1) =
                format!("{}/{}", health.current.trunc(), health.max);
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
