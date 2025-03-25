use bevy::math::vec3;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::{GravityScale, RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use crate::Tile;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)).after(drill));
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
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 50.0, 0.0),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::cuboid(32.0, 32.0),
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
                KeyCode::ArrowLeft => Vec3::new(position.x - 1.0, position.y, position.z),
                KeyCode::ArrowRight => Vec3::new(position.x + 1.0, position.y, position.z),
                KeyCode::ArrowDown => Vec3::new(position.x, position.y - 1.0, position.z),
                _ => position,
            }
        });
        terrain_tiles.iter().filter(|(tile_position, _)| tile_position.translation.x == to_drill.x
            && tile_position.translation.y == to_drill.y
            && tile_position.translation.z == to_drill.z)
            .for_each(|(_, entity)| {
                commands.entity(entity).despawn();
            });
    }
}

