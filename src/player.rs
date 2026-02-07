use crate::BlackQuartzCamera;
use crate::map::TileType::Empty;
use crate::map::{TILE_SIZE, Tile, WorldGrid};
use crate::menu::MenuState;
use crate::player::DrillState::{Drilling, Falling, Flying, Idle};
use crate::prelude::GameSystems::{Rendering, Running};
use crate::prelude::MenuState::GameOver;
use crate::prelude::{CameraShake, DrillAnimation};
use crate::prelude::{
    GameAssets, GameState, LoadingProgress, world_grid_position_to_idx, world_to_grid_position,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionEvent, Damping, GravityScale, LockedAxes, QueryFilter,
    ReadRapierContext, RigidBody, ShapeCastOptions, Velocity,
};
use bevy_rapier2d::rapier::prelude::SharedShape;
use std::collections::HashSet;
use std::time::Duration;

pub struct PlayerPlugin;

/// This plugin handles player-related stuff like movement
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Rendering),
            spawn_player.in_set(Rendering),
        )
        .add_systems(
            Update,
            (
                update_player_on_state_changes.in_set(Running),
                update_player_direction.in_set(Running),
                (move_player, drill).in_set(Running),
                falling_detection.in_set(Running),
                collision_detection.in_set(Running),
                death_detection.in_set(Running),
            )
                .run_if(in_state(GameState::Playing))
                .chain(),
        );
    }
}

#[derive(Component)]
#[require(
    Inventory,
    Health,
    Fuel,
    FieldOfView,
    DrillState,
    PlayerAttributes,
    Currency,
    DrillAnimation,
    PlayerDirection
)]
pub struct Player;

#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct PlayerAttributes {
    drill_power: f32,
    damage_factor: f32,
    armor_resistance: f32,
    ground_speed_factor: f32,
    flying_speed_factor: f32,
    fuel_efficiency: f32,
}

impl Default for PlayerAttributes {
    fn default() -> Self {
        Self {
            drill_power: 1.0,
            damage_factor: 0.05, //TODO deprecate in flavor of armor_resistance
            armor_resistance: 1.0,
            ground_speed_factor: 200.0,
            flying_speed_factor: 200.0,
            fuel_efficiency: 0.3,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum PlayerDirection {
    Left,
    Right,
}

impl Default for PlayerDirection {
    fn default() -> Self {
        PlayerDirection::Right
    }
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    #[allow(dead_code)]
    pub max: f32,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Component)]
pub struct Fuel {
    pub current: f32,
    pub max: f32,
}

impl Default for Fuel {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}
#[derive(Component, PartialEq, Eq, Debug, Clone, Copy, Default)]
pub enum DrillState {
    #[default]
    Idle,
    Flying,
    Drilling,
    Falling,
}

#[derive(Component, Clone, PartialEq, Debug)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<(i32, i32)>,
    pub radius: i32,
    pub dirty: bool,
}
impl Default for FieldOfView {
    fn default() -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: 10,
            dirty: false,
        }
    }
}

#[derive(Component, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub quantity: usize,
    pub value: u32,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
    capacity: usize,
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: 10,
        }
    }
}

impl Inventory {
    fn add_item(&mut self, new_item: Item) {
        if self.size() + new_item.quantity <= self.capacity {
            if let Some(existing) = self.items.iter_mut().find(|i| i.id == new_item.id) {
                existing.quantity += new_item.quantity;
            } else {
                self.items.push(new_item);
            }
        } else {
            info!("Inventory full!");
        }
    }

