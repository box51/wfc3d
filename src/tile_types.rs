use std::collections::HashMap;
use crate::wfc::{TileType, Direction};

pub fn create_tile_types() -> Vec<TileType> {
    let mut tiles = Vec::new();

    // Helper function to create connection maps
    let all_flat = || {
        let mut connections = HashMap::new();
        for dir in Direction::all() {
            connections.insert(dir, "flat".to_string());
        }
        connections
    };

    // 1. Basic Cube - connects on all sides with flat connections
    tiles.push(TileType::new(
        0,
        "Cube",
        "cube",
        all_flat(),
        1.0,
    ));

    // 2. Road/Straight piece - connects on two opposite sides
    let mut road_connections = HashMap::new();
    road_connections.insert(Direction::PosX, "flat".to_string());
    road_connections.insert(Direction::NegX, "flat".to_string());
    road_connections.insert(Direction::PosY, "none".to_string());
    road_connections.insert(Direction::NegY, "flat".to_string());
    road_connections.insert(Direction::PosZ, "road_end".to_string());
    road_connections.insert(Direction::NegZ, "road_end".to_string());
    tiles.push(TileType::new(
        1,
        "Road",
        "road",
        road_connections,
        1.2,
    ));

    // 3. Top piece - pyramid or cap, only connects on bottom
    let mut top_connections = HashMap::new();
    top_connections.insert(Direction::PosX, "road_end".to_string());
    top_connections.insert(Direction::NegX, "road_end".to_string());
    top_connections.insert(Direction::PosY, "none".to_string());
    top_connections.insert(Direction::NegY, "flat".to_string());
    top_connections.insert(Direction::PosZ, "road_end".to_string());
    top_connections.insert(Direction::NegZ, "road_end".to_string());
    tiles.push(TileType::new(
        2,
        "Top",
        "top",
        top_connections,
        0.3,
    ));

    // 4. Empty space - only the bottom connects to flat surfaces
    let mut empty_connections = HashMap::new();
    empty_connections.insert(Direction::PosX, "none".to_string());
    empty_connections.insert(Direction::NegX, "none".to_string());
    empty_connections.insert(Direction::PosY, "none".to_string());
    empty_connections.insert(Direction::NegY, "none".to_string());  // Can sit on flat surfaces
    empty_connections.insert(Direction::PosZ, "none".to_string());
    empty_connections.insert(Direction::NegZ, "none".to_string());
    tiles.push(TileType::new(
        3,
        "Empty",
        "empty",
        empty_connections,
        0.2, // Lower weight so structures are more solid
    ));

    tiles
}

// Helper function to select a random starting tile
pub fn select_random_start_tile(tiles: &[TileType]) -> TileType {
    use rand::prelude::*;
    let mut rng = thread_rng();

    // Prefer ground-connectable tiles for the start (but not empty tiles)
    let ground_tiles: Vec<&TileType> = tiles.iter()
        .filter(|t| t.visual_type != "empty" &&
                    t.connections.get(&Direction::NegY)
                        .map(|c| c != "none")
                        .unwrap_or(false))
        .collect();

    if !ground_tiles.is_empty() {
        ground_tiles[rng.gen_range(0..ground_tiles.len())].clone()
    } else {
        tiles.iter()
            .filter(|t| t.visual_type != "empty")
            .next()
            .unwrap_or(&tiles[0])
            .clone()
    }
}