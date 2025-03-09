use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d::default());

    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgba(0.3, 0.5, 0.3, 0.2),
            custom_size: Some(Vec2::new(500.0, 20.0)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(250.0, 10.0),
    ));

    // Drilling Machine (Player)
    commands.spawn((
        Sprite {
            color: Color::srgba(0.8, 0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(15.0, 15.0),
        GravityScale(1.0),
        Velocity::zero(),
    ));
}
