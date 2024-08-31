//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

// cube side width in pixels
static WIDTH: u32 = 3;
// final assemble dimension DIMxDIMxDIM
const X_DIM: usize = 3;
const Y_DIM: usize = 3;
const Z_DIM: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Top,
    Cube,
    Road,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    collapsed: bool,
    tile_type: TileType,
    entropy: u32,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            collapsed: false,
            tile_type: TileType::Cube, // Default type
            entropy: 0,
        }
    }
}

// Define the Position struct
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

// Define the function that takes a Tile and a Position as parameters
fn spawn_tile_at_position(commands: &mut Commands, asset_server: &Res<AssetServer>,
                          tile: &Tile, position: &Position) {
    println!("Processing tile at position ({}, {}, {}):",
              position.x, position.y, position.z);
    println!("Collapsed: {}", tile.collapsed);

    match tile.tile_type {
        TileType::Cube => {
                commands.spawn(SceneBundle {
                    scene : asset_server.load("cube.gltf#Scene0"),
                    transform: Transform::from_xyz(position.x, position.y, position.z),
                    ..default()
                });
        }
        TileType::Top => {
                commands.spawn(SceneBundle {
                    scene : asset_server.load("top.gltf#Scene0"),
                    transform: Transform::from_xyz(position.x, position.y, position.z),
                    ..default()
                });
        }
        TileType::Road => {
                commands.spawn(SceneBundle {
                    scene : asset_server.load("road.gltf#Scene0"),
                    transform: Transform::from_xyz(position.x, position.y, position.z),
                    ..default()
                });
        }
        // None => println!("Type: None"),
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(PanOrbitCameraPlugin)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // circular base
    commands.spawn((PbrBundle {
        mesh: meshes.add(Circle::new(20.0)),
        material: materials.add(Color::srgb_u8(25, 190, 55)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
        },
    ));
    // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //     material: materials.add(Color::srgb_u8(124, 144, 255)),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, Y_DIM as f32 * WIDTH as f32 * 1.5, 4.0),
        ..default()
    });
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.5, Y_DIM as f32 * WIDTH as f32 * 1.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
        PanOrbitCamera::default(),
    ));
    let default_tile = Tile::default();
    // Initialize the 3D array manually
    let mut objs_3d: [[[Tile; Z_DIM]; Y_DIM]; X_DIM] = [[[default_tile; Z_DIM]; Y_DIM]; X_DIM];
    // Use nested loops to set each element to the default_tile
    for x in 0..X_DIM {
        for y in 0..Y_DIM {
            for z in 0..Z_DIM {
                objs_3d[x][y][z] =  Tile {
                    collapsed: false,
                    tile_type: TileType::Road,
                    entropy: 42,
                };
            }
        }
    }

    // Creating the array
    for x in 0..X_DIM {
        for y in 0..Y_DIM {
            for z in 0..Z_DIM {
                let x_cart: f32 = x as f32 * (WIDTH as f32);
                let y_cart: f32 = y as f32 * (WIDTH as f32);
                let z_cart: f32 = z as f32 * (WIDTH as f32);
                let pos = Position{ x: x_cart, y: y_cart, z: z_cart};
                spawn_tile_at_position(&mut commands, &asset_server, &objs_3d[x][y][z], &pos)
            }
        }
    }
}