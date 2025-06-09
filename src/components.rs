use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Player {
    pub id: u32,
    pub network_id: u32, // Simplified for now
    pub movement_speed: f32,
    pub is_grounded: bool,
    pub jump_force: f32,
    pub can_jump: bool,
    pub health: i32,
    pub is_local: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: 0,
            network_id: 0,
            movement_speed: 5.0, // 5 meter per second base movement
            is_grounded: false,
            jump_force: 8.0, // Hop-style jump force
            can_jump: true,
            health: 1, // Instant death system
            is_local: true,
        }
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Arena;

#[derive(Component)]
pub struct Ground;

// Component to mark entities that can be landed on
#[derive(Component)]
pub struct Groundable;

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct RemotePlayer;

// Network input component for GGRS
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct NetworkInput {
    pub movement: Vec2,
    pub jump: bool,
    pub shoot: bool,
}

impl Default for NetworkInput {
    fn default() -> Self {
        Self {
            movement: Vec2::ZERO,
            jump: false,
            shoot: false,
        }
    }
}

// Bow and combat components
#[derive(Component)]
pub struct Bow {
    pub reload_timer: f32,
    pub can_shoot: bool,
    pub power_charge: f32,
}

impl Default for Bow {
    fn default() -> Self {
        Self {
            reload_timer: 0.0,
            can_shoot: true,
            power_charge: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Arrow {
    pub damage: i32,
    pub lifetime: f32,
    pub shooter_id: u32,
}

#[derive(Component)]
pub struct DodgeAbility {
    pub cooldown_timer: f32,
    pub can_dodge: bool,
    pub invincible_timer: f32,
    pub is_invincible: bool,
}

impl Default for DodgeAbility {
    fn default() -> Self {
        Self {
            cooldown_timer: 0.0,
            can_dodge: true,
            invincible_timer: 0.0,
            is_invincible: false,
        }
    }
} 