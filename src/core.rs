use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use std::collections::HashMap;
use crate::wfc::{WFCGrid, Direction, TileType};
use crate::tile_types::create_tile_types;
use crate::GenerateWFCEvent;

// Constants
pub const TILE_SIZE: f32 = 3.0;

// Resources
#[derive(Resource)]
pub struct GridDimensions {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Default for GridDimensions {
    fn default() -> Self {
        Self { x: 9, y: 9, z: 9 }
    }
}

// Components
#[derive(Component)]
pub struct WFCTile;

// Scene setup
pub fn setup_scene(
    mut commands: Commands,
    dimensions: Res<GridDimensions>,
) {
    // Camera only - no ground plane
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(
                -2.5,
                dimensions.y as f32 * TILE_SIZE * 1.5,
                dimensions.z as f32 * TILE_SIZE + 5.0
            ).looking_at(
                Vec3::new(
                    dimensions.x as f32 * TILE_SIZE / 2.0,
                    dimensions.y as f32 * TILE_SIZE / 2.0,
                    dimensions.z as f32 * TILE_SIZE / 2.0
                ),
                Vec3::Y
            ),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

// WFC Generation System
pub fn generate_wfc_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<GenerateWFCEvent>,
    existing_tiles: Query<Entity, With<WFCTile>>,
    dimensions: Res<GridDimensions>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for _ in event_reader.read() {
        // Remove existing tiles
        for entity in existing_tiles.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Update camera
        for mut transform in camera_query.iter_mut() {
            *transform = Transform::from_xyz(
                -2.5,
                dimensions.y as f32 * TILE_SIZE * 1.5,
                dimensions.z as f32 * TILE_SIZE + 5.0
            ).looking_at(
                Vec3::new(
                    dimensions.x as f32 * TILE_SIZE / 2.0,
                    dimensions.y as f32 * TILE_SIZE / 2.0,
                    dimensions.z as f32 * TILE_SIZE / 2.0
                ),
                Vec3::Y
            );
        }

        // Generate WFC
        let tile_types = create_tile_types();
        let mut rng = rand::thread_rng();
        let mut grid = WFCGrid::new((dimensions.x, dimensions.y, dimensions.z), tile_types);

        if grid.collapse_all(&mut rng) {
            println!("Successfully generated WFC structure with dimensions {}x{}x{}!",
                     dimensions.x, dimensions.y, dimensions.z);

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
}

fn spawn_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tile_type: &TileType,
    position: Vec3,
) {
    let mut entity_commands = commands.spawn(WFCTile);

    match tile_type.visual_type.as_str() {
        "cube" => {
            entity_commands.insert(SceneBundle {
                scene: asset_server.load("cube.gltf#Scene0"),
                transform: Transform::from_translation(position),
                ..default()
            });
        }
        "straight" | "road" => {
            entity_commands.insert(SceneBundle {
                scene: asset_server.load("road.gltf#Scene0"),
                transform: Transform::from_translation(position)
                    .with_rotation(get_rotation_for_connections(&tile_type.connections)),
                ..default()
            });
        }
        "top" => {
            entity_commands.insert(SceneBundle {
                scene: asset_server.load("top.gltf#Scene0"),
                transform: Transform::from_translation(position),
                ..default()
            });
        }
        "empty" => {
            entity_commands.despawn();
        }
        _ => {
            entity_commands.insert(PbrBundle {
                mesh: meshes.add(Cuboid::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
                transform: Transform::from_translation(position),
                ..default()
            });
        }
    }
}

fn get_rotation_for_connections(connections: &HashMap<Direction, String>) -> Quat {
    let mut rotation = Quat::IDENTITY;

    let has_male_x = connections.get(&Direction::PosX)
        .map(|c| c == "male")
        .unwrap_or(false);
    let has_male_z = connections.get(&Direction::PosZ)
        .map(|c| c == "male")
        .unwrap_or(false);

    if has_male_z && !has_male_x {
        rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
    } else if has_male_x && has_male_z {
        rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);
    }

    rotation
}