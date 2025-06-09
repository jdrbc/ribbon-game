use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::*;
use crate::resources::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_ground_detection,
                handle_boundaries,
            ));
    }
}

pub fn update_ground_detection(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut collision_ended_reader: EventReader<CollisionEnded>,
    mut player_query: Query<(Entity, &mut Player)>,
    groundable_query: Query<Entity, With<Groundable>>,
    spatial_query: SpatialQuery,
    player_transform_query: Query<&Transform, With<Player>>,
) {
    // Handle collision started events
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        // Check if player is colliding with groundable surface
        if let Ok((player_entity, mut player)) = player_query.get_mut(*entity1) {
            if groundable_query.contains(*entity2) {
                player.is_grounded = true;
                player.can_jump = true;
            }
        } else if let Ok((player_entity, mut player)) = player_query.get_mut(*entity2) {
            if groundable_query.contains(*entity1) {
                player.is_grounded = true;
                player.can_jump = true;
            }
        }
    }

    // Handle collision ended events
    for CollisionEnded(entity1, entity2) in collision_ended_reader.read() {
        // Use raycast to double-check if player is still grounded
        if let Ok((player_entity, mut player)) = player_query.get_mut(*entity1) {
            if groundable_query.contains(*entity2) {
                // Perform downward raycast to verify ground contact
                if let Ok(transform) = player_transform_query.get(player_entity) {
                    let ray_origin = transform.translation;
                    let ray_direction = Dir3::NEG_Y;
                    let max_distance = 1.2; // Slightly more than player height
                    
                    if let Some(_hit) = spatial_query.cast_ray(
                        ray_origin,
                        ray_direction,
                        max_distance,
                        true,
                        &SpatialQueryFilter::default(),
                    ) {
                        // Still touching ground
                        player.is_grounded = true;
                        player.can_jump = true;
                    } else {
                        // No longer grounded
                        player.is_grounded = false;
                        player.can_jump = false;
                    }
                }
            }
        } else if let Ok((player_entity, mut player)) = player_query.get_mut(*entity2) {
            if groundable_query.contains(*entity1) {
                // Perform downward raycast to verify ground contact
                if let Ok(transform) = player_transform_query.get(player_entity) {
                    let ray_origin = transform.translation;
                    let ray_direction = Dir3::NEG_Y;
                    let max_distance = 1.2;
                    
                    if let Some(_hit) = spatial_query.cast_ray(
                        ray_origin,
                        ray_direction,
                        max_distance,
                        true,
                        &SpatialQueryFilter::default(),
                    ) {
                        player.is_grounded = true;
                        player.can_jump = true;
                    } else {
                        player.is_grounded = false;
                        player.can_jump = false;
                    }
                }
            }
        }
    }
}

pub fn handle_boundaries(
    mut player_query: Query<(&mut Transform, &mut LinearVelocity), With<Player>>,
    arena_config: Res<ArenaConfig>,
) {
    for (mut transform, mut velocity) in player_query.iter_mut() {
        let half_length = arena_config.length / 2.0;
        let half_width = arena_config.width / 2.0;

        // Check and clamp player position within arena boundaries
        if transform.translation.x > half_length {
            transform.translation.x = half_length;
            velocity.x = 0.0;
        } else if transform.translation.x < -half_length {
            transform.translation.x = -half_length;
            velocity.x = 0.0;
        }

        if transform.translation.z > half_width {
            transform.translation.z = half_width;
            velocity.z = 0.0;
        } else if transform.translation.z < -half_width {
            transform.translation.z = -half_width;
            velocity.z = 0.0;
        }

        // Prevent falling below ground level
        if transform.translation.y < 0.1 {
            transform.translation.y = 0.1;
            velocity.y = 0.0;
        }
    }
} 