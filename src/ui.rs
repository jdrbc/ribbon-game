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
            .init_resource::<UITheme>()
            .add_systems(EguiContextPass, (
                main_menu_ui.run_if(in_state(GameState::MainMenu)),
                lobby_ui.run_if(in_state(GameState::Lobby)),
                in_game_ui.run_if(in_state(GameState::InGame)),
            ));
    }
}

#[derive(Resource)]
pub struct UITheme {
    pub primary_color: egui::Color32,
    pub secondary_color: egui::Color32,
    pub accent_color: egui::Color32,
    pub background_color: egui::Color32,
    pub text_color: egui::Color32,
    pub success_color: egui::Color32,
    pub warning_color: egui::Color32,
    pub error_color: egui::Color32,
}

impl Default for UITheme {
    fn default() -> Self {
        Self {
            primary_color: egui::Color32::from_rgb(64, 128, 255),
            secondary_color: egui::Color32::from_rgb(128, 128, 128),
            accent_color: egui::Color32::from_rgb(255, 165, 0),
            background_color: egui::Color32::from_rgb(32, 32, 40),
            text_color: egui::Color32::from_rgb(240, 240, 240),
            success_color: egui::Color32::from_rgb(76, 175, 80),
            warning_color: egui::Color32::from_rgb(255, 193, 7),
            error_color: egui::Color32::from_rgb(244, 67, 54),
        }
    }
}

pub fn main_menu_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    mut lobby_state: ResMut<LobbyState>,
    mut connection_info: ResMut<ConnectionInfo>,
    theme: Res<UITheme>,
) {
    // Apply dark theme
    apply_dark_theme(contexts.ctx_mut(), &theme);

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(theme.background_color))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(80.0);
                
                // Title with gradient effect
                ui.scope(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                    ui.label(
                        egui::RichText::new(">> RIBBON GAME <<")
                            .size(48.0)
                            .color(theme.primary_color)
                    );
                    
                    // Add subtitle
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("Multiplayer Arena Combat")
                            .size(16.0)
                            .color(theme.secondary_color)
                    );
                });

                ui.add_space(60.0);

                // Main menu buttons with styling
                ui.vertical_centered_justified(|ui| {
                    ui.set_width(300.0);

                    // Player name input
                    ui.group(|ui| {
                        ui.set_width(280.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("Player Settings")
                                    .size(14.0)
                                    .color(theme.text_color)
                            );
                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                ui.label("Your Name:");
                                if lobby_state.local_player_name.is_empty() {
                                    lobby_state.local_player_name = "Player".to_string();
                                }
                                ui.add(
                                    egui::TextEdit::singleline(&mut lobby_state.local_player_name)
                                        .desired_width(120.0)
                                        .hint_text("Enter name...")
                                );
                            });
                            ui.add_space(10.0);
                        });
                    });

                    ui.add_space(15.0);

                    // Create Game button
                    if create_styled_button(ui, "▶ Create Game", &theme, true).clicked() {
                        let room_id = generate_room_code();
                        let player_name = lobby_state.local_player_name.clone();
                        lobby_state.room_id = Some(room_id.clone());
                        lobby_state.is_host = true;
                        lobby_state.max_players = 4;
                        lobby_state.player_count = 1;
                        lobby_state.players.clear();
                        lobby_state.players.push(LobbyPlayer {
                            id: Uuid::new_v4(),
                            name: player_name,
                            is_ready: false,
                            network_handle: 0,
                            is_local: true,
                        });
                        lobby_state.chat_messages.clear();
                        add_system_message(&mut lobby_state, "Lobby created!");
                        connection_info.room_id = room_id;
                        game_state.set(GameState::Lobby);
                    }

                    ui.add_space(15.0);

                    // Join Game section with enhanced styling
                    ui.group(|ui| {
                        ui.set_width(280.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("Join Existing Game")
                                    .size(14.0)
                                    .color(theme.text_color)
                            );
                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                ui.label("Room Code:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut connection_info.room_id)
                                        .desired_width(100.0)
                                        .hint_text("Enter code...")
                                );
                            });

                            ui.add_space(10.0);

                            let join_enabled = !connection_info.room_id.is_empty() 
                                && connection_info.room_id.len() >= 4;
                            
                            if create_styled_button(ui, "→ Join Game", &theme, join_enabled).clicked() 
                                && join_enabled {
                                let player_name = lobby_state.local_player_name.clone();
                                let room_code = connection_info.room_id.clone();
                                lobby_state.room_id = Some(connection_info.room_id.clone());
                                lobby_state.is_host = false;
                                lobby_state.max_players = 4;
                                lobby_state.player_count = 2; // Assume host is already there
                                lobby_state.players.clear();
                                lobby_state.players.push(LobbyPlayer {
                                    id: Uuid::new_v4(),
                                    name: "Host".to_string(),
                                    is_ready: false,
                                    network_handle: 0,
                                    is_local: false,
                                });
                                lobby_state.players.push(LobbyPlayer {
                                    id: Uuid::new_v4(),
                                    name: player_name,
                                    is_ready: false,
                                    network_handle: 1,
                                    is_local: true,
                                });
                                lobby_state.chat_messages.clear();
                                add_system_message(&mut lobby_state, &format!("Joined room: {}", room_code));
                                game_state.set(GameState::Lobby);
                            }
                            ui.add_space(10.0);
                        });
                    });

                    ui.add_space(20.0);

                    // Single Player (for development/testing)
                    if create_styled_button(ui, "● Single Player (Dev)", &theme, true).clicked() {
                        game_state.set(GameState::InGame);
                    }

                    ui.add_space(30.0);

                    // Display connection errors with styling
                    if let Some(error) = &connection_info.connection_error {
                        ui.colored_label(theme.error_color, format!("⚠️ Error: {}", error));
                    }
                });

                // Footer with game info
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("Built with Bevy & Avian3D")
                            .size(12.0)
                            .color(theme.secondary_color)
                    );
                });
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
}

