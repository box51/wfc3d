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
        0.8,
    ));

    // 3. Cross piece - pyramid or cap, only connects on bottom
    let mut cross_connections = HashMap::new();
    cross_connections.insert(Direction::PosX, "none".to_string());
    cross_connections.insert(Direction::NegX, "none".to_string());
    cross_connections.insert(Direction::PosY, "none".to_string());
    cross_connections.insert(Direction::NegY, "flat".to_string());
    cross_connections.insert(Direction::PosZ, "none".to_string());
    cross_connections.insert(Direction::NegZ, "none".to_string());
    tiles.push(TileType::new(
        2,
        "Cross",
        "top",
        cross_connections,
        0.3,
    ));

    // 4. Empty space - only the bottom connects to flat surfaces
    let mut empty_connections: HashMap<Direction, String> = HashMap::new();
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

        // cube variant to be a wall that connects to roads
    let mut cube_road_conns = HashMap::new();

    cube_road_conns.insert(Direction::PosX, "road_end".to_string());
    cube_road_conns.insert(Direction::NegX, "road_end".to_string());
    cube_road_conns.insert(Direction::PosY, "flat".to_string());
    cube_road_conns.insert(Direction::NegY, "flat".to_string());
    cube_road_conns.insert(Direction::PosZ, "road_end".to_string());
    cube_road_conns.insert(Direction::NegZ, "road_end".to_string());
    tiles.push(TileType::new(
        4,
        "Cube Road",
        "cube",  // Still a cube visually
        cube_road_conns,
        1.5,
    ));

    tiles
}
