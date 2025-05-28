use crate::loading::GameAssets;
use crate::map::RockType::Asteroid;
use crate::map::TileType::Solid;
use crate::prelude::TileType::Empty;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody};
use rand::{Rng, random};
use std::collections::HashMap;

pub const TILE_SIZE: f32 = 32.0;
pub const GRID_WIDTH: isize = 50;
pub const GRID_HEIGHT: isize = 100;
const FILL_PROBABILITY: f32 = 0.55;
const SIMULATION_STEPS: usize = 4;
pub struct MapPlugin;

#[derive(Clone, Copy, PartialEq, Debug)]
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
    pub integrity: f32,
    pub hardness: f32,
}

#[derive(Resource)]
pub struct WorldGrid {
    pub grid: HashMap<(i32, i32), Entity>,
    pub tiles: Vec<Vec<TileType>>,
    pub map_area: Rect,
}
#[derive(Component, Clone, Copy, PartialEq)]
pub struct Tile {
    tile_type: TileType,
    rock_type: RockType,
    pub drilling: Drilling,
}

pub struct Map;

/// This plugin handles map related stuff
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                initialize_world_grid,
                render_map.after(initialize_world_grid),
            )
                .chain(),
        );
    }
}

fn initialize_world_grid(mut commands: Commands) {
    println!("Generating map using Cellular Automata algorithm");
    // Initialize an empty map
    let mut tiles = vec![vec![Empty; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
    let mut rng = rand::thread_rng();

    // 1st iteration: fill map with solid tiles
    for y in 0..GRID_HEIGHT as usize {
        for x in 0..GRID_WIDTH as usize {
            if rng.r#gen::<f32>() < FILL_PROBABILITY {
                tiles[y][x] = Solid;
            }
        }
    }

    // Next iterations: make simulations
    for s in 0..SIMULATION_STEPS {
        tiles = simulation(&mut tiles, s);
    }

    commands.insert_resource(WorldGrid {
        grid: HashMap::new(),
        tiles,
        map_area: Rect::new(
            -(GRID_WIDTH as f32 / 2.0) * TILE_SIZE,
            -(GRID_HEIGHT as f32) * TILE_SIZE,
            (GRID_WIDTH as f32 / 2.0) * TILE_SIZE,
            (GRID_HEIGHT as f32) * TILE_SIZE,
        ),
    });
}

fn simulation(tiles: &Vec<Vec<TileType>>, index: usize) -> Vec<Vec<TileType>> {
    let mut iterated_tiles = tiles.clone();

    for y in 0..GRID_HEIGHT as usize {
        for x in 0..GRID_WIDTH as usize {
            let solid_neighbors = count_solid_neighbors(&tiles, x, y);
            iterated_tiles[y][x] = match (tiles[y][x], solid_neighbors) {
                (Solid, n) if n < 3 => Empty,
                (Empty, n) if n > 4 => Solid,
                (current, _) => current,
            };
        }
    }
    iterated_tiles[0][index] = Solid;
    iterated_tiles
}

fn count_solid_neighbors(tiles: &Vec<Vec<TileType>>, x: usize, y: usize) -> usize {
    let mut solid_neighbors = 0;
    for i in -1isize..=1 {
        for j in -1isize..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let adjx = x as isize + j;
            let adjy = y as isize + i;
            if (adjx >= GRID_WIDTH || adjy >= GRID_HEIGHT)
                || (adjx < 0 || adjy < 0)
                || tiles[adjy as usize][adjx as usize] == Solid
            {
                solid_neighbors += 1;
            }
        }
    }
    solid_neighbors
}

fn render_map(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut world_grid: ResMut<WorldGrid>,
) {
    for x in -GRID_WIDTH / 2..GRID_WIDTH / 2 {
        for y in -GRID_HEIGHT..0 {
            let tile_type =
                &world_grid.tiles[(y + GRID_HEIGHT) as usize][(x + (GRID_WIDTH / 2)) as usize];
            match tile_type {
                Solid => {
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
                            ActiveEvents::COLLISION_EVENTS,
                            Tile {
                                tile_type: Solid,
                                rock_type: Asteroid,
                                drilling: Drilling {
                                    integrity: 0.3,
                                    hardness: 0.1,
                                },
                            },
                        ))
                        .id();
                    world_grid.grid.insert((x as i32, y as i32), entity);
                }
                _ => {}
            };
        }
    }
}

pub fn world_to_grid_position(world_position: Vec2) -> (i32, i32) {
    (
        ((world_position.x + (world_position.x / world_position.x.abs()) * TILE_SIZE / 2.0)
            / TILE_SIZE)
            .trunc() as i32,
        ((world_position.y + (world_position.y / world_position.y.abs()) * TILE_SIZE / 2.0)
            / TILE_SIZE)
            .trunc() as i32,
    )
}
