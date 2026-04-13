use crate::prelude::Item;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub const TILE_SIZE: f32 = 32.0;
pub const GRID_WIDTH: isize = 100;
pub const GRID_HEIGHT: isize = 500;
pub(super) const FILL_PROBABILITY: f32 = 0.55;
pub(super) const SIMULATION_STEPS: usize = 4;

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

impl TileType {
    pub fn to_item(&self) -> Option<Item> {
        use TileType::*;
        match self {
            Solid | Sand | Empty => None,
            Iron => Some(Item {
                id: "iron".to_string(),
                name: "Iron".to_string(),
                quantity: 1,
                value: 10,
            }),
            Copper => Some(Item {
                id: "copper".to_string(),
                name: "Copper".to_string(),
                quantity: 1,
                value: 5,
            }),
            Gold => Some(Item {
                id: "gold".to_string(),
                name: "Gold".to_string(),
                quantity: 1,
                value: 25,
            }),
            Crystal => Some(Item {
                id: "crystal".to_string(),
                name: "Crystal".to_string(),
                quantity: 1,
                value: 50,
            }),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
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
    pub revealed_tiles: HashSet<(i32, i32)>,
    pub tiles: Vec<Vec<TileType>>,
    pub map_area: Rect,
}

#[derive(Component, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct FovOverlay;

#[derive(Event)]
pub struct TileDestroyedEvent {
    pub tile_type: TileType,
    pub position: (i32, i32),
    pub entity: Entity,
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

pub fn world_grid_position_to_idx(world_grid_position: (i32, i32)) -> (usize, usize) {
    (
        (world_grid_position.0 + (GRID_WIDTH / 2) as i32) as usize,
        (world_grid_position.1 + GRID_HEIGHT as i32) as usize,
    )
}
