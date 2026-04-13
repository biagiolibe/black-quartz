use crate::BlackQuartzCamera;
use crate::map::{Tile, WorldGrid, world_grid_position_to_idx, world_to_grid_position};
use crate::map::TileType::Empty;
use crate::menu::MenuState;
use crate::prelude::MenuState::GameOver;
use crate::player::components::*;
use crate::prelude::{CameraShake, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, Velocity};
use std::time::Duration;

pub fn drill(
    time: Res<Time<Fixed>>,
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
        if *drill_state == DrillState::Drilling && direction == None {
            *drill_state = DrillState::Idle;
        }
        if *drill_state != DrillState::Idle && *drill_state != DrillState::Drilling {
            direction = None;
        }
        if let Some((dx, dy)) = direction {
            let target_index = (current_position.0 + dx, current_position.1 + dy);

            if let Some(entity) = world_grid.grid.get(&target_index) {
                if let Ok((mut tile, _)) = query_tile.get_mut(*entity) {
                    *drill_state = DrillState::Drilling;

                    tile.drilling.integrity -=
                        attributes.drill_power * time.delta_secs() * (1.0 - tile.drilling.hardness);
                    if tile.drilling.integrity <= 0.0 {
                        commands.entity(*entity).despawn();
                        world_grid.grid.remove(&target_index);
                        let grid_id =
                            world_grid_position_to_idx((target_index.0, target_index.1));
                        world_grid.tiles[grid_id.1][grid_id.0] = Empty;

                        if let Some(item) = tile.tile_type.to_item() {
                            inventory.add_item(item);
                        }
                        *drill_state = DrillState::Idle;
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

pub fn collision_detection(
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

                let grid_tile_pos =
                    world_to_grid_position(tile_transform.translation.truncate());
                let grid_player_pos =
                    world_to_grid_position(player_pos.translation.truncate());

                if grid_tile_pos.0 == grid_player_pos.0 && *drill_state != DrillState::Drilling {
                    *drill_state = DrillState::Idle;
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
                                timer: Timer::new(
                                    Duration::from_secs_f32(0.1),
                                    TimerMode::Once,
                                ),
                                intensity: 3.0,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn death_detection(
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
