use bevy::prelude::*;
// use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Arena Configuration
#[derive(Resource, Debug, Clone)]
pub struct ArenaConfig {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub length: f32,
    pub boundary_force: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            width: 10.0,
            height: 10.0,
            depth: 10.0,
            length: 10.0,
            boundary_force: 50.0,
        }
    }
}

// Game Statistics
#[derive(Resource, Debug, Clone, Default)]
pub struct GameStats {
    pub kills: std::collections::HashMap<u32, u32>,
    pub deaths: u32,
    pub shots_fired: u32,
    pub shots_hit: u32,
    pub game_time: f32,
}

// Lobby Management
#[derive(Resource, Debug, Clone)]
pub struct LobbyState {
    pub room_id: Option<String>,
    pub max_players: usize,
    pub connected_players: Vec<String>,
    pub is_host: bool,
    pub player_count: usize,
    pub players: Vec<LobbyPlayer>,
    pub game_started: bool,
    pub local_player_name: String,
    pub chat_messages: Vec<ChatMessage>,
    pub current_chat_input: String,
}

#[derive(Debug, Clone)]
pub struct LobbyPlayer {
    pub id: Uuid,
    pub name: String,
    pub is_ready: bool,
    pub network_handle: u32, // Simplified for now
    pub is_local: bool,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub message: String,
    pub timestamp: f64,
    pub message_type: ChatMessageType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatMessageType {
    Player,
    System,
    Info,
}

impl Default for LobbyState {
    fn default() -> Self {
        Self {
            room_id: None,
            max_players: 2,
            connected_players: Vec::new(),
            is_host: false,
            player_count: 0,
            players: Vec::new(),
            game_started: false,
            local_player_name: "Player".to_string(),
            chat_messages: Vec::new(),
            current_chat_input: String::new(),
        }
    }
}

// Connection Information
#[derive(Resource, Debug, Clone, Default)]
pub struct ConnectionInfo {
    pub player_id: Option<Uuid>,
    pub server_url: String,
    pub connection_status: ConnectionStatus,
    pub room_id: String,
    pub is_connecting: bool,
    pub connection_error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Failed(String),
}

// Network Session Information
#[derive(Resource, Debug, Clone, Default)]
pub struct NetworkSession {
    pub session_id: Option<String>,
    pub is_active: bool,
    pub frame_count: u32,
} 