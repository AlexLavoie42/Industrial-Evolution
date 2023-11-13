use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;

use crate::*;

#[derive(Component, Reflect)]
pub struct MoveToTile {
    pub target: Option<TilePos>,
    pub path: Option<Vec<TilePos>>,
    pub path_i: usize
}

pub fn move_towards_path(
    mut q_move: Query<(&MoveToTile, &mut Movement, &Transform)>,
    q_tilemap: Query<(&Transform, &TilemapGridSize, &TilemapType)>
) {
    let (map_transform, grid_size, map_type) = q_tilemap.single();
    for (move_to_tile, mut movement, transform) in q_move.iter_mut() {
        if let (Some(path), Some(target)) = (&move_to_tile.path, move_to_tile.target) {
            if move_to_tile.path_i >= path.len() {
                movement.input = None;
                continue;
            }
            let point: Vec2 = get_tile_world_pos(&path[move_to_tile.path_i], map_transform, grid_size, map_type);
            let direction = Vec2::new(point.x as f32, point.y as f32) - Vec2::new(transform.translation.x, transform.translation.y);
            movement.input = Some(direction.normalize());
        } else {
            movement.input = None;
        }
    }
}

pub fn iterate_path(
    mut q_move: Query<(&mut MoveToTile, &Transform)>,
    q_tilemap: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
) {
    let (map_size, grid_size, map_type, map_transform) = q_tilemap.single();
    for (mut move_to_tile, transform) in q_move.iter_mut() {
        let world_pos = get_world_pos(Vec2 { x: transform.translation.x, y: transform.translation.y }, map_transform);
        if let (Some(tile_pos), Some(target), Some(path)) = (TilePos::from_world_pos(
            &Vec2 { x: world_pos.x, y: world_pos.y }, 
            map_size, 
            grid_size, 
            map_type
        ), move_to_tile.target, &move_to_tile.path) {
            if Some(&tile_pos) == path.get(move_to_tile.path_i) && path.len() > move_to_tile.path_i + 1 {
                move_to_tile.path_i += 1;
            } else if tile_pos == target {
                move_to_tile.path = None;
                move_to_tile.path_i = 0;
            }
        }
    }
}

pub fn set_path_to_tile(
    mut q_move: Query<(&mut MoveToTile, &Transform)>,
    q_tilemap: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
    q_collision_tiles: Query<&TilePos, With<TileMapCollision>>,
) {    
    let (map_size, grid_size, map_type , map_transform) = q_tilemap.single();
    let collision_tiles = q_collision_tiles.iter().collect::<Vec<_>>();
    for (mut move_to_tile, transform) in q_move.iter_mut() {
        let world_pos = get_world_pos(Vec2 { x: transform.translation.x, y: transform.translation.y }, map_transform);
        if let (Some(tile_pos), Some(target)) = (TilePos::from_world_pos(
            &Vec2 { x: world_pos.x, y: world_pos.y }, 
            map_size, 
            grid_size, 
            map_type
        ), move_to_tile.target) {
            if tile_pos != target {
                if move_to_tile.path.is_none() {
                    move_to_tile.path = Some(Vec::new());
                }
                if let Some(move_path) = move_to_tile.path.as_mut() {
                    if !move_path.is_empty() { continue; };
                    let successors   = |pos: &TilePos| {
                        let neighbors = Neighbors::get_square_neighboring_positions(pos, map_size, true)
                            .iter()
                            .cloned()
                            .filter(|p| {
                                !collision_tiles.iter().any(|c| c.x == p.x && c.y == p.y)
                            })
                            .collect::<Vec<_>>();
                        neighbors.into_iter().map(|p| (p.clone(), 1)).collect::<Vec<_>>()
                    };
                    let distance = |pos: &TilePos| {
                        Vec2::new(pos.x as f32, pos.y as f32).distance(Vec2::new(target.x as f32, target.y as f32)) as u32
                    };
                    if let Some(path) = astar(
                        &tile_pos, 
                        successors,
                        distance,
                        |pos| {
                            pos == &target
                        }
                    ) {
                        move_to_tile.path = Some(path.0);
                    }
                } 
            }
        }
    }
}