pub fn lobby_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    mut lobby_state: ResMut<LobbyState>,
    connection_info: Res<ConnectionInfo>,
    theme: Res<UITheme>,
) {
    apply_dark_theme(contexts.ctx_mut(), &theme);

    // Top bar with lobby title and leave button
    egui::TopBottomPanel::top("lobby_top_bar")
        .resizable(false)
        .min_height(50.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.add_space(15.0);
                ui.label(
                    egui::RichText::new(">> RIBBON GAME LOBBY <<")
                        .size(20.0)
                        .color(theme.primary_color)
                        .strong()
                );
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(15.0);
                    if ui.add_sized([80.0, 30.0], egui::Button::new("× Leave")).clicked() {
                        leave_lobby(&mut lobby_state, &mut game_state);
                    }
                });
            });
        });

    // Left panel - Players
    egui::SidePanel::left("players_panel")
        .resizable(true)
        .default_width(320.0)
        .min_width(280.0)
        .max_width(400.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Players")
                        .size(18.0)
                        .color(theme.text_color)
                        .strong()
                );
                ui.separator();
                ui.add_space(8.0);

                // Player list in a scroll area
                egui::ScrollArea::vertical()
                    .id_salt("players_scroll")
                    .show(ui, |ui| {
                        let players_to_display: Vec<_> = lobby_state.players.iter().enumerate().collect();
                        let mut player_to_kick = None;
                        
                        for (i, player) in players_to_display {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    // Player avatar/icon
                                    let (avatar, avatar_color) = if player.is_local {
                                        ("★", theme.accent_color) // Local player
                                    } else if i == 0 && lobby_state.is_host { 
                                        ("♔", theme.warning_color) // Host
                                    } else { 
                                        ("●", theme.secondary_color) // Remote player
                                    };
                                    ui.label(egui::RichText::new(avatar).size(18.0).color(avatar_color));
                                    
                                    ui.vertical(|ui| {
                                        // Player name
                                        let name_text = if player.is_local {
                                            format!("{} (You)", player.name)
                                        } else {
                                            player.name.clone()
                                        };
                                        ui.label(
                                            egui::RichText::new(name_text)
                                                .size(15.0)
                                                .color(if player.is_local { theme.accent_color } else { theme.text_color })
                                        );
                                        
                                        // Ready status and connection
                                        ui.horizontal(|ui| {
                                            let (ready_text, ready_color) = if player.is_ready {
                                                ("✓ Ready", theme.success_color)
                                            } else {
                                                ("○ Not Ready", theme.secondary_color)
                                            };
                                            ui.label(egui::RichText::new(ready_text).size(12.0).color(ready_color));
                                            
                                            // Connection indicator
                                            let ping_color = match i % 3 {
                                                0 => theme.success_color,
                                                1 => theme.warning_color,
                                                _ => theme.error_color,
                                            };
                                            ui.label(egui::RichText::new("●").size(10.0).color(ping_color));
                                        });
                                    });
                                    
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        // Kick button for host
                                        if lobby_state.is_host && !player.is_local && i != 0 {
                                            if ui.small_button("×").on_hover_text("Kick player").clicked() {
                                                player_to_kick = Some(i);
                                            }
                                        }
                                    });
                                });
                            });
                            ui.add_space(5.0);
                        }
                        
                        // Handle kick after the loop
                        if let Some(index) = player_to_kick {
                            kick_player(&mut lobby_state, index);
                        }

                        // Empty slots
                        for _i in lobby_state.players.len()..lobby_state.max_players {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("○").size(18.0).color(theme.secondary_color));
                                    ui.label(
                                        egui::RichText::new("Waiting for player...")
                                            .size(13.0)
                                            .color(theme.secondary_color)
                                            .italics()
                                    );
                                });
                            });
                            ui.add_space(5.0);
                        }
                    });
            });
        });

    // Bottom right - Chat
    egui::Window::new("💬 Chat")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-15.0, -15.0))
        .default_size(egui::vec2(380.0, 280.0))
        .resizable(true)
        .collapsible(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                // Chat messages area
                egui::ScrollArea::vertical()
                    .id_salt("chat_scroll")
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for message in &lobby_state.chat_messages {
                            let color = match message.message_type {
                                ChatMessageType::System => theme.accent_color,
                                ChatMessageType::Info => theme.success_color,
                                ChatMessageType::Player => theme.text_color,
                            };
                            
                            let prefix = match message.message_type {
                                ChatMessageType::System => "⚙",
                                ChatMessageType::Info => "ⓘ",
                                ChatMessageType::Player => "",
                            };
                            
                            ui.horizontal_wrapped(|ui| {
                                if !prefix.is_empty() {
                                    ui.label(egui::RichText::new(prefix).color(color).size(12.0));
                                }
                                if message.message_type == ChatMessageType::Player {
                                    ui.label(egui::RichText::new(&message.sender).color(theme.primary_color).strong().size(13.0));
                                    ui.label(egui::RichText::new(":").size(13.0));
                                }
                                ui.label(egui::RichText::new(&message.message).color(color).size(13.0));
                            });
                        }
                    });

                ui.separator();

                // Chat input
                ui.horizontal(|ui| {
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut lobby_state.current_chat_input)
                            .desired_width(ui.available_width() - 60.0)
                            .hint_text("Type message...")
                    );
                    
                    let send_enabled = !lobby_state.current_chat_input.trim().is_empty();
                    if ui.add_enabled(send_enabled, egui::Button::new("→")).clicked() 
                        || (response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                        if send_enabled {
                            send_chat_message(&mut lobby_state);
                        }
                    }
                });
            });
        });

    // Top right - Room Info
    egui::Window::new("🔑 Room Info")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-15.0, 15.0))
        .default_size(egui::vec2(300.0, 140.0))
        .resizable(false)
        .collapsible(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                // Room code
                if let Some(room_id) = &lobby_state.room_id {
                    ui.horizontal(|ui| {
                        ui.label("Room Code:");
                        ui.label(
                            egui::RichText::new(room_id)
                                .color(theme.accent_color)
                                .strong()
                                .size(16.0)
                        );
                    });
                }
                
                // Player count
                ui.horizontal(|ui| {
                    ui.label("Players:");
                    ui.label(
                        egui::RichText::new(format!("{}/{}", lobby_state.player_count, lobby_state.max_players))
                            .color(theme.text_color)
                            .size(14.0)
                    );
                });
                
                // Connection status
                let (status_text, status_color) = if connection_info.is_connecting {
                    ("Connecting...", theme.warning_color)
                } else if connection_info.connection_error.is_some() {
                    ("Connection Error", theme.error_color)
                } else {
                    ("Connected", theme.success_color)
                };
                
                ui.horizontal(|ui| {
                    ui.label("Status:");
                    ui.label(egui::RichText::new(status_text).color(status_color));
                });
            });
        });

    // Center-left - Game Settings (Host only)
    if lobby_state.is_host {
        egui::Window::new("⚙ Game Settings")
            .anchor(egui::Align2::LEFT_CENTER, egui::vec2(340.0, -50.0))
            .default_size(egui::vec2(280.0, 180.0))
            .resizable(false)
            .collapsible(true)
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Max Players:");
                        egui::ComboBox::from_id_salt("max_players")
                            .selected_text(format!("{}", lobby_state.max_players))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut lobby_state.max_players, 2, "2");
                                ui.selectable_value(&mut lobby_state.max_players, 3, "3");
                                ui.selectable_value(&mut lobby_state.max_players, 4, "4");
                            });
                    });

                    ui.add_space(15.0);

                    // Dev buttons
                    ui.label(egui::RichText::new("Dev Tools:").size(12.0).color(theme.secondary_color));
                    ui.horizontal(|ui| {
                        if ui.button("+ Add Bot").clicked() && lobby_state.players.len() < lobby_state.max_players {
                            add_bot_player(&mut lobby_state);
                        }
                        
                        if ui.button("Fill Lobby").clicked() {
                            fill_lobby_with_bots(&mut lobby_state);
                        }
                    });
                });
            });
    }

    // Bottom center - Control Buttons
    egui::TopBottomPanel::bottom("lobby_controls")
        .resizable(false)
        .min_height(70.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(15.0);
                
                // Ready button for all players
                let local_player = lobby_state.players.iter().find(|p| p.is_local);
                
                if let Some(player) = local_player {
                    let ready_text = if player.is_ready { "✓ Ready!" } else { "Ready Up" };
                    let ready_color = if player.is_ready { theme.success_color } else { theme.primary_color };
                    if ui.add_sized([130.0, 40.0], egui::Button::new(egui::RichText::new(ready_text).color(egui::Color32::WHITE).size(16.0)).fill(ready_color)).clicked() {
                        toggle_ready_status(&mut lobby_state);
                    }
                }

                ui.add_space(30.0);

                // Host controls
                if lobby_state.is_host {
                    let can_start = lobby_state.player_count >= 2 && all_players_ready(&lobby_state.players);
                    
                    if ui.add_sized([180.0, 40.0], egui::Button::new(egui::RichText::new("▶ Start Game").color(egui::Color32::WHITE).size(16.0)).fill(if can_start { theme.success_color } else { theme.secondary_color })).clicked() && can_start {
                        lobby_state.game_started = true;
                        add_system_message(&mut lobby_state, "Game starting!");
                        game_state.set(GameState::InGame);
                    }
                    
                    if !can_start {
                        ui.add_space(15.0);
                        let reason = if lobby_state.player_count < 2 {
                            "Need at least 2 players"
                        } else {
                            "All players must be ready"
                        };
                        ui.label(egui::RichText::new(reason).color(theme.secondary_color).size(12.0));
                    }
                } else {
                    ui.label(
                        egui::RichText::new("Waiting for host to start...")
                            .color(theme.secondary_color)
                            .size(14.0)
                    );
                }
            });
        });
}

