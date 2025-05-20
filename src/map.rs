use crate::loading::GameAssets;
use crate::map::RockType::Asteroid;
use crate::map::TileType::Solid;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};
use std::collections::HashMap;

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

#[derive(Clone, Copy, PartialEq)]
pub struct Drilling {
    pub integrity:f32,
    pub hardness:f32,
}

#[derive(Resource)]
pub struct WorldGrid {
    pub grid: HashMap<(i32, i32), Entity>,
}
#[derive(Component, Clone, Copy, PartialEq)]
pub struct Tile {
    tile_type: TileType,
    rock_type: RockType,
    pub drilling: Drilling
}

pub struct Map;

/// This plugin handles map related stuff
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_map);
    }
}

fn generate_map(mut commands: Commands, game_assets: Res<GameAssets>) {
    println!("Generating map");
    let mut world_grid: HashMap<(i32, i32), Entity> = HashMap::new();
    for x in -GRID_WIDTH..GRID_WIDTH {
        for y in -GRID_HEIGHT..0 {
            let entity = commands
                .spawn((
                    Sprite {
                        image: game_assets.terrain_texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: game_assets.terrain_texture_layout.clone(),
                            index: 0,
                        }),
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                    RigidBody::Fixed,
                    Collider::cuboid(TILE_SIZE / 2f32, TILE_SIZE / 2f32),
                    Tile {
                        tile_type: Solid,
                        rock_type: Asteroid,
                        drilling:Drilling{
                            integrity:0.3,
                            hardness:0.1,
                        }
                    },
                ))
                .id();
            world_grid.insert((x as i32, y as i32), entity);
        }
    }
    commands.insert_resource(WorldGrid {
        grid: world_grid.clone(),
    });
}
