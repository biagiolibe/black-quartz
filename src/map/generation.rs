use crate::map::components::{
    Drilling, FILL_PROBABILITY, GRID_HEIGHT, GRID_WIDTH, SIMULATION_STEPS, TILE_SIZE,
    Tile, TileDestroyedEvent, TileType, WorldGrid, world_grid_position_to_idx,
};
use crate::prelude::{GameAssets, LoadingProgress};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

use TileType::*;

pub fn initialize_world_grid(mut commands: Commands) {
    info!("Generating map using Cellular Automata algorithm");
    let mut tiles = vec![vec![Empty; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
    let mut rng = rand::thread_rng();

    for y in 0..GRID_HEIGHT as usize {
        for x in 0..GRID_WIDTH as usize {
            if rng.r#gen::<f32>() < FILL_PROBABILITY {
                tiles[y][x] = Solid;
            }
        }
    }

    for s in 0..SIMULATION_STEPS {
        tiles = simulation(&tiles, s);
    }

    tiles = distribute_materials(&mut tiles);

    commands.insert_resource(WorldGrid {
        grid: HashMap::new(),
        revealed_tiles: HashSet::new(),
        tiles,
        map_area: Rect::new(
            -(GRID_WIDTH as f32 / 2.0) * TILE_SIZE - TILE_SIZE / 2.0,
            -(GRID_HEIGHT as f32) * TILE_SIZE - TILE_SIZE / 2.0,
            (GRID_WIDTH as f32 / 2.0) * TILE_SIZE - TILE_SIZE / 2.0,
            (GRID_HEIGHT as f32) * TILE_SIZE - TILE_SIZE / 2.0,
        ),
    });
    info!("Map generated");
}

fn distribute_materials(tiles: &mut Vec<Vec<TileType>>) -> Vec<Vec<TileType>> {
    let perlin = Perlin::new(rand::thread_rng().gen());
    let mut materialized_tiles = tiles.clone();

    for y in 0..GRID_HEIGHT as usize {
        for x in 0..GRID_WIDTH as usize {
            if tiles[y][x] == Empty {
                continue;
            }
            let scale = 0.45;
            let noise_value = perlin.get([x as f64 * scale, y as f64 * scale]);
            let noise_val = ((noise_value + 1.0) / 2.0) as f32;
            let depth = y as f32;

            let material =
                if depth > (GRID_HEIGHT - ((GRID_HEIGHT * 20) / 100)) as f32 {
                    if noise_val < 0.7 {
                        Solid
                    } else if noise_val < 0.8 {
                        Sand
                    } else if noise_val < 0.9 {
                        Copper
                    } else {
                        Iron
                    }
                } else if depth > (GRID_HEIGHT - ((GRID_HEIGHT * 80) / 100)) as f32 {
                    if noise_val < 0.7 {
                        Solid
                    } else if noise_val < 0.9 {
                        Iron
                    } else {
                        Gold
                    }
                } else {
                    if noise_val < 0.7 {
                        Solid
                    } else if noise_val < 0.8 {
                        Iron
                    } else if noise_val < 0.9 {
                        Gold
                    } else {
                        Crystal
                    }
                };
            materialized_tiles[y][x] = material;
        }
    }
    materialized_tiles
}

fn simulation(tiles: &Vec<Vec<TileType>>, index: usize) -> Vec<Vec<TileType>> {
    let mut iterated_tiles = tiles.clone();

    for y in 0..GRID_HEIGHT as usize {
        for x in 0..GRID_WIDTH as usize {
            let solid_neighbors = count_solid_neighbors(tiles, x, y);
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

pub fn render_map(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    tile_query: Query<Entity, With<Tile>>,
    mut world_grid: ResMut<WorldGrid>,
) {
    for tile_entity in tile_query.iter() {
        commands.entity(tile_entity).despawn();
    }

    for x in -GRID_WIDTH / 2..GRID_WIDTH / 2 {
        for y in -GRID_HEIGHT..0 {
            let tile_type =
                &world_grid.tiles[(y + GRID_HEIGHT) as usize][(x + (GRID_WIDTH / 2)) as usize];
            let (tile, texture_layout_index) = get_tile_to_render(tile_type);
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
                    tile,
                )),
                _ => commands.spawn((
                    Sprite {
                        image: game_assets.terrain.texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: game_assets.terrain.texture_layout.clone(),
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
                )),
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
                tile_type: Copper,
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
                tile_type: Gold,
                drilling: Drilling {
                    integrity: 0.4,
                    hardness: 0.2,
                },
            },
            6,
        ),
        Crystal => (
            Tile {
                tile_type: Crystal,
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

pub fn handle_tile_destroyed(
    mut commands: Commands,
    mut events: EventReader<TileDestroyedEvent>,
    mut world_grid: ResMut<WorldGrid>,
) {
    for event in events.read() {
        commands.entity(event.entity).despawn();
        world_grid.grid.remove(&event.position);
        let grid_id = world_grid_position_to_idx(event.position);
        world_grid.tiles[grid_id.1][grid_id.0] = TileType::Empty;
    }
}

pub fn setup_borders(
    mut commands: Commands,
    world_grid: Res<WorldGrid>,
    mut loading_progress: ResMut<LoadingProgress>,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(world_grid.map_area.max.x, 1.0),
        Transform::from_xyz(0.0, world_grid.map_area.max.y, 0.0),
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(world_grid.map_area.max.x, 1.0),
        Transform::from_xyz(0.0, world_grid.map_area.min.y, 0.0),
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1.0, world_grid.map_area.max.y / 2.0),
        Transform::from_xyz(world_grid.map_area.min.x, 0.0, 0.0),
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1.0, world_grid.map_area.max.y / 2.0),
        Transform::from_xyz(world_grid.map_area.max.x, 0.0, 0.0),
    ));
    commands.spawn((RigidBody::Fixed, Transform::from_xyz(0.0, 0.0, 0.0)));
    loading_progress.rendering_map = true;
}