pub fn in_game_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    game_stats: Option<Res<GameStats>>,
    lobby_state: Res<LobbyState>,
    theme: Res<UITheme>,
) {
    // Minimal in-game HUD with modern styling
    egui::Window::new("Game HUD")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
        .resizable(false)
        .title_bar(false)
        .frame(egui::Frame::window(&contexts.ctx_mut().style()).fill(theme.background_color.gamma_multiply(0.8)))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("🎮 Controls").color(theme.primary_color).size(14.0));
                ui.label("WASD - Move");
                ui.label("Space - Jump");
                ui.label("↑ - Shoot/Aim");
                ui.label("Shift - Dodge");
                
                if let Some(stats) = game_stats {
                    ui.separator();
                    ui.label(egui::RichText::new("📊 Stats").color(theme.primary_color).size(14.0));
                    for (player_id, kills) in &stats.kills {
                        ui.label(format!("Player {}: {} kills", player_id, kills));
                    }
                }
            });
        });

    // Enhanced pause menu
    if contexts.ctx_mut().input(|i| i.key_pressed(egui::Key::Escape)) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(theme.background_color.gamma_multiply(0.9)))
            .show(contexts.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(200.0);
                    ui.label(egui::RichText::new("⏸️ Game Paused").size(32.0).color(theme.primary_color));
                    ui.add_space(30.0);

                    if create_styled_button(ui, "▶️ Resume", &theme, true).clicked() {
                        // Just close the menu, game continues
                    }

                    ui.add_space(10.0);

                    if create_styled_button(ui, "🏛️ Back to Lobby", &theme, true).clicked() {
                        game_state.set(GameState::Lobby);
                    }

                    if create_styled_button(ui, "🏠 Main Menu", &theme, true).clicked() {
                        game_state.set(GameState::MainMenu);
                    }
                });
            });
    }
}

