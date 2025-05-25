use crate::map::{TILE_SIZE, Tile, WorldGrid};
use crate::prelude::{GameAssets, GameState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionEvent, GravityScale, LockedAxes,
    RigidBody, Velocity,
};
use crate::BlackQuartzCamera;

pub const PLAYER_DRILLING_STRENGTH: f32 = 0.2; //TODO: add as component of the player
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

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    collision_detection,
                    drill,
                    move_player.run_if(in_state(GameState::Playing)),
                )
                    .chain(),
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
            Sprite {
                image: game_assets.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_assets.texture_layout.clone(),
                    index: 2,
                }),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, 50.0, 0.0),
            RigidBody::Dynamic,
            Collider::capsule_y((TILE_SIZE - 28.0) / 2f32, 14.0),
            ActiveEvents::COLLISION_EVENTS,
            GravityScale(1.0),
            Velocity::zero(),
        ))
        .insert(LockedAxes::ROTATION_LOCKED);
}
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<(&mut Velocity, &Transform), With<Player>>,
    mut query_camera: Query<(&mut Transform, &Camera), (With<BlackQuartzCamera>, Without<Player>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut velocity, player_pos)) = query_player.get_single_mut() {
        keyboard_input
            .get_pressed()
            .fold(Vec2::ZERO, |mut acceleration, key| {
                match key {
                    KeyCode::ArrowLeft => acceleration.x -= 1.0,
                    KeyCode::ArrowRight => acceleration.x += 1.0,
                    KeyCode::ArrowUp => acceleration.y += 1.0,
                    KeyCode::ArrowDown => acceleration.y -= 1.0,
                    _ => (),
                }
                velocity.linvel = acceleration * 100.0;
                acceleration
            });
        //

        // Camera handling
        let window = windows.single();

        let (mut camera_pos, camera) = query_camera.single_mut();
        let x_pos = (player_pos.translation.x).min(window.width());
        let y_pos = (player_pos.translation.y).min(window.height());
        camera_pos.translation = Vec3::new(x_pos, y_pos, camera_pos.translation.z);
    }
}

fn drill(
    time: Res<Time>,
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    mut world_grid: ResMut<WorldGrid>,
    mut query_tile: Query<(&mut Tile, &Transform), With<Tile>>,
) {
    if let Ok(transform) = player.get_single() {
        let position = transform.translation;
        //println!("Player position in world px: {:?}", position);
        let current_position = (
            ((position.x + (position.x / position.x.abs()) * TILE_SIZE / 2.0) / TILE_SIZE).trunc()
                as i32,
            ((position.y + (position.y / position.y.abs()) * TILE_SIZE / 2.0) / TILE_SIZE).trunc()
                as i32,
        );
        //println!("Computed current position: {:?}", current_position);

        let direction = keyboard_input.get_pressed().find_map(|key| match key {
            KeyCode::ArrowLeft => Some((-1, 0)),
            KeyCode::ArrowRight => Some((1, 0)),
            KeyCode::ArrowDown => Some((0, -1)),
            KeyCode::ArrowUp => Some((0, 1)),
            _ => None,
        });
        if let Some((dx, dy)) = direction {
            let target_index = (current_position.0 + dx, current_position.1 + dy);

            if let Some(entity) = world_grid.grid.get(&target_index) {
                if let Ok((mut tile, transform)) = query_tile.get_mut(*entity) {
                    tile.drilling.integrity -= PLAYER_DRILLING_STRENGTH
                        * time.delta_secs()
                        * (1.0 - tile.drilling.hardness);
                    //println!("tile integrity {:?}", tile.drilling.integrity);
                    if tile.drilling.integrity <= 0.0 {
                        commands.entity(*entity).despawn();
                        world_grid.grid.remove(&target_index);
                        println!(
                            "Drilled tile at {:?} with player on position {:?}",
                            target_index, current_position
                        );
                    }
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
    mut player: Query<(&Velocity, &mut Health, &Damage), With<Player>>,
    tiles: Query<&Tile, With<Tile>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let (player_entity, tile_entity) =
                    if player.get(*entity1).is_ok() && tiles.get(*entity2).is_ok() {
                        (*entity1, *entity2)
                    } else if player.get(*entity2).is_ok() && tiles.get(*entity1).is_ok() {
                        (*entity2, *entity1)
                    } else {
                        continue;
                    };

                let (velocity, mut health, damage) = player.get_mut(player_entity).unwrap();
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
