mod game;
mod loading;
mod player;
mod menu;

mod prelude {
    pub use crate::game::*;
    pub use crate::loading::*;
    pub use crate::menu::*;
    pub use crate::player::*;
}


use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;
use crate::game::GamePlugin;

#[derive(Component)]
struct BlackQuartzCamera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Drilly McDrillface".to_string(),
                    mode: WindowMode::Windowed,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(
        (Camera2d::default(),
         BlackQuartzCamera,
        )
    );

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
}

