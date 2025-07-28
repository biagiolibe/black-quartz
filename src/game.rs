#![allow(clippy::type_complexity)]

use crate::prelude::*;
use bevy::app::App;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

#[derive(Resource)]
pub struct EconomyConfig {
    pub fuel_price_per_unit: u32,
    pub fuel_refill_amount: f32,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        EconomyConfig {
            fuel_price_per_unit: 2,
            fuel_refill_amount: 100.0,
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EconomyConfig::default())
            .init_state::<GameState>()
            .init_state::<MenuState>()
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                CameraPlugin,
                MapPlugin,
                WorldBasePlugin,
                HUDPlugin,
                PlayerPlugin,
            ));
        /*
               #[cfg(debug_assertions)]
               {
                   app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
               }
        */
    }
}