// Helper functions
fn apply_dark_theme(ctx: &mut egui::Context, theme: &UITheme) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = theme.background_color;
    visuals.panel_fill = theme.background_color;
    visuals.faint_bg_color = theme.background_color.gamma_multiply(1.2);
    visuals.extreme_bg_color = theme.background_color.gamma_multiply(0.5);
    visuals.selection.bg_fill = theme.primary_color.gamma_multiply(0.3);
    ctx.set_visuals(visuals);
}

fn create_styled_button(ui: &mut egui::Ui, text: &str, theme: &UITheme, enabled: bool) -> egui::Response {
    let button_color = if enabled { theme.primary_color } else { theme.secondary_color };
    let text_color = if enabled { egui::Color32::WHITE } else { theme.secondary_color };
    
    ui.add_sized(
        [ui.available_width(), 35.0],
        egui::Button::new(egui::RichText::new(text).color(text_color).size(14.0))
            .fill(button_color)
            .rounding(egui::Rounding::same(6))
    )
}

fn generate_room_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    (0..6).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}

fn leave_lobby(lobby_state: &mut LobbyState, game_state: &mut NextState<GameState>) {
    lobby_state.room_id = None;
    lobby_state.players.clear();
    lobby_state.player_count = 0;
    lobby_state.is_host = false;
    lobby_state.game_started = false;
    game_state.set(GameState::MainMenu);
}