    pub fn size(&self) -> usize {
        self.items.iter().map(|i| i.quantity).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn print_items(&self) -> String {
        self.items
            .iter()
            .map(|i| format!("{} x{}", i.name, i.quantity))
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[derive(Component, Debug)]
pub struct Currency {
    pub amount: u32,
}
impl Default for Currency {
    fn default() -> Self {
        Self { amount: 100 }
    }
}

impl Currency {
    pub fn add_amount(&mut self, amount: u32) {
        self.amount += amount;
    }
}

pub fn spawn_player(
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut loading_progress: ResMut<LoadingProgress>,
) {
    if let Ok(entity) = player.single() {
        commands.entity(entity).despawn();
    }
    info!("Spawning Drilling Machine (Player)");
    commands
        .spawn((
            Player,
            Health::default(),
            Fuel::default(),
            Inventory::default(),
            Currency::default(),
            FieldOfView::default(),
            DrillState::default(),
            PlayerAttributes::default(),
            PlayerDirection::default(),
            DrillAnimation::default(),
        ))
        .insert((
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            Sprite {
                image: game_assets.player.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_assets.player.texture_layout.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, 25.0, 0.0),
            RigidBody::Dynamic,
            Collider::capsule_y((TILE_SIZE - 28.0) / 2f32, 14.0),
            ActiveEvents::COLLISION_EVENTS,
            GravityScale(1.0),
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
        ));
    loading_progress.spawning_player = true;
}
pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<
        (&mut Velocity, &mut DrillState, &PlayerAttributes, &mut Fuel),
        With<Player>,
    >,
) {
    if let Ok((mut velocity, mut drill_state, attributes, mut fuel)) = query_player.single_mut()
    {
        let direction = keyboard_input
            .get_pressed()
            .fold(Vec2::ZERO, |mut direction, key| {
                match key {
                    KeyCode::ArrowLeft => direction.x -= 1.0,
                    KeyCode::ArrowRight => direction.x += 1.0,
                    KeyCode::ArrowUp => direction.y += 1.0,
                    _ => (),
                }
                direction
            });
        if direction != Vec2::ZERO {
            if direction.x != 0.0 {
                velocity.linvel.x = direction.x * attributes.ground_speed_factor;
            }
            if direction.y != 0.0 {
                velocity.linvel.y = direction.y * attributes.flying_speed_factor;
                *drill_state = Flying;
            }
            fuel.current -= (1.0 / attributes.fuel_efficiency) * time.delta_secs();
        }
    }
}

fn update_player_on_state_changes(
    mut query: Query<(&DrillState, &mut Damping, &mut Sprite), (With<Player>, Changed<DrillState>)>,
) {
    if let Ok((state, mut damping, mut sprite)) = query.single_mut() {
        debug!(
            "update_player_on_state_changes {{ DrillState: {:?}, Damping: {:?} }} ",
            state, damping
        );
        if *state == Idle {
            damping.linear_damping = 10.0;
        } else {
            damping.linear_damping = 0.5;
        }
        if let Some(texture_sprite) = &mut sprite.texture_atlas {
            match state {
                Idle | Falling => texture_sprite.index = 0,
                Flying => texture_sprite.index = 1,
                Drilling => texture_sprite.index = 2,
            };
        };
    }
}

// Sistema per gestire il flip del player basato sulla direzione del movimento
fn update_player_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut PlayerDirection, &mut Sprite), With<Player>>,
) {
    if let Ok((_transform, mut direction, mut sprite)) = player_query.single_mut() {
        // Controlla input orizzontale
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if *direction != PlayerDirection::Left {
                *direction = PlayerDirection::Left;
                // Capovolgimento orizzontale - scala X negativa
                sprite.flip_x = true;
            }
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            if *direction != PlayerDirection::Right {
                *direction = PlayerDirection::Right;
                // Direzione normale - scala X positiva
                sprite.flip_x = false;
            }
        }
    }
}

