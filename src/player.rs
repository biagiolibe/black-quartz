use crate::map::TileType::Empty;
use crate::map::{TILE_SIZE, Tile, WorldGrid};
use crate::player::DrillState::{Drilling, Falling, Flying, Idle};
use crate::prelude::{GameAssets, GameState, world_grid_position_to_idx, world_to_grid_position};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionEvent, Damping, GravityScale, LockedAxes, QueryFilter,
    ReadRapierContext, RigidBody, ShapeCastOptions, Velocity,
};
use std::collections::{HashSet, VecDeque};
pub struct PlayerPlugin;

#[derive(Component)]
#[require(
    Inventory,
    Health,
    Fuel,
    FieldOfView,
    DrillState,
    PlayerAttributes,
    Currency
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

#[derive(Component)]
pub struct Health {
    pub current: f32,
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

#[derive(Component, Clone, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<(i32, i32)>,
    radius: i32,
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

#[derive(Component)]
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
            println!("Inventory full!");
        }
    }

    pub fn size(&self) -> usize {
        self.items.iter().map(|i| i.quantity).sum()
    }

    pub fn print_items(&self) -> String {
        self.items
            .iter()
            .map(|i| format!("{} x{}", i.name, i.quantity))
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[derive(Component)]
pub struct Currency {
    pub amount: u32,
}
impl Default for Currency {
    fn default() -> Self {
        Self { amount: 100 }
    }
}

/// This plugin handles player-related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                update_player_on_state_changes,
                update_fov,
                (
                    (move_player, drill).run_if(in_state(GameState::Playing)),
                    falling_detection,
                    collision_detection,
                )
                    .chain(),
            ),
        );
    }
}
pub fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Drilling Machine (Player)
    commands
        .spawn((
            Player,
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            Sprite {
                image: game_assets.player.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_assets.player.texture_layout.clone(),
                    index: 2,
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
        ))
        .insert(LockedAxes::ROTATION_LOCKED);
}
pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<
        (
            &mut Velocity,
            &mut DrillState,
            &PlayerAttributes,
            &mut Fuel,
            &Transform,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut velocity, mut drill_state, attributes, mut fuel, position)) =
        query_player.get_single_mut()
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
    if let Ok((state, mut damping, mut sprite)) = query.get_single_mut() {
        if *state == Idle {
            damping.linear_damping = 10.0;
        } else {
            damping.linear_damping = 0.5;
        }
        if let Some(texture_sprite) = &mut sprite.texture_atlas {
            match state {
                Idle => texture_sprite.index = 2,
                Flying => texture_sprite.index = 3,
                Falling => texture_sprite.index = 1,
                Drilling => texture_sprite.index = 0,
            };
        };
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
    if let Ok((transform, mut inventory, mut drill_state, attributes)) = player.get_single_mut() {
        let position = transform.translation.truncate();
        let current_position = world_to_grid_position(position);

        let mut direction = keyboard_input.get_pressed().find_map(|key| match key {
            KeyCode::ArrowLeft => Some((-1, 0)),
            KeyCode::ArrowRight => Some((1, 0)),
            KeyCode::ArrowDown => Some((0, -1)),
            _ => None,
        });
        if *drill_state != Idle && *drill_state != Drilling {
            direction = None;
        }
        if let Some((dx, dy)) = direction {
            let target_index = (current_position.0 + dx, current_position.1 + dy);

            if let Some(entity) = world_grid.grid.get(&target_index) {
                if let Ok((mut tile, _)) = query_tile.get_mut(*entity) {
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
                    }
                    //Update drilling state
                    *drill_state = Drilling;
                } else {
                    println!(
                        "No tile exists to be drilled on position {:?}",
                        target_index
                    );
                };
            }
        }
    }
}

fn collision_detection(
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

                if grid_tile_pos.0 == grid_player_pos.0 {
                    // collision from bottom
                    *drill_state = Idle;
                    let impact_speed = velocity.linvel.y.abs();
                    if impact_speed > 300.0 {
                        let damage_amount = impact_speed * player_attributes.damage_factor;
                        health.current -= damage_amount;
                        println!(
                            "Player collision detected, impact speed {:?}, damage {:?}, player integrity {:?}",
                            impact_speed, damage_amount, health.current
                        );
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
    if let Ok((velocity, transform, mut drill_state)) = player_query.get_single_mut() {
        let player_pos = transform.translation.truncate();

        if let Some((_, toi)) = read_rapier_context.single().cast_shape(
            player_pos,
            0.0,
            Vec2::NEG_Y,
            &Collider::cuboid(8.0, 16.0), // A little rectangle under the player
            ShapeCastOptions {
                stop_at_penetration: false,
                ..default()
            },
            QueryFilter::default(),
        ) {
            if toi.time_of_impact > 1.0 && velocity.linvel.y < -1.0 {
                *drill_state = Falling;
            }
        }
    }
}

//TODO improve in some way
pub fn update_fov(
    mut player_query: Query<(&Transform, Mut<FieldOfView>), With<Player>>,
    world_grid: ResMut<WorldGrid>,
) {
    let (player_transform, mut fov) = player_query.single_mut();
    let player_pos = IVec2::from(world_to_grid_position(
        player_transform.translation.truncate(),
    ));

    let mut queue = VecDeque::new();
    queue.push_back((player_pos, 0));
    let mut visited = vec![player_pos];

    while let Some((pos, dist)) = queue.pop_front() {
        if dist > fov.radius {
            continue;
        }
        if visited.contains(&pos) && player_pos != pos {
            //println!("tile already visited {:?}", pos);
            continue;
        }
        // Add position to list of visited
        visited.push(pos);

        let (id_x, id_y) = world_grid_position_to_idx((pos.x, pos.y));

        if id_x >= world_grid.tiles[0].len() || id_y >= world_grid.tiles.len() {
            //println!("out of bounds ({},{})", id_x, id_y);
            continue;
        }

        //Add to player's fov
        fov.visible_tiles.insert((pos.x, pos.y));
        fov.dirty = true;

        if world_grid.tiles[id_y][id_x] != Empty {
            continue; // Blocca la propagazione della visibilità
        }

        let neighbors = [
            IVec2::new(pos.x + 1, pos.y + 1),
            IVec2::new(pos.x + 1, pos.y),
            IVec2::new(pos.x + 1, pos.y - 1),
            IVec2::new(pos.x, pos.y - 1),
            IVec2::new(pos.x - 1, pos.y - 1),
            IVec2::new(pos.x - 1, pos.y),
            IVec2::new(pos.x - 1, pos.y + 1),
            IVec2::new(pos.x, pos.y + 1),
        ];

        for n in neighbors {
            queue.push_back((n, dist + 1));
        }
    }
}
