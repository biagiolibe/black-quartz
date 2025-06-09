use crate::player::Player;
use crate::prelude::{world_to_grid_position, Health, Inventory};
use bevy::prelude::{
    App, AssetServer, BuildChildren, ChildBuild, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PositionType, Query, Res, Startup, Text, TextColor, TextFont,
    TextLayout, TextUiWriter, Transform, Update, Val, With,
};
use bevy::text::JustifyText::Left;
use bevy::text::TextSpan;
use bevy::ui::AlignItems::Start;
use bevy::ui::Val::Px;
use bevy::ui::{BackgroundColor, UiRect};
use bevy::utils::default;
use bevy_rapier2d::prelude::Velocity;

pub struct HUDPlugin;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudIntegrityText;

#[derive(Component)]
struct HudDepthText;

#[derive(Component)]
struct HudVelocityText;

#[derive(Component)]
struct HudInventoryText;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_hud)
            .add_systems(Update, update_hud);
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
            /// Integrity stat
            hud_children
                .spawn((
                    Text::new("Integrity: "),
                    font_style.clone(),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Left),
                    HudIntegrityText,
                ))
                .with_child((TextSpan::default(), font_style.clone()));
            /// Depth stat
            hud_children
                .spawn((
                    Text::new("Depth: "),
                    font_style.clone(),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Left),
                    HudDepthText,
                ))
                .with_child((TextSpan::default(), font_style.clone()));
            /// Velocity stat
            hud_children
                .spawn((
                    Text::new("Velocity: "),
                    font_style.clone(),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(Left),
                    HudVelocityText,
                ))
                .with_child((TextSpan::default(), font_style.clone()));
            /// Inventory stat
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
    hud_velocity_text: Query<Entity, With<HudVelocityText>>,
    hud_inventory_text: Query<Entity, With<HudInventoryText>>,
    player: Query<(&Health, &Transform, &Velocity, &Inventory), With<Player>>,
    mut text_writer: TextUiWriter,
) {
    /// Updating hud integrity stats
    let player_stats = player.single();
    if let Ok(integrity_text_entity) = hud_integrity_text.get_single() {
        let health = player_stats.0;
        *text_writer.text(integrity_text_entity, 1) =
            format!("{}/{}", health.current.trunc(), health.max);
    }
    if let Ok(depth_text_entity) = hud_depth_text.get_single() {
        let position = world_to_grid_position(player_stats.1.translation.truncate());
        *text_writer.text(depth_text_entity, 1) = format!("{}", position.1);
    }
    if let Ok(velocity_text_entity) = hud_velocity_text.get_single() {
        let velocity = player_stats.2;
        *text_writer.text(velocity_text_entity, 1) = format!("{}", velocity.linvel.y);
    }
    if let Ok(inventory_text_entity) = hud_inventory_text.get_single() {
        let inventory = player_stats.3;
        *text_writer.text(inventory_text_entity, 1) = format!("{}", inventory.size());
    }
}
