use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::*;
use crate::{GameState};
use bevy_ggrs::ggrs::PlayerHandle;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                gather_input.run_if(in_state(GameState::SinglePlayer)),
                process_local_input.run_if(in_state(GameState::SinglePlayer)),
            ));
    }
}

// This function is responsible for collecting input for the local player
// in a single-player context. It's not used for networked games.
pub fn gather_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut local_input_query: Query<&mut NetworkInput, With<LocalPlayer>>,
) {
    for mut input in local_input_query.iter_mut() {
        // Gather movement input
        let mut movement = Vec2::ZERO;
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }

        // Normalize movement to prevent faster diagonal movement
        if movement.length() > 0.0 {
            movement = movement.normalize();
        }

        input.movement = movement;
        input.jump = keyboard_input.just_pressed(KeyCode::Space);
        input.shoot = keyboard_input.just_pressed(KeyCode::ArrowUp) || 
                     keyboard_input.pressed(KeyCode::ArrowUp);
    }
}

// This is for GGRS. It reads the local player's input and returns it
// for GGRS to handle. This is the real deal for multiplayer.
pub fn read_local_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    _handle: In<PlayerHandle>,
) -> NetworkInput {
    let mut movement = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    if movement.length() > 0.0 {
        movement = movement.normalize();
    }

    NetworkInput {
        movement,
        jump: keys.just_pressed(KeyCode::Space),
        shoot: keys.just_pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::ArrowUp),
    }
}

// This processes the local input for the single-player mode. It directly
// applies forces to the player character. In multiplayer, the server-side
// logic (or in our case, GGRS) would handle this.
pub fn process_local_input(
    input_query: Query<&NetworkInput, With<LocalPlayer>>,
    mut player_query: Query<(&mut Player, &mut ExternalImpulse, &Transform), With<LocalPlayer>>,
    time: Res<Time>,
) {
    for input in input_query.iter() {
        for (mut player, mut impulse, transform) in player_query.iter_mut() {
            let speed = player.movement_speed;

            // Apply horizontal movement using impulses (improved version)
            let movement_force = Vec3::new(
                input.movement.x * speed,
                0.0,
                -input.movement.y * speed, // Negative because forward is -Z
            );
            
            // Apply impulse only if there's movement input
            if movement_force.length() > 0.1 {
                impulse.apply_impulse(movement_force);
            }

            // Jump mechanics - only if grounded and can jump
            if input.jump && player.can_jump && player.is_grounded {
                let jump_impulse = Vec3::new(0.0, player.jump_force, 0.0);
                impulse.apply_impulse(jump_impulse);
                player.can_jump = false; // Prevent multiple jumps until landing
            }
        }
    }
} 