use crate::map::TileType::Empty;
use crate::map::{GRID_HEIGHT, GRID_WIDTH, TILE_SIZE, Tile, WorldGrid};
use crate::player::DrillState::{Drilling, Falling, Flying, Idle};
use crate::prelude::{GameAssets, GameState, world_to_grid_position};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionEvent, GravityScale, LockedAxes, QueryFilter,
    ReadRapierContext, RigidBody, ShapeCastOptions, Velocity,
};
use std::collections::{HashSet, VecDeque};

pub const PLAYER_DRILLING_STRENGTH: f32 = 1.0; //TODO: add as component of the player

pub const PLAYER_ARMOR_RESISTANCE: f32 = 1.0; //TODO: add as component of the player and rename
pub const PLAYER_SPEED_FACTOR: f32 = 200.0; //TODO: add as component of the player
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Damage {
    pub factor: f32,
}

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy)]
pub enum DrillState {
    Idle,
    Flying,
    Drilling,
    Falling,
}

#[derive(Component, Clone, PartialEq)]
pub struct FieldOfView {
    visible_tiles: HashSet<(i32, i32)>,
    visited_tiles: HashSet<(i32, i32)>,
    radius: i32,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    update_player_sprite,
                    update_fov,
                    (
                        move_player.run_if(in_state(GameState::Playing)),
                        drill,
                        falling_detection,
                        collision_detection,
                    )
                        .chain(),
                ),
            );
    }
}
fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Drilling Machine (Player)
    commands
        .spawn((
            Player,
            Health {
                current: 100.0,
                max: 100.0,
            },
            Damage { factor: 0.05 },
            Idle,
            Sprite {
                image: game_assets.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_assets.texture_layout.clone(),
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
            FieldOfView {
                visible_tiles: HashSet::new(),
                visited_tiles: HashSet::new(),
                radius: 2,
            },
        ))
        .insert(LockedAxes::ROTATION_LOCKED);
}
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Velocity, &mut DrillState), With<Player>>,
) {
    if let Ok((mut velocity, mut drill_state)) = query_player.get_single_mut() {
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
            velocity.linvel = direction.normalize() * PLAYER_SPEED_FACTOR;
            if velocity.linvel.y > 0.0 {
                *drill_state = Flying;
            }
        }
    }
}

fn update_player_sprite(
    mut query: Query<(&DrillState, &mut Sprite), (With<Player>, Changed<DrillState>)>,
) {
    if let Ok((state, mut sprite)) = query.get_single_mut() {
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
    mut player: Query<(&Transform, &mut DrillState), With<Player>>,
    mut world_grid: ResMut<WorldGrid>,
    mut query_tile: Query<(&mut Tile, &Transform), With<Tile>>,
) {
    if let Ok((transform, mut drill_state)) = player.get_single_mut() {
        let position = transform.translation.truncate();
        let current_position = world_to_grid_position(position);

        let direction = keyboard_input.get_pressed().find_map(|key| match key {
            KeyCode::ArrowLeft => Some((-1, 0)),
            KeyCode::ArrowRight => Some((1, 0)),
            KeyCode::ArrowDown => Some((0, -1)),
            _ => None,
        });
        if let Some((dx, dy)) = direction {
            let target_index = (current_position.0 + dx, current_position.1 + dy);

            if let Some(entity) = world_grid.grid.get(&target_index) {
                if let Ok((mut tile, _)) = query_tile.get_mut(*entity) {
                    tile.drilling.integrity -= PLAYER_DRILLING_STRENGTH
                        * time.delta_secs()
                        * (1.0 - tile.drilling.hardness);
                    if tile.drilling.integrity <= 0.0 {
                        commands.entity(*entity).despawn();
                        world_grid.grid.remove(&target_index);
                        world_grid.tiles[(target_index.1 + (GRID_HEIGHT) as i32) as usize]
                            [(target_index.0 + (GRID_WIDTH/2) as i32) as usize] = Empty;
                        println!(
                            "Drilled tile at {:?} with player on position {:?}",
                            target_index, current_position
                        );
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
    mut player: Query<(&Velocity, &mut Health, &Damage, &mut DrillState), With<Player>>,
    tiles: Query<&Tile, With<Tile>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let (player_entity, _) =
                    if player.get(*entity1).is_ok() && tiles.get(*entity2).is_ok() {
                        (*entity1, *entity2)
                    } else if player.get(*entity2).is_ok() && tiles.get(*entity1).is_ok() {
                        (*entity2, *entity1)
                    } else {
                        continue;
                    };

                let (velocity, mut health, damage, mut drill_state) =
                    player.get_mut(player_entity).unwrap();

                *drill_state = Idle;
                let impact_speed = velocity.linvel.y.abs();
                if impact_speed > 300.0 {
                    let damage_amount = impact_speed * damage.factor;
                    health.current -= damage_amount;
                    println!(
                        "Player collision detected, impact speed {:?}, damage {:?}, player integrity {:?}",
                        impact_speed, damage_amount, health.current
                    );
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
            &Collider::cuboid(8.0, 16.0), // Un piccolo rettangolo sotto il player
            ShapeCastOptions {
                stop_at_penetration: false,
                ..default()
            },
            QueryFilter::default(),
        ) {
            if toi.time_of_impact > 0.2 && velocity.linvel.y < -0.2 {
                *drill_state = Falling;
            }
        }
    }
}

pub fn update_fov(
    mut player_query: Query<(&Transform, &mut FieldOfView), With<Player>>,
    mut world_grid: ResMut<WorldGrid>,
) {
    let (player_transform, mut fov) = player_query.single_mut();
    let player_pos = IVec2::from(world_to_grid_position(
        player_transform.translation.truncate(),
    ));

    println!(
        "Converted {} to grid position {}",
        player_transform.translation.truncate(),
        player_pos
    );
    let max_radius = 10; // o qualsiasi valore tu voglia

    let mut queue = VecDeque::new();
    queue.push_back((player_pos, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        if dist > max_radius {
            continue;
        }
        let id_x = pos.x + (GRID_WIDTH / 2) as i32;
        let id_y = pos.y + GRID_HEIGHT as i32;

        if id_x < 0
            || id_y < 0
            || id_x >= world_grid.tiles[0].len() as i32
            || id_y >= world_grid.tiles.len() as i32
        {
            println!("out of bounds ({},{})", id_x, id_y);
            continue;
        }

        let x = id_x as usize;
        let y = id_y as usize;

        fov.visible_tiles.insert((pos.x, pos.y));
        if fov.visited_tiles.get(&(pos.x, pos.y)).is_some() {
            continue;
        }
        fov.visited_tiles.insert((pos.x, pos.y));
        //println!("visible_tiles {:?}", fov.visible_tiles);
        if world_grid.tiles[id_y as usize][id_x as usize] != Empty {
            //println!("tile in ({},{}) is not empty", pos_x, pos_y);
            continue; // Blocca la propagazione della visibilità
        }

        let neighbors = [
            IVec2::new(pos.x + 1, pos.y),
            IVec2::new(pos.x - 1, pos.y),
            IVec2::new(pos.x, pos.y + 1),
            IVec2::new(pos.x, pos.y - 1),
        ];

        for n in neighbors {
            queue.push_back((n, dist + 1));
        }
    }
}
