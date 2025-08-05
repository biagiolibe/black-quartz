#![allow(clippy::type_complexity)]

use crate::prelude::*;
use bevy::app::App;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystems {
    Loading,
    Rendering,
    Running,
    Movement,
    Physics,
    Camera,
    Collision,
    Ui
}
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // Start window
    #[default]
    Loading,   // Caricamento iniziale
    MainMenu,  // Menu principale
    Rendering, // Rendering iniziale
    Menu,      // Menu in game
    Playing,   // Gioco attivo
    //Paused,  // Gioco in pausa
    GameOver,  // Fine partita
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
        app.configure_sets(
            OnEnter(GameState::Loading),
            (
                GameSystems::Loading,
                GameSystems::Rendering,
                GameSystems::Running,
            )
                .chain(),
        )
            .configure_sets(
                Update,
                (
                    GameSystems::Loading,
                    GameSystems::Rendering,
                    GameSystems::Movement,
                    GameSystems::Camera,
                    GameSystems::Physics,
                    GameSystems::Collision,
                    GameSystems::Ui
                )
                    .chain(),
            )
            .insert_resource(EconomyConfig::default())
            .init_state::<GameState>()
            .init_state::<MenuState>()
            .add_plugins(ResourcePlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(MapPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(WorldBasePlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(HUDPlugin)
            .add_plugins(DrillAnimationPlugin)
            .add_systems(OnEnter(GameState::GameOver), exit_game);
        /*
               #[cfg(debug_assertions)]
               {
                   app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
               }
        */
    }
}

fn exit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}
