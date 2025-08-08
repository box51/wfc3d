// tile_types.rs
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
    road_connections.insert(Direction::PosX, "male".to_string());
    road_connections.insert(Direction::NegX, "female".to_string());
    road_connections.insert(Direction::PosY, "flat".to_string());
    road_connections.insert(Direction::NegY, "ground".to_string());
    road_connections.insert(Direction::PosZ, "road_side".to_string());
    road_connections.insert(Direction::NegZ, "road_side".to_string());

    tiles.push(TileType::new(
        1,
        "Road",
        "road",
        road_connections,
        1.2,
    ));

    // 3. Corner piece - L-shaped connector
    let mut corner_connections = HashMap::new();
    corner_connections.insert(Direction::PosX, "male".to_string());
    corner_connections.insert(Direction::NegX, "flat".to_string());
    corner_connections.insert(Direction::PosY, "flat".to_string());
    corner_connections.insert(Direction::NegY, "ground".to_string());
    corner_connections.insert(Direction::PosZ, "female".to_string());
    corner_connections.insert(Direction::NegZ, "flat".to_string());

    tiles.push(TileType::new(
        2,
        "Corner",
        "corner",
        corner_connections,
        0.8,
    ));

    // 4. Junction - connects on 4 horizontal sides
    let mut junction_connections = HashMap::new();
    junction_connections.insert(Direction::PosX, "male".to_string());
    junction_connections.insert(Direction::NegX, "female".to_string());
    junction_connections.insert(Direction::PosY, "flat".to_string());
    junction_connections.insert(Direction::NegY, "ground".to_string());
    junction_connections.insert(Direction::PosZ, "male".to_string());
    junction_connections.insert(Direction::NegZ, "female".to_string());

    tiles.push(TileType::new(
        3,
        "Junction",
        "junction",
        junction_connections,
        0.6,
    ));

    // 5. Top piece - pyramid or cap, only connects on bottom
    let mut top_connections = HashMap::new();
    top_connections.insert(Direction::PosX, "none".to_string());
    top_connections.insert(Direction::NegX, "none".to_string());
    top_connections.insert(Direction::PosY, "none".to_string());
    top_connections.insert(Direction::NegY, "flat".to_string());
    top_connections.insert(Direction::PosZ, "none".to_string());
    top_connections.insert(Direction::NegZ, "none".to_string());

    tiles.push(TileType::new(
        4,
        "Top",
        "top",
        top_connections,
        0.3,
    ));

    // 6. Empty space - no connections
    let mut empty_connections = HashMap::new();
    for dir in Direction::all() {
        empty_connections.insert(dir, "none".to_string());
    }

    tiles.push(TileType::new(
        5,
        "Empty",
        "empty",
        empty_connections,
        0.2, // Lower weight so structures are more solid
    ));

    // 7. Vertical pillar
    let mut pillar_connections = HashMap::new();
    pillar_connections.insert(Direction::PosX, "flat".to_string());
    pillar_connections.insert(Direction::NegX, "flat".to_string());
    pillar_connections.insert(Direction::PosY, "male".to_string());
    pillar_connections.insert(Direction::NegY, "female".to_string());
    pillar_connections.insert(Direction::PosZ, "flat".to_string());
    pillar_connections.insert(Direction::NegZ, "flat".to_string());

    tiles.push(TileType::new(
        6,
        "Pillar",
        "pillar",
        pillar_connections,
        0.7,
    ));

    tiles
}

// Optional: Create specific tile sets for different structure types
pub fn create_building_tiles() -> Vec<TileType> {
    let mut tiles = create_tile_types();

    // Add building-specific tiles
    // Floor tile
    let mut floor_connections = HashMap::new();
    floor_connections.insert(Direction::PosX, "flat".to_string());
    floor_connections.insert(Direction::NegX, "flat".to_string());
    floor_connections.insert(Direction::PosY, "floor_top".to_string());
    floor_connections.insert(Direction::NegY, "none".to_string());
    floor_connections.insert(Direction::PosZ, "flat".to_string());
    floor_connections.insert(Direction::NegZ, "flat".to_string());

    tiles.push(TileType::new(
        tiles.len(),
        "Floor",
        "floor",
        floor_connections,
        1.5,
    ));

    // Wall tile
    let mut wall_connections = HashMap::new();
    wall_connections.insert(Direction::PosX, "wall".to_string());
    wall_connections.insert(Direction::NegX, "flat".to_string());
    wall_connections.insert(Direction::PosY, "flat".to_string());
    wall_connections.insert(Direction::NegY, "floor_top".to_string());
    wall_connections.insert(Direction::PosZ, "wall".to_string());
    wall_connections.insert(Direction::NegZ, "wall".to_string());

    tiles.push(TileType::new(
        tiles.len(),
        "Wall",
        "wall",
        wall_connections,
        1.0,
    ));

    tiles
}

// Helper function to select a random starting tile
pub fn select_random_start_tile(tiles: &[TileType]) -> TileType {
    use rand::prelude::*;
    let mut rng = thread_rng();

    // Prefer ground-connectable tiles for the start
    let ground_tiles: Vec<&TileType> = tiles.iter()
        .filter(|t| t.connections.get(&Direction::NegY)
            .map(|c| c != "none")
            .unwrap_or(false))
        .collect();

    if !ground_tiles.is_empty() {
        ground_tiles[rng.gen_range(0..ground_tiles.len())].clone()
    } else {
        tiles[rng.gen_range(0..tiles.len())].clone()
    }
}