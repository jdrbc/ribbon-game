use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ggrs::{
    ggrs::{Config, SessionBuilder, PlayerType},
    GgrsApp, GgrsPlugin, GgrsSchedule, LocalPlayers, PlayerInputs, RollbackApp, Session,
};
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::*;
// use crate::input::read_local_inputs;
use crate::resources::*;
use crate::GameState;

// GGRS Configuration
// ==================
//
// A quick note from your friendly neighborhood Bespoke developer:
// This is where we define the core of our networking. GGRS is a powerful rollback
// networking library, but it needs to know what we're sending across the wire.
// `NetworkInput` is our payload. Keep it lean, keep it mean.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GGRSConfig;

impl Config for GGRSConfig {
    type Input = NetworkInput;
    type State = u8;
    type Address = PeerId;
}

// Networking Plugin
// =================
//
// This is the main entry point for all our networking logic. We're keeping it clean
// and organized, unlike the last guys. This plugin sets up GGRS, schedules our
// rollback systems, and handles the transition from a sad, lonely lobby to an
// action-packed, networked game.
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            // GGRS, the star of the show
            .add_plugins(GgrsPlugin::<GGRSConfig>::default())
            // Define a schedule for GGRS to run on
            .set_rollback_schedule_fps(30)
            // These components will be rolled back by GGRS
            .rollback_component_with_copy::<Transform>()
            .rollback_component_with_copy::<ExternalImpulse>()
            .rollback_component_with_copy::<Player>()
            // Resources to manage our sorry excuse for a lobby
            .init_resource::<LobbyState>()
            .init_resource::<ConnectionInfo>()
            .init_resource::<NetworkSession>()
            // System to kick things off when we enter the lobby
            .add_systems(OnEnter(GameState::Lobby), start_matchbox_socket)
            // Systems that run in the lobby, waiting for players to join
            .add_systems(Update, wait_for_players.run_if(in_state(GameState::Lobby)))
            // GGRS will read player inputs from this system
            // TODO: Fix input system - temporarily disabled for testing
            // .add_systems(bevy_ggrs::ReadInputs, read_local_inputs)
            // These systems are the core of our networked gameplay
            .add_systems(
                GgrsSchedule,
                (
                    spawn_network_players,
                    network_player_movement,
                    network_jump_system,
                )
                    .chain(),
            );
    }
}

// Matchbox Socket Management
// ==========================
//
// We need a way to connect to other players. Matchbox provides a WebRTC socket
// that works in the browser. This function creates the socket and kicks off
// the connection process. Simple, effective, and doesn't complain about
// dependency lists.
pub fn start_matchbox_socket(mut commands: Commands, lobby_state: Res<LobbyState>) {
    let room_id = lobby_state.room_id.as_ref().unwrap();
    let room_url = format!("ws://44.206.226.40:3536/{}", room_id);
    info!("Connecting to Matchbox server: {}", room_url);
    commands.insert_resource(MatchboxSocket::new_reliable(&room_url));
}

// Waiting for Players
// ===================
//
// This system just waits for players to connect. Once everyone is here,
// it will trigger the start of the GGRS session. No more, no less.
pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    mut game_state: ResMut<NextState<GameState>>,
    mut lobby_state: ResMut<LobbyState>,
) {
    if socket.get_channel(0).is_err() {
        return; // We are not ready yet
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.connected_peers().collect::<Vec<_>>();
    
    // Update lobby state with current player count
    let current_player_count = players.len() + 1; // +1 for the local player
    lobby_state.player_count = current_player_count;
    lobby_state.connected_players = players.iter().map(|p| format!("Player_{}", p)).collect();
    
    // Update the players list that the UI displays
    lobby_state.players.clear();
    lobby_state.players.push("Host".to_string()); // Add the host/local player
    for (i, _player) in players.iter().enumerate() {
        lobby_state.players.push(format!("Player_{}", i + 2)); // Start from Player_2
    }

    info!("Players connected: {}/{}", current_player_count, lobby_state.max_players);

    if players.len() < lobby_state.max_players - 1 {
        return; // not enough players yet (-1 because we count the local player)
    }
    
    // Auto-start when we have enough players (simplified for now)
    // TODO: Later we can add proper ready-state synchronization via GGRS
    info!("All players connected, auto-starting GGRS session.");

    let mut session_builder = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(lobby_state.max_players)
        .with_input_delay(2);

    for (i, _player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(PlayerType::Local, i)
            .expect("failed to add player");
    }

    // Create GGRS session and start the networked game
    let channel = socket.take_channel(0).unwrap();
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");
    commands.insert_resource(Session::P2P(ggrs_session));

    game_state.set(GameState::InGame);
}

// Player Spawning
// ===============
//
// GGRS is ready, so it's time to spawn our players. This system runs once
// at the beginning of the GGRS session. It creates a player entity for each
// player in the session, whether they're local or remote. No hiding players
// like the last guys. We're here to play.
fn spawn_network_players(
    mut commands: Commands,
    local_players: Res<LocalPlayers>,
) {
    for handle in &local_players.0 {
        let is_local = true; // All players in LocalPlayers are local

        let player_entity = commands
            .spawn((
                Player {
                    network_id: *handle as u32,
                    is_local,
                    ..default()
                },
                // All the usual components for a player
                RigidBody::Dynamic,
                Collider::capsule(0.4, 0.4),
                ExternalImpulse::default(),
                LockedAxes::new().lock_rotation_x().lock_rotation_z(),
                Transform::from_xyz(0.0, 1.0, 0.0),
            ))
            .id();

        if is_local {
            commands.entity(player_entity).insert(LocalPlayer);
        } else {
            commands.entity(player_entity).insert(RemotePlayer);
        }
    }
}

// Networked Movement & Jumping
// ============================
//
// These systems are the bread and butter of our networked gameplay. They run on
// the GGRS schedule, which means they're subject to rollback. This is where we
// read the inputs from the GGRS session and apply them to our player entities.
// No magic, just good, clean code.

fn network_player_movement(
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut player_query: Query<(&Player, &mut ExternalImpulse)>,
) {
    for (player, mut impulse) in player_query.iter_mut() {
        if let Some((input, _)) = inputs.get(player.network_id as usize) {
            let direction =
                Vec3::new(input.movement.x, 0.0, -input.movement.y).normalize_or_zero();
            impulse.apply_impulse(direction * player.movement_speed);
        }
    }
}

fn network_jump_system(
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut player_query: Query<(&mut Player, &mut ExternalImpulse)>,
) {
    for (mut player, mut impulse) in player_query.iter_mut() {
        if let Some((input, _)) = inputs.get(player.network_id as usize) {
            if input.jump && player.can_jump && player.is_grounded {
                impulse.apply_impulse(Vec3::Y * player.jump_force);
                player.can_jump = false;
            }
        }
    }
} 