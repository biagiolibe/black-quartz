use crate::prelude::MenuButton::{NewGame, QuitGame, Refill, Resume, Sell};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::ui::Interaction::Pressed;

pub struct MenuPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MenuState {
    None,
    #[default]
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
    NewGame,
    QuitGame,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (init_menu, handle_start_menu)
                .in_set(GameSystems::Rendering)
                .chain(),
        )
        .add_systems(
            OnEnter(MenuState::WorldBase),
            handle_base_menu,
        )
        .add_systems(
            OnEnter(MenuState::GameOver),
            handle_gameover_menu,
        )
        .add_systems(
            OnEnter(MenuState::Inventory),
            handle_inventory_menu,
        )
        .add_systems(
            OnEnter(MenuState::Settings),
            handle_settings_menu,
        )
        .add_systems(
            Update,
            handle_button_interaction.in_set(GameSystems::Ui),
        )
        .add_systems(
            OnExit(GameState::Menu),
            cleanup_menu,
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            cleanup_menu,
        );
    }
}

#[derive(Component)]
pub struct Menu;

pub fn init_menu(mut commands: Commands, assets_server: Res<AssetServer>) {
    info!("Initializing menu");
    let font = assets_server.load("fonts/FiraSans-Regular.ttf");

    let font_style = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..Default::default()
    };

    let parent_node = Node {
        width: Val::Percent(50.0),
        height: Val::Percent(50.0),
        position_type: PositionType::Absolute,
        left: Val::Percent(25.0),
        top: Val::Percent(25.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceEvenly,
        padding: UiRect::all(Val::Px(20.)),
        ..default()
    };
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            Menu,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            //Start game menu [index-0]
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                    Visibility::Hidden,
                ))
                .with_children(|popup| {
                    popup.spawn((
                        Text::new("Drill McDrillface"),
                        font_style.clone(),
                        TextColor(Color::WHITE),
                    ));

                    popup
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(60.0), // Occupa la parte centrale
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Start, // Allinea a sinistra
                                align_items: AlignItems::Center,        // Centro verticalmente
                                padding: UiRect::left(Val::Px(100.0)),  // Margine da sinistra
                                ..default()
                            },
                        ))
                        .with_children(|popup| {
                            popup.spawn((Button, NewGame)).with_children(|button| {
                                button.spawn((
                                    Text::new("Start game"),
                                    font_style.clone(),
                                    TextColor(Color::WHITE),
                                ));
                            });
                        });
                });
            //World base menu [index-1]
            parent
                .spawn((
                    parent_node.clone(),
                    BackgroundColor(Color::BLACK),
                    Visibility::Hidden,
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
            // Game over menu [index-2]
            parent
                .spawn((
                    parent_node.clone(),
                    BackgroundColor(Color::BLACK),
                    Visibility::Hidden,
                ))
                .with_children(|popup| {
                    popup.spawn((
                        Text::new("Game Over"),
                        font_style.clone(),
                        TextColor(Color::WHITE),
                    ));
                    popup.spawn((Button, NewGame)).with_children(|button| {
                        button.spawn((
                            Text::new("Restart game"),
                            font_style.clone(),
                            TextColor(Color::WHITE),
                        ));
                    });
                    popup.spawn((Button, QuitGame)).with_children(|button| {
                        button.spawn((
                            Text::new("Exit game"),
                            font_style.clone(),
                            TextColor(Color::WHITE),
                        ));
                    });
                });
        });
}
pub fn handle_start_menu(
    menu_query: Query<(Entity, &Children), With<Menu>>,
    visibility_query: Query<&mut Visibility>,
) {
    info!("start menu");
    if let Ok((entity, children)) = menu_query.get_single() {
        set_visibility_recursive(
            Visibility::Visible,
            entity,
            children,
            Some(0),
            visibility_query,
        );
    }
}

