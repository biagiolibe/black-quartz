mod game;
mod hud;
mod loading;
mod map;
mod menu;
mod player;
mod camera;

mod prelude {
    pub use crate::game::*;
    pub use crate::hud::*;
    pub use crate::loading::*;
    pub use crate::map::*;
    pub use crate::menu::*;
    pub use crate::player::*;
    pub use crate::camera::*;
}

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_rapier2d::prelude::*;
use crate::prelude::GamePlugin;

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
            //RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(GamePlugin)
        .run();
}
