pub mod components;
pub mod systems;
pub mod resources;
pub mod networking;
pub mod ui;
pub mod physics;
pub mod input;

// Re-export commonly used items
pub use components::*;
pub use resources::*;

// Game state management
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Lobby,
    InGame,
    GameOver,
    SinglePlayer,
}

pub struct RibbonGamePlugin;

impl Plugin for RibbonGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins((
                physics::PhysicsPlugin,
                input::InputPlugin,
                networking::NetworkingPlugin,
                ui::UIPlugin,
                systems::GameSystemsPlugin,
            ));
    }
} 