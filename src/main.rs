use bevy::prelude::*;
use avian3d::prelude::*;
use ribbon_game::RibbonGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ribbon Game".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(RibbonGamePlugin)
        .run();
} 