fn all_players_ready(players: &[LobbyPlayer]) -> bool {
    !players.is_empty() && players.iter().all(|p| p.is_ready)
}

fn add_system_message(lobby_state: &mut LobbyState, message: &str) {
    use crate::resources::{ChatMessage, ChatMessageType};
    
    lobby_state.chat_messages.push(ChatMessage {
        sender: "System".to_string(),
        message: message.to_string(),
        timestamp: 0.0, // Would use actual time in production
        message_type: ChatMessageType::System,
    });
}

fn send_chat_message(lobby_state: &mut LobbyState) {
    use crate::resources::{ChatMessage, ChatMessageType};
    
    if let Some(local_player) = lobby_state.players.iter().find(|p| p.is_local) {
        let message = lobby_state.current_chat_input.trim().to_string();
        if !message.is_empty() {
            lobby_state.chat_messages.push(ChatMessage {
                sender: local_player.name.clone(),
                message,
                timestamp: 0.0, // Would use actual time in production
                message_type: ChatMessageType::Player,
            });
            lobby_state.current_chat_input.clear();
        }
    }
}

fn toggle_ready_status(lobby_state: &mut LobbyState) {
    let mut player_name = String::new();
    let mut new_status = false;
    
    if let Some(player) = lobby_state.players.iter_mut().find(|p| p.is_local) {
        player.is_ready = !player.is_ready;
        new_status = player.is_ready;
        player_name = player.name.clone();
    }
    
    if !player_name.is_empty() {
        let status = if new_status { "ready" } else { "not ready" };
        add_system_message(lobby_state, &format!("{} is now {}", player_name, status));
    }
}

fn kick_player(lobby_state: &mut LobbyState, player_index: usize) {
    if player_index < lobby_state.players.len() {
        let kicked_player = lobby_state.players.remove(player_index);
        lobby_state.player_count = lobby_state.player_count.saturating_sub(1);
        add_system_message(lobby_state, &format!("{} was kicked from the lobby", kicked_player.name));
    }
}

fn add_bot_player(lobby_state: &mut LobbyState) {
    use crate::resources::LobbyPlayer;
    
    let bot_names = ["Archer", "Hunter", "Ranger", "Scout", "Sniper", "Robin", "Legolas"];
    let next_id = lobby_state.players.len();
    let bot_name = format!("{} (Bot)", bot_names[next_id % bot_names.len()]);
    
    lobby_state.players.push(LobbyPlayer {
        id: Uuid::new_v4(),
        name: bot_name.clone(),
        is_ready: rand::random(), // Randomly ready or not
        network_handle: next_id as u32,
        is_local: false,
    });
    
    lobby_state.player_count += 1;
    add_system_message(lobby_state, &format!("{} joined the lobby", bot_name));
}

fn fill_lobby_with_bots(lobby_state: &mut LobbyState) {
    while lobby_state.players.len() < lobby_state.max_players {
        add_bot_player(lobby_state);
    }
} 