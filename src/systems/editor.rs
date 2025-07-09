//! Editor-specific systems for development tools

use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::scene::spawn_entity;

/// Handle editor-specific functionality
pub fn editor_update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut editor_state: ResMut<EditorState>,
    mut grid_settings: ResMut<GridSettings>,
    mut background_settings: ResMut<BackgroundSettings>,
) {
    // Toggle grid with G key
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        grid_settings.enabled = !grid_settings.enabled;
        editor_state.show_grid = grid_settings.enabled;
    }
    
    // Toggle background with B key
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        background_settings.enabled = !background_settings.enabled;
        editor_state.show_background = background_settings.enabled;
    }
    
    // Toggle inspector with Tab key
    if keyboard_input.just_pressed(KeyCode::Tab) {
        editor_state.show_inspector = !editor_state.show_inspector;
    }
    
    // Toggle hierarchy with H key
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        editor_state.show_hierarchy = !editor_state.show_hierarchy;
    }
}

/// Handle debug information display
pub fn debug_info_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    // Toggle debug mode with F1
    if keyboard_input.just_pressed(KeyCode::F1) {
        game_state.debug_mode = !game_state.debug_mode;
    }
    
    // Toggle pause with P
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        game_state.paused = !game_state.paused;
    }
}

/// Handle entity spawning from UI
pub fn entity_spawn_system(
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
) {
    if scene_manager.should_spawn {
        spawn_entity(
            &mut commands,
            scene_manager.spawn_entity_type,
            scene_manager.spawn_position,
            Some(scene_manager.spawn_z),
        );
        scene_manager.should_spawn = false;
    }
}