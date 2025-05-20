use crate::map::{Tile, WorldGrid, TILE_SIZE};
use crate::prelude::*;
use crate::BlackQuartzCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub const PLAYER_DRILLING_STRENGHT: f32 = 0.2;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (drill, move_player.run_if(in_state(GameState::Playing))).chain(),
            );
    }
}
fn spawn_player(mut commands: Commands,
game_assets: Res<GameAssets>,) {
    // Drilling Machine (Player)
    commands
        .spawn((
            Player,
            Sprite {
                image: game_assets.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_assets.texture_layout.clone(),
                    index: 1,
                }),
                //color: Color::srgb(0.90, 0.75, 0.25), //INDUSTRIAL YELLOW
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, 50.0, 0.0),
            RigidBody::Dynamic,
            Collider::capsule_y((TILE_SIZE - 28.0) / 2f32, 14.0),
            GravityScale(7.0),
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
        let movement = keyboard_input
            .get_pressed()
            .fold(Vec2::ZERO, |mut acceleration, key| {
                match key {
                    KeyCode::ArrowLeft => acceleration.x -= 1.0,
                    KeyCode::ArrowRight => acceleration.x += 1.0,
                    KeyCode::ArrowUp => acceleration.y += 1.0,
                    KeyCode::ArrowDown => acceleration.y -= 1.0,
                    _ => (),
                }
                acceleration
            });
        velocity.linvel = movement * 200.0;

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
        let current_position = (
            (position.x / TILE_SIZE) as i32,
            (position.y / TILE_SIZE) as i32,
        );

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
                    tile.drilling.integrity -= PLAYER_DRILLING_STRENGHT * time.delta_secs() * (1.0 - tile.drilling.hardness);
                    println!("tile integrity {:?}", tile.drilling.integrity);
                    if tile.drilling.integrity <= 0.0 {
                        commands.entity(*entity).despawn();
                        world_grid.grid.remove(&target_index);
                        println!("Drilled tile at {:?} with player on position {:?}", target_index, current_position);
                    }
                }
                else {
                    println!("No tile exists to be drilled on position {:?}", target_index);
                };

            }
        }
    }
}

fn is_in_target(tile_position: Vec3, target: Vec3) -> bool {
    (tile_position.x - target.x).abs() < 4.0
        && (tile_position.y - target.y).abs() < 4.0
        && (tile_position.z - target.z).abs() < 4.0
}
