pub mod components;
pub mod fov;
pub mod generation;

pub use components::*;
pub use fov::*;
pub use generation::*;

use crate::prelude::GameState::Playing;
use crate::prelude::GameSystems::{Rendering, Running};
use crate::prelude::GameState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileDestroyedEvent>()
            .add_systems(
                OnEnter(GameState::Rendering),
                (initialize_world_grid, render_map, setup_borders)
                    .in_set(Rendering)
                    .chain(),
            )
            .add_systems(
                Update,
                (update_fov, update_fov_overlay)
                    .in_set(Running)
                    .run_if(in_state(Playing)),
            );
    }
}
