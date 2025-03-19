use crate::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::dynamics::{GravityScale, RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Menu;

fn spawn_player(
    mut commands: Commands,
) {
    // Drilling Machine (Player)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 50.0, 0.0),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::cuboid(15.0, 15.0),
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

