use crate::loading::GameAssets;
use crate::map::TileType::Solid;
use crate::prelude::TileType::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::collections::HashMap;

pub const TILE_SIZE: f32 = 32.0;
pub const GRID_WIDTH: isize = 100;
pub const GRID_HEIGHT: isize = 500;
const FILL_PROBABILITY: f32 = 0.55;
const SIMULATION_STEPS: usize = 4;
pub struct MapPlugin;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Solid,
    Sand,
    Iron,
    Copper,
    Gold,
    Crystal,
    Empty,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Tile {
    tile_type: TileType,
    pub drilling: Drilling,
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
pub struct FovOverlay;

pub struct Map;

/// This plugin handles map related stuff
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                initialize_world_grid,
                (render_map, setup_borders).after(initialize_world_grid),
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

    //Distribute materials
    tiles = distribute_materials(&mut tiles);

    /* Debug: create a temp vertically tunnel through the entire map
    for y in 1..GRID_HEIGHT as usize {
        tiles[y][(GRID_WIDTH / 2) as usize] = Empty;
    }
     */

    commands.insert_resource(WorldGrid {
        grid: HashMap::new(),
        tiles,
        map_area: Rect::new(
            //subtract half of TILE_SIZE in order to align with the last tile in grid
            -(GRID_WIDTH as f32 / 2.0) * TILE_SIZE - TILE_SIZE / 2.0,
            -(GRID_HEIGHT as f32) * TILE_SIZE - TILE_SIZE / 2.0,
            (GRID_WIDTH as f32 / 2.0) * TILE_SIZE - TILE_SIZE / 2.0,
            (GRID_HEIGHT as f32) * TILE_SIZE - TILE_SIZE / 2.0,
        ),
    });
}

fn distribute_materials(tiles: &mut Vec<Vec<TileType>>) -> Vec<Vec<TileType>> {
    let perlin = Perlin::new(0);
    let mut materialized_tiles = tiles.clone();

    for y in 0..GRID_HEIGHT as usize {
        for x in 0..GRID_WIDTH as usize {
            if tiles[y][x] == Empty {
                continue;
            }
            // Normalizza le coordinate per ottenere un pattern ampio
            let scale = 0.45;
            let noise_value = perlin.get([x as f64 * scale, y as f64 * scale]);

            // Convertilo in un valore 0.0 - 1.0
            let noise_val = ((noise_value + 1.0) / 2.0) as f32;

            // La profondità influenza la rarità
            let depth = y as f32;
            // Rarità controllata da profondità e noise
            let mut material = Solid;
            if depth > (GRID_HEIGHT - ((GRID_HEIGHT * 20) / 100)) as f32 {
                // first 20% (as reversed for generation indexes)
                if noise_val < 0.7 {
                    material = Solid;
                } else if noise_val < 0.8 {
                    material = Sand;
                } else if noise_val < 0.9 {
                    material = Copper;
                } else {
                    material = Iron;
                }
            } else if depth > (GRID_HEIGHT - ((GRID_HEIGHT * 80) / 100)) as f32 {
                if noise_val < 0.7 {
                    material = Solid;
                } else if noise_val < 0.9 {
                    material = Iron;
                } else {
                    material = Gold;
                }
            } else {
                if noise_val < 0.7 {
                    material = Solid
                } else if noise_val < 0.8 {
                    material = Iron
                } else if noise_val < 0.9 {
                    material = Gold
                } else {
                    material = Crystal
                }
            }
            materialized_tiles[y][x] = material;
        }
    }
    materialized_tiles
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

            let entity = match tile_type {
                Empty => commands.spawn((
                    Sprite {
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        color: match y {
                            depth if depth < -1 => Color::srgba(0.0, 0.0, 0.0, 1.0),
                            _ => Color::NONE,
                        },
                        ..default()
                    },
                    Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                )),
                _ => {
                    let (tile, texture_layout_index) = get_tile_to_render(tile_type);
                    commands.spawn((
                        Sprite {
                            image: game_assets.terrain_texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: game_assets.terrain_texture_layout.clone(),
                                index: texture_layout_index,
                            }),
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            color: match y {
                                depth if depth < -1 => Color::srgba(0.0, 0.0, 0.0, 1.0),
                                _ => Color::WHITE,
                            },
                            ..default()
                        },
                        Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                        RigidBody::Fixed,
                        Collider::cuboid(TILE_SIZE / 2f32, TILE_SIZE / 2f32),
                        ActiveEvents::COLLISION_EVENTS,
                        tile,
                    ))
                }
            }
                .id();
            world_grid.grid.insert((x as i32, y as i32), entity);
        }
    }
}

fn get_tile_to_render(tile_type: &TileType) -> (Tile, usize) {
    match tile_type {
        Solid => (
            Tile {
                tile_type: Solid,
                drilling: Drilling {
                    integrity: 0.4,
                    hardness: 0.1,
                },
            },
            0,
        ),
        Sand => (
            Tile {
                tile_type: Sand,
                drilling: Drilling {
                    integrity: 0.1,
                    hardness: 0.05,
                },
            },
            3,
        ),
        Copper => (
            Tile {
                tile_type: Solid,
                drilling: Drilling {
                    integrity: 0.4,
                    hardness: 0.2,
                },
            },
            5,
        ),
        Iron => (
            Tile {
                tile_type: Iron,
                drilling: Drilling {
                    integrity: 0.6,
                    hardness: 0.3,
                },
            },
            4,
        ),
        Gold => (
            Tile {
                tile_type: Solid,
                drilling: Drilling {
                    integrity: 0.4,
                    hardness: 0.2,
                },
            },
            6,
        ),
        Crystal => (
            Tile {
                tile_type: Solid,
                drilling: Drilling {
                    integrity: 0.1,
                    hardness: 0.07,
                },
            },
            7,
        ),
        _ => (
            Tile {
                tile_type: Empty,
                drilling: Drilling {
                    integrity: 0.0,
                    hardness: 0.0,
                },
            },
            0,
        ),
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

pub fn world_position_to_idx(world_position: Vec2) -> (usize, usize) {
    let world_grid_position = world_to_grid_position(world_position);
    (
        (world_grid_position.0 + (GRID_WIDTH / 2) as i32) as usize,
        (world_grid_position.1 + GRID_HEIGHT as i32) as usize
    )
}

fn setup_borders(mut commands: Commands, world_grid: Res<WorldGrid>) {
    //Top border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(world_grid.map_area.max.x, 1.0),
        Transform::from_xyz(0.0, world_grid.map_area.max.y, 0.0),
    ));

    //Bottom border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(world_grid.map_area.max.x, 1.0),
        Transform::from_xyz(0.0, world_grid.map_area.min.y, 0.0),
    ));

    //Left border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1.0, world_grid.map_area.max.y / 2.0),
        Transform::from_xyz(world_grid.map_area.min.x, 0.0, 0.0),
    ));

    //Right border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1.0, world_grid.map_area.max.y / 2.0),
        Transform::from_xyz(world_grid.map_area.max.x, 0.0, 0.0),
    ));

    //center border
    commands.spawn((RigidBody::Fixed, Transform::from_xyz(0.0, 0.0, 0.0)));
}
