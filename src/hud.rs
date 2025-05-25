use bevy::prelude::AlignItems::Center;
use bevy::prelude::{App, AssetServer, BuildChildren, ChildBuild, Color, Commands, Component, JustifyContent, Node, Plugin, PositionType, Res, Startup, Text, TextColor, TextFont, TextLayout, Val};
use bevy::render::mesh::CylinderAnchor::Top;
use bevy::text::JustifyText::Left;
use bevy::text::TextSpan;
use bevy::ui::Val::Px;
use bevy::ui::{BackgroundColor, UiRect};
use bevy::ui::AlignItems::Start;
use bevy::utils::default;

pub struct HUDPlugin;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudIntegrityText;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_hud);
    }
}

fn init_hud(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
) {
    commands.spawn((
        Hud,
        BackgroundColor(Color::NONE),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: Start,
            position_type: PositionType::Absolute,
            left: Px(10.0),
            top: Px(10.0),
            padding: UiRect::all(Px(10.0)),
            justify_content: JustifyContent::FlexStart,
            ..default()
        }
    )).with_children(|hud_children| {
        hud_children.spawn((
            Text::new("Integrity: "),
            TextFont {
                font_size: 12.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(Left),
        ));
    }).with_child((
        TextSpan::default(),
        HudIntegrityText
    ));
}