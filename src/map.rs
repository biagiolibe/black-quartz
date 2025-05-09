use crate::map::RockType::Asteroid;
use crate::map::TileType::Solid;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub const TILE_SIZE: f32 = 32.0;
const GRID_WIDTH: isize = 200;
const GRID_HEIGHT: isize = 200;
pub struct MapPlugin;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Solid,
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RockType {
    Asteroid,
}

#[derive(Resource)]
pub struct WorldGrid {
    grid: Vec<Vec<Tile>>,
}
#[derive(Component, Clone, Copy, PartialEq)]
pub struct Tile {
    tile_type: TileType,
    rock_type: RockType,
    entity: Option<Entity>,
}

/// This plugin handles map related stuff
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGrid {
            grid: vec![
                vec![
                    Tile {
                        tile_type: TileType::Empty,
                        rock_type: RockType::Asteroid,
                        entity: None
                    };
                    GRID_WIDTH as usize
                ];
                GRID_HEIGHT as usize
            ],
        })
        .add_systems(Startup, generate_map);
    }
}

fn generate_map(mut commands: Commands) {
    for x in -GRID_WIDTH..GRID_WIDTH {
        for y in -GRID_HEIGHT..0 {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.28, 0.22, 0.20),
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                RigidBody::Fixed,
                Collider::cuboid(TILE_SIZE / 2f32, TILE_SIZE / 2f32),
                Tile {
                    tile_type: Solid,
                    rock_type: Asteroid,
                    entity: None,
                },
            ));
        }
    }
}
