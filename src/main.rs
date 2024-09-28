mod objs;
mod tile_factory;
use objs::{Tile, TileType, Position, Bond};
use tile_factory::{create_tile_options, select_random_start_tile};
use rand::Rng;
use bevy::{prelude::*, reflect::Array};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

// cube side width in pixels
static WIDTH: u32 = 3;
// final assemble dimension DIMxDIMxDIM
const X_DIM: usize = 3;
const Y_DIM: usize = 3;
const Z_DIM: usize = 3;

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
                scene: asset_server.load("road.gltf#Scene0"),
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, position.z),
                    rotation: Quat::from_rotation_y(90.0_f32.to_radians()), // Rotate 45 degrees around the Y-axis
                    scale: Vec3::ONE, // Default scale (no scaling)
                },
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

    let tile_corpus: [Tile; 3] = create_tile_options();

    let default_tile: Tile = Tile::default();
    // Initialize the 3D array manually
    // Initialize the 3D array
    let mut objs_3d: Vec<Vec<Vec<Tile>>> = vec![
        vec![
            vec![default_tile.clone(); Z_DIM];
            Y_DIM
        ];
        X_DIM
    ];    // Use nested loops to set each element to the default_tile

    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate random indices
    let x = rng.gen_range(0..X_DIM);
    let y = 0;  // Start at the bottom
    let z = rng.gen_range(0..Z_DIM);

    // TODO: check if tile xists
    let start_tile: Tile = select_random_start_tile(&tile_corpus.to_vec());
    objs_3d[x][y][z] = start_tile.clone();
    let selected_tile: &mut Tile = &mut objs_3d[x][y][z];
    selected_tile.collapsed = true;

    //update entropy

    // for x in 0..X_DIM {
    //     for y in 0..Y_DIM {
    //         for z in 0..Z_DIM {
    //             objs_3d[x][y][z] =  Tile {
    //                 collapsed: false,
    //                 tile_type: TileType::Road,
    //                 entropy: 42,
    //                 bonds: vec![Bond{id: "face".to_string(), vectdir: [0, 0, 0]}]
    //                 starter: false
    //             };
    //         }
    //     }
    // }

    // Creating the array
    for x in 0..X_DIM {
        for y in 0..Y_DIM {
            for z in 0..Z_DIM {
                let x_cart: f32 = x as f32 * (WIDTH as f32);
                let y_cart: f32 = y as f32 * (WIDTH as f32);
                let z_cart: f32 = z as f32 * (WIDTH as f32);
                let pos = Position{ x: x_cart, y: y_cart, z: z_cart};
                if objs_3d[x][y][z].collapsed {
                    spawn_tile_at_position(&mut commands, &asset_server, &objs_3d[x][y][z], &pos);
                }
            }
        }
    }
}