use bevy::prelude::*;

/// Creates XYZ axis visualization for spatial orientation
/// X = Red (pointing right), Y = Green (pointing up), Z = Blue (pointing forward)
pub fn spawn_axis_arrows(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Offset the entire axis system to avoid overlapping with objects at origin
    let axis_offset = Vec3::new(-10.0, -5.0, -10.0); // Adjust this to position the axis where you want

    let axis_length = 5.0;
    let axis_radius = 0.1;
    let arrow_head_height = 0.5;
    let arrow_head_radius = 0.25;

    // X-Axis (Red) - pointing in positive X direction
    // Cylinder shaft
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::new(axis_radius, axis_length)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            ..default()
        }),
        transform: Transform::from_xyz(
            axis_offset.x + axis_length / 2.0,
            axis_offset.y,
            axis_offset.z
        ).with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 2.0)),
        ..default()
    });
    // Arrow head (cone)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cone {
            radius: arrow_head_radius,
            height: arrow_head_height,
        }),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            ..default()
        }),
        transform: Transform::from_xyz(
            axis_offset.x + axis_length + arrow_head_height / 2.0,
            axis_offset.y,
            axis_offset.z
        ).with_rotation(Quat::from_rotation_z(-std::f32::consts::PI / 2.0)),
        ..default()
    });

    // Y-Axis (Green) - pointing in positive Y direction (up)
    // Cylinder shaft
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::new(axis_radius, axis_length)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 1.0, 0.0),
            ..default()
        }),
        transform: Transform::from_xyz(
            axis_offset.x,
            axis_offset.y + axis_length / 2.0,
            axis_offset.z
        ),
        ..default()
    });
    // Arrow head (cone)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cone {
            radius: arrow_head_radius,
            height: arrow_head_height,
        }),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 1.0, 0.0),
            ..default()
        }),
        transform: Transform::from_xyz(
            axis_offset.x,
            axis_offset.y + axis_length + arrow_head_height / 2.0,
            axis_offset.z
        ),
        ..default()
    });

    // Z-Axis (Blue) - pointing in positive Z direction (forward)
    // Cylinder shaft
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::new(axis_radius, axis_length)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 1.0),
            ..default()
        }),
        transform: Transform::from_xyz(
            axis_offset.x,
            axis_offset.y,
            axis_offset.z + axis_length / 2.0
        ).with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0)),
        ..default()
    });
    // Arrow head (cone)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cone {
            radius: arrow_head_radius,
            height: arrow_head_height,
        }),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 1.0),
            ..default()
        }),
        transform: Transform::from_xyz(
            axis_offset.x,
            axis_offset.y,
            axis_offset.z + axis_length + arrow_head_height / 2.0
        ).with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0)),
        ..default()
    });

    // Origin sphere at the base of the axes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(0.2)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0),
            ..default()
        }),
        transform: Transform::from_xyz(axis_offset.x, axis_offset.y, axis_offset.z),
        ..default()
    });
}