use bevy::prelude::*;

/// Sets up balanced directional lights for even coverage
pub fn setup_lighting(mut commands: Commands) {
    // Define 4 directional lights evenly distributed (90° apart)
    // All use -45° pitch (angling down from above) for consistency
    let light_directions = [
        // North (0°) - Primary shadow caster
        ((-45.0_f32.to_radians(), 0.0_f32.to_radians()), true),
        // East (90°) - No shadows to avoid overlap
        ((-45.0_f32.to_radians(), 90.0_f32.to_radians()), false),
        // South (180°) - Secondary shadow caster
        ((-45.0_f32.to_radians(), 180.0_f32.to_radians()), true),
        // West (270° or -90°) - No shadows to avoid overlap
        ((-45.0_f32.to_radians(), 270.0_f32.to_radians()), false),
    ];

    // Spawn all 4 directional lights with selective shadow settings
    for ((pitch, yaw), shadows) in light_directions.iter() {
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 1500.0, // Slightly reduced since we have 4 lights
                shadows_enabled: *shadows,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                *pitch,
                *yaw,
                0.0
            )),
            ..default()
        });
    }

    // // Set moderate ambient light to fill in shadows
    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 2.0, // Increased to better fill in dark shadows
    // });
}