use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::*;
use crate::resources::*;

const ARENA_LENGTH: f32 = 15.0;
const ARENA_WIDTH: f32 = 5.0;
const JUMP_IMPULSE: f32 = 8.0;

// Startup Systems
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add arena config resource
    commands.insert_resource(ArenaConfig::default());

    // Create ground plane (15m x 5m arena)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(ARENA_LENGTH, ARENA_WIDTH))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(ARENA_LENGTH, 0.1, ARENA_WIDTH),
        Ground,
        Arena,
    ));

    // Add lighting
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
    ));

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
        affects_lightmapped_meshes: false,
    });
}

pub fn setup_camera(mut commands: Commands) {
    // Create 3rd person camera behind player position
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        MainCamera,
    ));
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn player as 3D capsule
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.8))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RigidBody::Dynamic,
        Collider::capsule(1.8, 0.4),
        LockedAxes::ROTATION_LOCKED, // Prevent player from falling over
        Player::default(),
    ));
}

// Update Systems
pub fn update_ground_detection(
    mut player_query: Query<(&mut Player, &Transform, &LinearVelocity)>,
) {
    for (mut player, transform, velocity) in player_query.iter_mut() {
        // Simple ground detection - if player is close to ground and not moving up
        player.is_grounded = transform.translation.y <= 1.1 && velocity.y <= 0.1;
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut LinearVelocity, &Transform), With<Player>>,
    time: Res<Time>,
) {
    for (player, mut velocity, transform) in player_query.iter_mut() {
        let mut movement = Vec3::ZERO;
        let speed = player.movement_speed;

        // WASD movement
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.z += 1.0;
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

        // Apply movement to velocity (preserve Y for physics)
        velocity.x = movement.x * speed;
        velocity.z = movement.z * speed;

        // Jump mechanics (Spacebar) - only if grounded
        if keyboard_input.just_pressed(KeyCode::Space) && player.is_grounded {
            velocity.y = JUMP_IMPULSE;
        }
    }
}

pub fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            // Calculate desired camera position (5 units behind and above player)
            let desired_pos = player_transform.translation + Vec3::new(0.0, 5.0, 10.0);
            
            // Smooth camera movement
            let camera_speed = 2.0;
            camera_transform.translation = camera_transform.translation.lerp(
                desired_pos, 
                camera_speed * time.delta_secs()
            );

            // Always look at player position (slightly above for better view)
            let look_target = player_transform.translation + Vec3::new(0.0, 1.0, 0.0);
            camera_transform.look_at(look_target, Vec3::Y);
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
        if transform.translation.y < 0.5 {
            transform.translation.y = 0.5;
            velocity.y = 0.0;
        }
    }
} 