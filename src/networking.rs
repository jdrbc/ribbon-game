use bevy::prelude::*;
use avian3d::prelude::*;
// TODO: Re-enable when GGRS configuration is properly set up
// use bevy_ggrs::*;
// use bevy_ggrs::ggrs::{PlayerHandle, Config, P2PSession, SessionBuilder};
// use matchbox_socket::{WebRtcSocket, PeerId};
use serde::{Deserialize, Serialize};
use crate::components::*;
use crate::resources::*;
use crate::{GameState};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Add GGRS plugin when dependencies are working
        app
            .init_resource::<LobbyState>()
            .init_resource::<ConnectionInfo>();
            // TODO: Add network systems when implemented
            // .add_systems(Update, (
            //     handle_network_events.run_if(in_state(GameState::Lobby)),
            //     update_connection_status,
            //     sync_network_players.run_if(in_state(GameState::InGame)),
            // ));
    }
}

// TODO: Implement full networking systems once GGRS dependencies are resolved
/*
// GGRS input system
fn input_system(
    _handle: In<PlayerHandle>,
    input_query: Query<&NetworkInput, With<LocalPlayer>>,
) -> NetworkInput {
    if let Ok(input) = input_query.get_single() {
        *input
    } else {
        NetworkInput::default()
    }
}

// Network movement system that runs in rollback
fn network_player_movement(
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut player_query: Query<(&mut Player, &mut ExternalImpulse, &Transform), With<Rollback>>,
) {
    for (mut player, mut impulse, transform) in player_query.iter_mut() {
        let input = inputs[player.network_id].0;
        let speed = player.movement_speed;

        // Apply horizontal movement using impulses
        let movement_force = Vec3::new(
            input.movement.x * speed,
            0.0,
            -input.movement.y * speed,
        );

        if movement_force.length() > 0.1 {
            impulse.apply_impulse(movement_force);
        }
    }
}

// Network jump system that runs in rollback
fn network_jump_system(
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut player_query: Query<(&mut Player, &mut ExternalImpulse), With<Rollback>>,
) {
    for (mut player, mut impulse) in player_query.iter_mut() {
        let input = inputs[player.network_id].0;

        if input.jump && player.can_jump && player.is_grounded {
            let jump_impulse = Vec3::new(0.0, player.jump_force, 0.0);
            impulse.apply_impulse(jump_impulse);
            player.can_jump = false;
        }
    }
}

// Handle WebRTC socket events
pub fn handle_network_events(
    mut network_session: ResMut<NetworkSession>,
    mut lobby_state: ResMut<LobbyState>,
) {
    if let Some(socket) = &mut network_session.socket {
        // Handle new peer connections
        for (peer, new_state) in socket.accept_new_connections() {
            info!("New peer connected: {:?}", peer);
            lobby_state.player_count += 1;
        }

        // Handle peer disconnections
        for peer in socket.disconnected_peers() {
            info!("Peer disconnected: {:?}", peer);
            lobby_state.player_count = lobby_state.player_count.saturating_sub(1);
        }
    }
}

pub fn update_connection_status(
    network_session: Res<NetworkSession>,
    mut connection_info: ResMut<ConnectionInfo>,
) {
    if let Some(socket) = &network_session.socket {
        match socket.connected_peers().len() {
            0 => {
                connection_info.is_connecting = false;
                connection_info.connection_error = Some("No peers connected".to_string());
            }
            _ => {
                connection_info.is_connecting = false;
                connection_info.connection_error = None;
            }
        }
    }
}

pub fn sync_network_players(
    mut commands: Commands,
    player_query: Query<(Entity, &Player)>,
    local_player_query: Query<Entity, With<LocalPlayer>>,
    remote_player_query: Query<Entity, With<RemotePlayer>>,
    lobby_state: Res<LobbyState>,
) {
    // Ensure proper player entity management based on network state
    for (entity, player) in player_query.iter() {
        if player.is_local {
            if local_player_query.get(entity).is_err() {
                commands.entity(entity).insert(LocalPlayer);
            }
        } else {
            if remote_player_query.get(entity).is_err() {
                commands.entity(entity).insert(RemotePlayer);
            }
        }
    }
}

// Utility functions for network setup
pub fn create_p2p_session(
    local_player_handle: PlayerHandle,
    num_players: usize,
    input_delay: usize,
) -> Result<P2PSession<GGRSConfig>, Box<dyn std::error::Error>> {
    let session = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(input_delay)
        .start_p2p_session(local_player_handle)?;
    Ok(session)
}

pub async fn connect_to_room(room_id: &str) -> Result<WebRtcSocket, Box<dyn std::error::Error>> {
    let room_url = format!("ws://127.0.0.1:3536/{}", room_id);
    let (socket, _) = WebRtcSocket::new_reliable(&room_url);
    Ok(socket)
}
*/ 