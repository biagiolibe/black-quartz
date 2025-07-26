use crate::prelude::GameState::Playing;
use crate::prelude::MenuButton::{Refill, Resume, Sell};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::ui::Interaction::Pressed;

pub struct MenuPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MenuState {
    #[default]
    None,
    Start,
    GameOver,
    Settings,
    Inventory,
    WorldBase,
}
#[derive(Component, Debug)]
pub enum MenuButton {
    Sell,
    Refill,
    Resume,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::WorldBase), handle_base_menu)
            .add_systems(OnEnter(MenuState::Start), handle_start_menu)
            .add_systems(OnEnter(MenuState::GameOver), handle_gameover_menu)
            .add_systems(OnEnter(MenuState::Inventory), handle_inventory_menu)
            .add_systems(OnEnter(MenuState::Settings), handle_settings_menu)
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
pub struct Menu;

pub fn handle_start_menu(mut commands: Commands, next_state: ResMut<NextState<MenuState>>) {
    info!("start menu");
    //TODO implementation
}

pub fn handle_base_menu(mut commands: Commands, assets_server: Res<AssetServer>) {
    info!("base menu");
    let font = assets_server.load("fonts/FiraSans-Regular.ttf");

    let font_style = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..Default::default()
    };
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(50.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        padding: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|popup| {
                    popup.spawn((
                        Text::new("World base"),
                        font_style.clone(),
                        TextColor(Color::WHITE),
                    ));

                    popup.spawn((Button, Sell)).with_children(|button| {
                        button.spawn((
                            Text::new("Sell inventory"),
                            font_style.clone(),
                            TextColor(Color::WHITE),
                        ));
                    });
                    popup.spawn((Button, Refill)).with_children(|button| {
                        button.spawn((
                            Text::new("Refill tank"),
                            font_style.clone(),
                            TextColor(Color::WHITE),
                        ));
                    });
                    popup.spawn((Button, Resume)).with_children(|button| {
                        button.spawn((
                            Text::new("Resume"),
                            font_style.clone(),
                            TextColor(Color::WHITE),
                        ));
                    });
                });
        });
}

fn handle_button_interaction(
    interaction: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut player: Query<(&mut Inventory, &mut Fuel, &mut Currency), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, button) in interaction.iter() {
        if *interaction == Pressed {
            match button {
                Sell => {
                    if let Ok((mut inventory, _, mut currency)) = player.get_single_mut() {
                        sell_all_inventory(&mut inventory, &mut currency);
                    }
                }
                Refill => {
                    if let Ok((_, mut fuel, mut currency)) = player.get_single_mut() {
                        refill_tank(&mut fuel, &mut currency);
                    }
                }
                Resume => {
                    next_menu_state.set(MenuState::None);
                    next_state.set(Playing);
                }
            }
        }
    }
}

fn refill_tank(fuel: &mut Fuel, currency: &mut Currency) {
    println!("Recharge fuel");
}

fn sell_all_inventory(inventory: &mut Inventory, currency: &mut Currency) {
    println!(
        "Selling inventory {:?}, having {:?} money",
        inventory.items, currency
    );
    
}

pub fn handle_inventory_menu(mut commands: Commands, next_state: ResMut<NextState<MenuState>>) {
    info!("Inventory menu");
    //TODO implementation
}

pub fn handle_settings_menu(mut commands: Commands, next_state: ResMut<NextState<MenuState>>) {
    info!("Settings menu");
    //TODO implementation
}

pub fn handle_gameover_menu(mut commands: Commands, next_state: ResMut<NextState<MenuState>>) {
    info!("Game over menu");
    //TODO implementation
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
