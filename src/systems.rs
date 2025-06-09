use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::{GameState};

const ARENA_LENGTH: f32 = 150.0;
const ARENA_WIDTH: f32 = 50.0;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ArenaConfig>()
            .init_resource::<GameStats>()
            .add_systems(OnEnter(GameState::InGame), (
                setup_scene,
                setup_camera,
                spawn_player,
            ))
            .add_systems(Update, (
                camera_follow.run_if(in_state(GameState::InGame)),
            ));
    }
}

// Startup Systems
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create ground plane (150m x 50m arena) with separate visual and physics
    let ground_parent = commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(ARENA_LENGTH, ARENA_WIDTH))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0), // Visual plane at Y=0
        Ground,
        Arena,
    )).id();
    
    // Add physics collider as child, positioned so top surface aligns with visual plane
    let collider_entity = commands.spawn((
        Transform::from_xyz(0.0, -0.5, 0.0), // Position collider so top is at Y=0
        RigidBody::Static,
        Collider::cuboid(ARENA_LENGTH, 1.0, ARENA_WIDTH), // Thicker collider
        Groundable,
    )).id();
    
    // Set parent relationship
    commands.entity(collider_entity).insert(ChildOf(ground_parent));

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
        Transform::from_xyz(0.0, 2.0, 0.0), // Start slightly above ground
        RigidBody::Dynamic,
        Collider::capsule(1.8, 0.4),
        LockedAxes::ROTATION_LOCKED, // Prevent player from falling over
        Player::default(),
        // Add friction and restitution for better physics
        Friction::new(1.0),
        Restitution::new(0.1),
        // Add external impulse component for movement
        ExternalImpulse::default(),
        // Network components
        NetworkInput::default(),
        LocalPlayer, // Mark as local player for now
    ));
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