pub fn handle_base_menu(
    menu_query: Query<(Entity, &Children), With<Menu>>,
    visibility_query: Query<&mut Visibility>,
) {
    info!("base menu");
    if let Ok((entity, children)) = menu_query.get_single() {
        set_visibility_recursive(
            Visibility::Visible,
            entity,
            children,
            Some(1),
            visibility_query,
        );
    }
}
fn handle_button_interaction(
    interaction: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut player: Query<(&mut Inventory, &mut Fuel, &mut Currency), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    economy: Res<EconomyConfig>,
    mut loading_progress: ResMut<LoadingProgress>,
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
                        refill_tank(&mut fuel, &mut currency, &economy);
                    }
                }
                NewGame => {
                    next_state.set(GameState::Rendering);
                    next_menu_state.set(MenuState::None);
                }
                Resume => {
                    loading_progress.spawning_base = false;
                    loading_progress.rendering_map = false;
                    loading_progress.spawning_player = false;
                    loading_progress.init_camera = false;
                    next_state.set(GameState::Playing);
                    next_menu_state.set(MenuState::None);
                }
                QuitGame => {
                    next_state.set(GameState::GameOver);
                    next_menu_state.set(MenuState::None);
                }
                _ => {}
            }
        }
    }
}

fn refill_tank(fuel: &mut Fuel, currency: &mut Currency, economy_config: &Res<EconomyConfig>) {
    info!("Refill tank");
    let fuel_needed = fuel.max - fuel.current;
    let refill_cost = if fuel_needed >= 90.0 {
        info!("Fuel low: {}", fuel_needed);
        economy_config.fuel_refill_amount
    } else {
        fuel_needed * economy_config.fuel_price_per_unit as f32
    };
    let (amount_spent, refilled) = if currency.amount >= refill_cost as u32 {
        (refill_cost as u32, fuel_needed)
    } else {
        let refilled = (currency.amount / economy_config.fuel_price_per_unit) as f32;
        (currency.amount, refilled)
    };
    info!("Refilled {} spending: {}", refilled, amount_spent);
    currency.amount -= amount_spent;
    fuel.current += refilled;
}

fn sell_all_inventory(inventory: &mut Inventory, currency: &mut Currency) {
    if inventory.is_empty() {
        info!("No items to be sold");
        return;
    }
    let total_to_sell: u32 = inventory
        .items
        .iter()
        .map(|i| i.value * i.quantity as u32)
        .sum();

    info!("Total earned: {}", total_to_sell);
    currency.add_amount(total_to_sell);
    inventory.clear();
    info!("Inventory {:?}", inventory.items);
    info!("Currency: {}", currency.amount);
}

pub fn handle_inventory_menu() {
    info!("Inventory menu");
    //TODO implementation
}

pub fn handle_settings_menu() {
    info!("Settings menu");
    //TODO implementation
}

pub fn handle_gameover_menu(
    menu_query: Query<(Entity, &Children), With<Menu>>,
    visibility_query: Query<&mut Visibility>,
) {
    info!("Game over menu");
    if let Ok((entity, children)) = menu_query.get_single() {
        set_visibility_recursive(
            Visibility::Visible,
            entity,
            children,
            Some(2),
            visibility_query,
        );
    }
}

fn cleanup_menu(
    menu_query: Query<(Entity, &Children), With<Menu>>,
    visibility_query: Query<&mut Visibility>,
) {
    info!("Cleanup menu");
    if let Ok((entity, children)) = menu_query.get_single() {
        set_visibility_recursive(Visibility::Hidden, entity, children, None, visibility_query);
    }
}

fn set_visibility_recursive(
    visibility_target: Visibility,
    menu_entity: Entity,
    children: &Children,
    child_index: Option<usize>,
    mut visibility_query: Query<&mut Visibility>,
) {
    if let Ok(mut visibility) = visibility_query.get_mut(menu_entity) {
        *visibility = visibility_target;
    }

    children
        .iter()
        .enumerate()
        .filter(|(index, _)| child_index.is_none_or(|idx| *index == idx))
        .for_each(|(id, entity)| {
            if let Ok(mut visibility) = visibility_query.get_mut(*entity) {
                    *visibility = visibility_target;
            };
        });
}
