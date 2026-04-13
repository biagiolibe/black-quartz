pub mod components;
pub mod drilling;
pub mod movement;

pub use components::*;
pub use drilling::*;
pub use movement::*;

use crate::map::handle_tile_destroyed;
use crate::prelude::GameSystems::Rendering;
use crate::prelude::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerImpactEvent>()
            .add_systems(
                OnEnter(GameState::Rendering),
                spawn_player.in_set(Rendering),
            )
            .add_systems(
                FixedUpdate,
                (move_player, drill, handle_tile_destroyed, handle_loot_pickup, falling_detection)
                    .run_if(in_state(GameState::Playing))
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    update_player_on_state_changes,
                    update_player_direction,
                    collision_detection,
                    apply_impact_damage,
                    death_detection,
                )
                    .run_if(in_state(GameState::Playing))
                    .chain(),
            );
    }
}
