use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameState {
    pub current_state: CurrentGameState,
}

#[derive(Default)]
pub enum CurrentGameState {
    #[default]
    SinglePlayer,
    Lobby,
    InGame,
    GameOver,
}

#[derive(Resource)]
pub struct ArenaConfig {
    pub length: f32,
    pub width: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            length: 15.0, // 15 meters long
            width: 5.0,   // 5 meters wide
        }
    }
} 