use bevy::prelude::*;
use avian3d::prelude::*;

mod components;
mod systems;
mod resources;

use components::*;
use systems::*;
use resources::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        // Initialize resources
        .init_resource::<GameState>()
        // Add startup systems
        .add_systems(Startup, (
            setup_scene,
            setup_camera,
            spawn_player,
        ))
        // Add update systems
        .add_systems(Update, (
            update_ground_detection,
            player_movement,
            camera_follow,
            handle_boundaries,
        ))
        .run();
} 