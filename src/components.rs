use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: u32,
    pub movement_speed: f32,
    pub is_grounded: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: 0,
            movement_speed: 1.0, // 1 meter per second base movement
            is_grounded: false,
        }
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Arena;

#[derive(Component)]
pub struct Ground; 