fn drill(
    time: Res<Time>,
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<
        (
            &Transform,
            &mut Inventory,
            &mut DrillState,
            &PlayerAttributes,
        ),
        With<Player>,
    >,
    mut world_grid: ResMut<WorldGrid>,
    mut query_tile: Query<(&mut Tile, &Transform), With<Tile>>,
) {
    if let Ok((transform, mut inventory, mut drill_state, attributes)) = player.single_mut() {
        let position = transform.translation.truncate();
        let current_position = world_to_grid_position(position);

        let mut direction = keyboard_input.get_pressed().find_map(|key| match key {
            KeyCode::ArrowLeft => Some((-1, 0)),
            KeyCode::ArrowRight => Some((1, 0)),
            KeyCode::ArrowDown => Some((0, -1)),
            _ => None,
        });
        if *drill_state == Drilling && direction == None {
            *drill_state = Idle;
        }
        if *drill_state != Idle && *drill_state != Drilling {
            direction = None;
        }
        if let Some((dx, dy)) = direction {
            let target_index = (current_position.0 + dx, current_position.1 + dy);

            if let Some(entity) = world_grid.grid.get(&target_index) {
                if let Ok((mut tile, _)) = query_tile.get_mut(*entity) {
                    //Update drilling state
                    *drill_state = Drilling;

                    tile.drilling.integrity -=
                        attributes.drill_power * time.delta_secs() * (1.0 - tile.drilling.hardness);
                    if tile.drilling.integrity <= 0.0 {
                        commands.entity(*entity).despawn();
                        world_grid.grid.remove(&target_index);
                        let grid_id = world_grid_position_to_idx((target_index.0, target_index.1));
                        world_grid.tiles[grid_id.1][grid_id.0] = Empty;

                        //add item to inventory
                        if let Some(item) = tile.tile_type.to_item() {
                            inventory.add_item(item);
                        }
                        //Update drilling state
                        *drill_state = Idle;
                    }
                } else {
                    warn!(
                        "No tile exists to be drilled on position {:?}",
                        target_index
                    );
                };
            }
        }
    }
}

fn collision_detection(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player: Query<
        (
            &Velocity,
            &mut Health,
            &PlayerAttributes,
            &mut DrillState,
            &Transform,
        ),
        With<Player>,
    >,
    tiles: Query<&Transform, With<Tile>>,
    mut camera: Query<Entity, With<BlackQuartzCamera>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(collider1, collider2, _) => {
                let (player_entity, tile_entity) =
                    if player.get(*collider1).is_ok() && tiles.get(*collider2).is_ok() {
                        (*collider1, *collider2)
                    } else if player.get(*collider2).is_ok() && tiles.get(*collider1).is_ok() {
                        (*collider2, *collider1)
                    } else {
                        continue;
                    };

                let (velocity, mut health, player_attributes, mut drill_state, player_pos) =
                    player.get_mut(player_entity).unwrap();
                let tile_transform = tiles.get(tile_entity).unwrap();

                let grid_tile_pos = world_to_grid_position(tile_transform.translation.truncate());
                let grid_player_pos = world_to_grid_position(player_pos.translation.truncate());

                if grid_tile_pos.0 == grid_player_pos.0 && *drill_state != Drilling {
                    // collision from bottom
                    *drill_state = Idle;
                    let impact_speed = velocity.linvel.y.abs();
                    if impact_speed > 300.0 {
                        let damage_amount = impact_speed * player_attributes.damage_factor;
                        health.current -= damage_amount;
                        info!(
                            "Player collision detected, impact speed {:?}, damage {:?}, player integrity {:?}",
                            impact_speed, damage_amount, health.current
                        );
                        for entity in camera.iter_mut() {
                            commands.entity(entity).insert(CameraShake {
                                base_position: None,
                                timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once),
                                intensity: 3.0, // Intensità pixel del tremolio
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn falling_detection(
    mut player_query: Query<(&Velocity, &Transform, &mut DrillState), With<Player>>,
    read_rapier_context: ReadRapierContext,
) {
    if let Ok((velocity, transform, mut drill_state)) = player_query.single_mut() {
        let player_pos = transform.translation.truncate();

        if let Ok(context) = read_rapier_context.single() {
            if let Some((_, toi)) = context.cast_shape(
                player_pos,
                0.0,
                Vec2::NEG_Y,
                &*SharedShape::cuboid(8.0, 16.0), // A little rectangle under the player
                ShapeCastOptions {
                    stop_at_penetration: false,
                    ..default()
                },
                QueryFilter::default(),
            ) {
                if toi.time_of_impact > 10.0 && velocity.linvel.y < -1.0 {
                    *drill_state = Falling;
                }
            }
        }
    }
}

fn death_detection(
    player: Query<(&Health, &Fuel), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    if let Ok((health, fuel)) = player.single() {
        if health.current <= 0.0 || fuel.current <= 0.0 {
            next_menu_state.set(GameOver);
            next_state.set(GameState::Menu);
        }
    }
}
