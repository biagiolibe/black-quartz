use crate::map::components::{TileType, WorldGrid, world_grid_position_to_idx, world_to_grid_position};
use crate::prelude::{FieldOfView, Player};
use bevy::prelude::*;
use crate::map::components::Tile;
use std::collections::VecDeque;

pub fn update_fov(
    mut player_query: Query<(&Transform, Mut<FieldOfView>), With<Player>>,
    world_grid: ResMut<WorldGrid>,
) {
    if let Ok((player_transform, mut fov)) = player_query.single_mut() {
        let player_pos = IVec2::from(world_to_grid_position(
            player_transform.translation.truncate(),
        ));

        let mut queue = VecDeque::new();
        queue.push_back((player_pos, 0));
        let mut visited = vec![player_pos];

        while let Some((pos, dist)) = queue.pop_front() {
            if dist > fov.radius {
                continue;
            }
            if visited.contains(&pos) && player_pos != pos {
                continue;
            }
            visited.push(pos);

            let (id_x, id_y) = world_grid_position_to_idx((pos.x, pos.y));

            if id_x >= world_grid.tiles[0].len() || id_y >= world_grid.tiles.len() {
                continue;
            }

            fov.visible_tiles.insert((pos.x, pos.y));
            fov.dirty = true;

            if world_grid.tiles[id_y][id_x] != TileType::Empty {
                continue;
            }

            let neighbors = [
                IVec2::new(pos.x + 1, pos.y + 1),
                IVec2::new(pos.x + 1, pos.y),
                IVec2::new(pos.x + 1, pos.y - 1),
                IVec2::new(pos.x, pos.y - 1),
                IVec2::new(pos.x - 1, pos.y - 1),
                IVec2::new(pos.x - 1, pos.y),
                IVec2::new(pos.x - 1, pos.y + 1),
                IVec2::new(pos.x, pos.y + 1),
            ];

            for n in neighbors {
                queue.push_back((n, dist + 1));
            }
        }
    }
}

pub fn update_fov_overlay(
    mut fov_query: Query<&mut FieldOfView, With<Player>>,
    mut query_tiles: Query<(&mut Sprite, &Tile), With<Tile>>,
    mut world_grid: ResMut<WorldGrid>,
) {
    if let Ok(mut fov) = fov_query.single_mut() {
        if fov.dirty {
            fov.visible_tiles.iter().for_each(|(x, y)| {
                if !world_grid.revealed_tiles.contains(&(*x, *y)) {
                    if let Some(entity) = world_grid.grid.get(&(*x, *y)) {
                        let (mut sprite, tile) = query_tiles.get_mut(*entity).unwrap();
                        info!("Foving {}x{}", x, y);
                        match tile.tile_type {
                            TileType::Empty => sprite.color = Color::NONE,
                            _ => sprite.color = Color::WHITE,
                        }
                    }
                    world_grid.revealed_tiles.insert((*x, *y));
                }
            });
            fov.dirty = false;
        }
    }
}
