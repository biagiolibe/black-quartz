mod game;
mod hud;
mod loading;
mod map;
mod menu;
mod player;
mod camera;
mod world_base;

mod prelude {
    pub use crate::camera::*;
    pub use crate::game::*;
    pub use crate::hud::*;
    pub use crate::loading::*;
    pub use crate::map::*;
    pub use crate::menu::*;
    pub use crate::player::*;
    pub use crate::world_base::*;
}

use crate::prelude::GamePlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;

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
        .run();
}
