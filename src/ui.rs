use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};
use crate::resources::*;
use crate::{GameState};
use uuid::Uuid;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
            .add_systems(EguiContextPass, (
                main_menu_ui.run_if(in_state(GameState::MainMenu)),
                lobby_ui.run_if(in_state(GameState::Lobby)),
                in_game_ui.run_if(in_state(GameState::InGame)),
            ));
    }
}

pub fn main_menu_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    mut lobby_state: ResMut<LobbyState>,
    mut connection_info: ResMut<ConnectionInfo>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            
            ui.heading("Ribbon Game");
            ui.add_space(50.0);

            // Create Game button
            if ui.button("Create Game").clicked() {
                let room_id = Uuid::new_v4().to_string()[..8].to_string();
                lobby_state.room_id = Some(room_id.clone());
                lobby_state.is_host = true;
                lobby_state.max_players = 4;
                lobby_state.player_count = 1;
                connection_info.room_id = room_id;
                game_state.set(GameState::Lobby);
            }

            ui.add_space(20.0);

            // Join Game section
            ui.horizontal(|ui| {
                ui.label("Room ID:");
                ui.text_edit_singleline(&mut connection_info.room_id);
            });

            if ui.button("Join Game").clicked() && !connection_info.room_id.is_empty() {
                lobby_state.room_id = Some(connection_info.room_id.clone());
                lobby_state.is_host = false;
                lobby_state.max_players = 4;
                lobby_state.player_count = 1;
                game_state.set(GameState::Lobby);
            }

            ui.add_space(20.0);

            // Single Player (for development/testing)
            // A quick note from your friendly neighborhood Bespoke developer:
            // This is for testing only. Don't get any funny ideas about
            // shipping a single-player mode without talking to us first.
            if ui.button("Single Player (Dev)").clicked() {
                game_state.set(GameState::SinglePlayer);
            }

            // Display connection errors
            if let Some(error) = &connection_info.connection_error {
                ui.add_space(20.0);
                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            }
        });
    });
}

pub fn lobby_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    mut lobby_state: ResMut<LobbyState>,
    connection_info: Res<ConnectionInfo>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            ui.heading("Game Lobby");
            ui.add_space(30.0);

            // Room information
            if let Some(room_id) = &lobby_state.room_id {
                ui.label(format!("Room ID: {}", room_id));
            }
            
            ui.label(format!("Players: {}/{}", lobby_state.player_count, lobby_state.max_players));
            ui.add_space(20.0);

            // Player list
            ui.group(|ui| {
                ui.label("Players:");
                for (i, player) in lobby_state.players.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}. {}", i + 1, player));
                        ui.colored_label(egui::Color32::GREEN, "Ready");
                    });
                }
            });

            ui.add_space(20.0);

            // Host controls
            if lobby_state.is_host {
                if ui.button("Start Game").clicked() && lobby_state.player_count >= 2 {
                    lobby_state.game_started = true;
                    game_state.set(GameState::InGame);
                }
                
                if lobby_state.player_count < 2 {
                    ui.colored_label(egui::Color32::GRAY, "Need at least 2 players to start");
                }
            } else {
                ui.label("Waiting for host to start the game...");
            }

            ui.add_space(20.0);

            // Back to main menu
            if ui.button("Back to Main Menu").clicked() {
                lobby_state.room_id = None;
                lobby_state.players.clear();
                lobby_state.player_count = 0;
                lobby_state.is_host = false;
                game_state.set(GameState::MainMenu);
            }

            // Connection status
            if connection_info.is_connecting {
                ui.spinner();
                ui.label("Connecting...");
            }
        });
    });
}

pub fn in_game_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    game_stats: Option<Res<GameStats>>,
    lobby_state: Res<LobbyState>,
) {
    // Top-left HUD
    egui::Window::new("Game HUD")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Controls:");
            ui.label("WASD - Move");
            ui.label("Space - Jump");
            ui.label("↑ - Shoot/Aim");
            ui.label("Shift - Dodge");
            
            if let Some(stats) = game_stats {
                ui.separator();
                ui.label("Stats:");
                for (player_id, kills) in &stats.kills {
                    ui.label(format!("Player {}: {} kills", player_id, kills));
                }
            }
        });

    // ESC menu
    if contexts.ctx_mut().input(|i| i.key_pressed(egui::Key::Escape)) {
        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(200.0);
                ui.heading("Game Paused");
                ui.add_space(30.0);

                if ui.button("Resume").clicked() {
                    // Just close the menu, game continues
                }

                ui.add_space(10.0);

                if ui.button("Back to Lobby").clicked() {
                    game_state.set(GameState::Lobby);
                }

                if ui.button("Main Menu").clicked() {
                    game_state.set(GameState::MainMenu);
                }
            });
        });
    }
} 