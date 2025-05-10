mod game;
mod loading;
mod menu;
mod player;
mod map;

mod prelude {
    pub use crate::game::*;
    pub use crate::loading::*;
    pub use crate::menu::*;
    pub use crate::player::*;
}

use crate::game::GamePlugin;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_rapier2d::prelude::*;
use crate::map::MapPlugin;

#[derive(Component)]
struct BlackQuartzCamera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Black Quartz".to_string(),
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
        .add_systems(Startup, (setup))
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    //TODO move to a plugin?
    // Camera
    commands.spawn((
                       Camera2d::default(),
                       OrthographicProjection{
                           near: -1000.0,
                           far: 1000.0,
                           viewport_origin: Vec2::new(0.5,0.5),
                           scaling_mode: Default::default(),
                           scale: 0.6,
                           area: Default::default(),
                       },
                       BlackQuartzCamera,
                   ));
}

fn setup_borders(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    //Top border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(window.resolution.width() / 2.0, 5.0),
        Transform::from_xyz(0.0, window.height() / 2.0, 0.0),
    ));

    //Bottom border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(window.resolution.width() / 2.0, 5.0),
        Transform::from_xyz(0.0, -(window.height() / 2.0), 0.0),
    ));

    //Left border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(5.0, window.resolution.height() / 2.0),
        Transform::from_xyz(-(window.width() / 2.0), 0.0, 0.0),
    ));

    //Right border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(5.0, window.resolution.height() / 2.0),
        Transform::from_xyz(window.width() / 2.0, 0.0, 0.0),
    ));
}
