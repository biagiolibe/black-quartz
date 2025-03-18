use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.5, 0.3),
            custom_size: Some(Vec2::new(1000.0, 600.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -100.0, 0.0),
        GlobalTransform::default(),
        RigidBody::Fixed,
        Collider::cuboid(500.0, 300.0),
    ));

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

fn player_movement(
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

