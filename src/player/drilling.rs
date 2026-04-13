use crate::map::{Tile, TileDestroyedEvent, WorldGrid, world_to_grid_position};
use crate::menu::MenuState;
use crate::prelude::MenuState::GameOver;
use crate::player::components::*;
use crate::prelude::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, Velocity};

pub fn drill(
    time: Res<Time<Fixed>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<
        (
            &Transform,
            &mut DrillState,
            &PlayerAttributes,
            &mut Fuel,
        ),
        With<Player>,
    >,
    world_grid: Res<WorldGrid>,
    mut query_tile: Query<(&mut Tile, &Transform), With<Tile>>,
    mut tile_destroyed_events: EventWriter<TileDestroyedEvent>,
) {
    if let Ok((transform, mut drill_state, attributes, mut fuel)) = player.single_mut() {
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

            if let Some(&entity) = world_grid.grid.get(&target_index) {
                if let Ok((mut tile, _)) = query_tile.get_mut(entity) {
                    *drill_state = DrillState::Drilling;

                    tile.drilling.integrity -=
                        attributes.drill_power * time.delta_secs() * (1.0 - tile.drilling.hardness);
                    fuel.current -= (1.0 / attributes.fuel_efficiency) * time.delta_secs();
                    if tile.drilling.integrity <= 0.0 {
                        tile_destroyed_events.write(TileDestroyedEvent {
                            tile_type: tile.tile_type,
                            position: target_index,
                            entity,
                        });
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

pub fn handle_loot_pickup(
    mut events: EventReader<TileDestroyedEvent>,
    mut player: Query<&mut Inventory, With<Player>>,
) {
    if let Ok(mut inventory) = player.single_mut() {
        for event in events.read() {
            if let Some(item) = event.tile_type.to_item() {
                inventory.add_item(item);
            }
        }
    }
}

pub fn collision_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut player: Query<
        (
            &Velocity,
            &PlayerAttributes,
            &mut DrillState,
            &Transform,
        ),
        With<Player>,
    >,
    tiles: Query<&Transform, With<Tile>>,
    mut impact_events: EventWriter<PlayerImpactEvent>,
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

                let (velocity, player_attributes, mut drill_state, player_pos) =
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
                        let damage = impact_speed * player_attributes.damage_factor;
                        impact_events.write(PlayerImpactEvent { impact_speed, damage });
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn apply_impact_damage(
    mut events: EventReader<PlayerImpactEvent>,
    mut player: Query<(&mut Health, &PlayerAttributes), With<Player>>,
) {
    if let Ok((mut health, attributes)) = player.single_mut() {
        for event in events.read() {
            let damage_reduction = (1.0 - attributes.armor_resistance.min(0.9)).max(0.1);
            let actual_damage = event.damage * damage_reduction;
            health.current -= actual_damage;
            info!(
                "Player impact: speed={:.1}, damage={:.1} (reduced to {:.1}), health={:.1}",
                event.impact_speed, event.damage, actual_damage, health.current
            );
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
