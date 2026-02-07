use std::ops::Mul;
use crate::game::GameState;
use crate::map::TILE_SIZE;
use crate::player::Player;
use crate::prelude::GameState::Playing;
use crate::prelude::GameSystems::Ui;
use crate::prelude::{GameAssets, Inventory, LoadingProgress, MenuState};
use bevy::prelude::*;
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Sensor};

pub struct WorldBasePlugin;

#[derive(Component)]
pub struct WorldBase;

/// This plugin handles base-related stuff
impl Plugin for WorldBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Rendering), spawn_base)
            .add_systems(Update, base_access.in_set(Ui).run_if(in_state(Playing)));
    }
}
fn spawn_base(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut loading_progress: ResMut<LoadingProgress>,
) {
    // World Base
    let base_size = TILE_SIZE * 6.0;
    commands
        .spawn((
            Sprite {
                image: game_assets.buildings.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_assets.buildings.texture_layout.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(base_size,base_size-TILE_SIZE.mul(2.0))),
                ..default()
            },
            Transform::from_xyz(-base_size, (base_size/ 3.0) - TILE_SIZE+7.0, -1.0),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldBase,
                Transform::from_xyz(0.0, -((base_size / 2.0) - TILE_SIZE), -1.0),
                Collider::cuboid(TILE_SIZE / 2f32, TILE_SIZE / 2f32),
                Sensor,
            ));
        });
    loading_progress.spawning_base = true;
}

fn base_access(
    mut collision_events: EventReader<CollisionEvent>,
    player: Query<&Inventory, With<Player>>,
    world_base: Query<&Transform, With<WorldBase>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(collider1, collider2, _) => {
                if (player.get(*collider1).is_ok() && world_base.get(*collider2).is_ok())
                    || (player.get(*collider2).is_ok() && world_base.get(*collider1).is_ok())
                {
                    info!("Player has accessed the base");
                    next_game_state.set(GameState::Menu);
                    next_menu_state.set(MenuState::WorldBase);
                };
            }
            _ => {}
        }
    }
}
