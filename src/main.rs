// main.rs
mod wfc;
mod tile_types;
mod lighting;
mod debug_axis;  // Add this new module

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use wfc::{WFCGrid, Direction, TileType};
use lighting::setup_lighting;
use tile_types::create_tile_types;
use debug_axis::spawn_axis_arrows;

// Cube side width in world units
const TILE_SIZE: f32 = 3.0;
// Grid dimensions
const X_DIM: usize = 3;
const Y_DIM: usize = 4;
const Z_DIM: usize = 5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, (setup, setup_lighting, spawn_axis_arrows))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(40.0)),
        material: materials.add(Color::srgb(0.1, 0.7, 0.2)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                -2.5,
                Y_DIM as f32 * TILE_SIZE * 1.5,
                Z_DIM as f32 * TILE_SIZE + 5.0
            ).looking_at(
                Vec3::new(
                    X_DIM as f32 * TILE_SIZE / 2.0,
                    Y_DIM as f32 * TILE_SIZE / 2.0,
                    Z_DIM as f32 * TILE_SIZE / 2.0
                ),
                Vec3::Y
            ),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    // Generate the voxel structure using WFC
    let tile_types = create_tile_types();
    let mut rng = rand::thread_rng();
    let mut grid = WFCGrid::new((X_DIM, Y_DIM, Z_DIM), tile_types);

    if grid.collapse_all(&mut rng) {
        println!("Successfully generated WFC structure!");

        // Spawn tiles based on the collapsed grid
        for ((x, y, z), cell) in &grid.cells {
            if let Some(tile_id) = cell.collapsed {
                let tile_type = &grid.tile_types[tile_id];
                let world_pos = Vec3::new(
                    *x as f32 * TILE_SIZE,
                    *y as f32 * TILE_SIZE,
                    *z as f32 * TILE_SIZE,
                );

                spawn_tile(
                    &mut commands,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    tile_type,
                    world_pos,
                );
            }
        }
    } else {
        println!("Failed to generate valid WFC structure!");
    }
}

fn spawn_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tile_type: &TileType,
    position: Vec3,
) {
    match tile_type.visual_type.as_str() {
        "cube" => {
            // Always try to load GLTF model first, Bevy will handle missing assets gracefully
            commands.spawn(SceneBundle {
                scene: asset_server.load("cube.gltf#Scene0"),
                transform: Transform::from_translation(position),
                ..default()
            });
        }
        "straight" | "road" => {
            commands.spawn(SceneBundle {
                scene: asset_server.load("road.gltf#Scene0"),
                transform: Transform::from_translation(position)
                    .with_rotation(get_rotation_for_connections(&tile_type.connections)),
                ..default()
            });
        }
        "top" => {
            commands.spawn(SceneBundle {
                scene: asset_server.load("top.gltf#Scene0"),
                transform: Transform::from_translation(position),
                ..default()
            });
        }
        "empty" => {
            // Don't spawn anything for empty tiles
        }
        _ => {
            // Default fallback: basic cube
            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
                transform: Transform::from_translation(position),
                ..default()
            });
        }
    }
}

// Helper function to determine rotation based on connection types
fn get_rotation_for_connections(connections: &std::collections::HashMap<Direction, String>) -> Quat {
    // Determine primary axis based on male/female connections
    let mut rotation = Quat::IDENTITY;

    // Check which directions have male connectors to determine orientation
    let has_male_x = connections.get(&Direction::PosX)
        .map(|c| c == "male")
        .unwrap_or(false);
    let has_male_z = connections.get(&Direction::PosZ)
        .map(|c| c == "male")
        .unwrap_or(false);

    if has_male_z && !has_male_x {
        // Rotate 90 degrees around Y axis
        rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
    } else if has_male_x && has_male_z {
        // Corner piece - rotate 45 degrees
        rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);
    }

    rotation
}
