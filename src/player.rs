use crate::prelude::*;
use crate::Tile;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (drill, move_player.run_if(in_state(GameState::Playing)).after(drill)));
    }
}

#[derive(Component)]
pub struct Menu;

fn spawn_player(
    mut commands: Commands,
) {
    // Drilling Machine (Player)
    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 50.0, 0.0),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::cuboid(TILE_SIZE / 2f32, TILE_SIZE / 2f32),
        GravityScale(6.0),
        Velocity::zero(),
    ));
}
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<RigidBody>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        let movement = keyboard_input.get_pressed().fold(Vec2::ZERO, |mut acceleration, key| {
            match key {
                KeyCode::ArrowLeft => acceleration.x -= 1.0,
                KeyCode::ArrowRight => acceleration.x += 1.0,
                KeyCode::ArrowUp => acceleration.y += 1.0,
                KeyCode::ArrowDown => acceleration.y -= 1.0,
                _ => (),
            }
            acceleration
        });
        velocity.linvel = movement * 100.0;
    }
}

fn drill(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    terrain_tiles: Query<(&Transform, Entity), With<Tile>>,
) {
    if let Ok(mut transform) = player.get_single() {
        let to_drill = keyboard_input.get_pressed().fold(transform.translation, |position, key| {
            match key {
                KeyCode::ArrowLeft => Vec3::new(position.x - TILE_SIZE, position.y, position.z),
                KeyCode::ArrowRight => Vec3::new(position.x + TILE_SIZE, position.y, position.z),
                KeyCode::ArrowDown => Vec3::new(position.x, position.y - TILE_SIZE, position.z),
                _ => position,
            }
        });

        terrain_tiles.iter()
            .filter(|(tile_position, _)| is_in_target(tile_position.translation, to_drill))
            .for_each(|(tile_pos, entity)| {
                commands.entity(entity).despawn();
            });
    }
}

fn is_in_target(tile_position: Vec3, target: Vec3) -> bool {
    (tile_position.x - target.x).abs() < 1.0 && (tile_position.y - target.y).abs() < 1.0
        && (tile_position.z - target.z).abs() < 1.0
}

