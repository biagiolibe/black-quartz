use crate::game::GameState;
use crate::map::TILE_SIZE;
use crate::player::Player;
use crate::prelude::{GameAssets, Inventory, Menu};
use bevy::prelude::*;
use bevy_rapier2d::pipeline::CollisionEvent;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Sensor};

pub struct WorldBasePlugin;

#[derive(Component)]
pub struct WorldBase;

/// This plugin handles base-related stuff
impl Plugin for WorldBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_base)
            .add_systems(Update, base_access);
    }
}
fn spawn_base(mut commands: Commands, game_assets: Res<GameAssets>) {
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
                custom_size: Some(Vec2::splat(base_size)),
                ..default()
            },
            Transform::from_xyz(-base_size, (base_size / 2.0) - TILE_SIZE, -1.0),
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
}

fn base_access(
    mut collision_events: EventReader<CollisionEvent>,
    player: Query<&Inventory, With<Player>>,
    world_base: Query<&Transform, With<WorldBase>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(collider1, collider2, _) => {
                let player_entity =
                    if player.get(*collider1).is_ok() && world_base.get(*collider2).is_ok() {
                        (*collider1)
                    } else if player.get(*collider2).is_ok() && world_base.get(*collider1).is_ok() {
                        (*collider2)
                    } else {
                        continue;
                    };
                println!("Player has access to base");
                let inventory = player.get(player_entity).unwrap();
                next_state.set(GameState::Menu)
            }
            _ => {}
        }
    }
}
