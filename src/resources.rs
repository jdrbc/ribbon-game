use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// TODO: Re-enable when networking dependencies are resolved
// use ggrs::{PlayerHandle, SessionState};
// use matchbox_socket::WebRtcSocket;

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

impl ArenaConfig {
    pub fn new() -> Self {
        Self {
            length: 150.0, // 150 meters long
            width: 50.0,   // 50 meters wide
        }
    }
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self::new()
    }
}

// Lobby management
#[derive(Resource, Default)]
pub struct LobbyState {
    pub room_id: Option<String>,
    pub player_count: usize,
    pub max_players: usize,
    pub is_host: bool,
    pub players: Vec<LobbyPlayer>,
    pub game_started: bool,
    pub local_player_name: String,
    pub chat_messages: Vec<ChatMessage>,
    pub current_chat_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyPlayer {
    pub id: Uuid,
    pub name: String,
    pub is_ready: bool,
    pub network_handle: u32, // Simplified for now
    pub is_local: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender: String,
    pub message: String,
    pub timestamp: f64,
    pub message_type: ChatMessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatMessageType {
    Player,
    System,
    Info,
}

// TODO: Re-enable when networking dependencies are resolved
/*
// Network session management
#[derive(Resource)]
pub struct NetworkSession {
    pub socket: Option<WebRtcSocket>,
    pub local_player_handle: PlayerHandle,
    pub session_state: SessionState,
}

impl Default for NetworkSession {
    fn default() -> Self {
        Self {
            socket: None,
            local_player_handle: 0,
            session_state: SessionState::Synchronizing,
        }
    }
}
*/

// Input delay for rollback netcode
#[derive(Resource)]
pub struct NetworkConfig {
    pub input_delay: usize,
    pub check_distance: u32,
    pub max_prediction: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            input_delay: 2, // 2 frames of input delay
            check_distance: 3,
            max_prediction: 8,
        }
    }
}

// Game statistics and tracking
#[derive(Resource, Default)]
pub struct GameStats {
    pub kills: std::collections::HashMap<u32, u32>, // Simplified player ID
    pub deaths: std::collections::HashMap<u32, u32>,
    pub advancements: std::collections::HashMap<u32, u32>,
}

// Connection and matchmaking
#[derive(Resource, Default)]
pub struct ConnectionInfo {
    pub room_url: String,
    pub room_id: String,
    pub is_connecting: bool,
    pub connection_error: Option<String>,
} 