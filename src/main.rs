//! GameMaker Rust - Main application entry point

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use gamemaker_rust::GameEnginePlugin;

fn main() {
    App::new()
        // Add Bevy default plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GameMaker Rust - 2D Game Engine".into(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        
        // Add egui plugin for UI
        .add_plugins(EguiPlugin)
        
        // Add our game engine plugin
        .add_plugins(GameEnginePlugin)
        
        // Run the app
        .run();
}