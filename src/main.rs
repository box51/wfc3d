mod wfc;
mod tile_types;
mod lighting;
mod debug_axis;
mod ui;
mod core;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use core::{GridDimensions, setup_scene, generate_wfc_system};
use ui::{setup_ui, handle_ui_interactions};

#[derive(Event)]
pub struct GenerateWFCEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .init_resource::<ui::UIState>()
        .init_resource::<GridDimensions>()
        .add_event::<GenerateWFCEvent>()
        .add_systems(Startup, (
            setup_scene,
            lighting::setup_lighting,
            debug_axis::spawn_axis_arrows,
            setup_ui
        ))
        .add_systems(Update, (handle_ui_interactions, generate_wfc_system))
        .run();
}