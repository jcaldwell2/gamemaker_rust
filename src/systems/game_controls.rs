use bevy::prelude::*;
use crate::resources::{GameState, EditorSceneState};
use crate::components::*;
use crate::scene::{save_scene_to_string, load_scene_from_string};

pub fn game_controls_system(
    mut game_state: ResMut<GameState>,
    mut editor_scene_state: ResMut<EditorSceneState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    entity_query: Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Projectile>, Option<&Health>, Option<&Collision>, Option<&SpriteAsset>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
) {
    // Handle keyboard shortcuts
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        toggle_pause(&mut game_state);
    }
    
    if keyboard_input.just_pressed(KeyCode::F1) {
        game_state.debug_mode = !game_state.debug_mode;
        info!("Debug mode: {}", game_state.debug_mode);
    }
    
    // This logic is now handled in the UI when the play button is clicked
    // to avoid conflicts and flickering
}

fn toggle_pause(game_state: &mut GameState) {
    if game_state.playing {
        game_state.paused = !game_state.paused;
        info!("Game {}", if game_state.paused { "paused" } else { "resumed" });
    }
}

fn save_scene_state(
    editor_scene_state: &mut EditorSceneState,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Projectile>, Option<&Health>, Option<&Collision>, Option<&SpriteAsset>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
) {
    // Save the current scene state as RON string
    if let Ok(scene_data) = save_scene_to_string(entity_query) {
        editor_scene_state.saved_scene_data = Some(scene_data);
    } else {
        warn!("Failed to save scene state");
    }
}

pub fn handle_play_mode_transition(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut editor_scene_state: ResMut<EditorSceneState>,
    entity_query: Query<Entity, Or<(With<Player>, With<Enemy>, With<Projectile>)>>,
) {
    // Handle stopping play mode and restoring scene state
    if !game_state.playing && game_state.editor_mode {
        if let Some(saved_data) = &editor_scene_state.saved_scene_data {
            // Remove all current game entities
            for entity in entity_query.iter() {
                commands.entity(entity).despawn();
            }
            
            // Restore the saved scene
            if let Err(e) = load_scene_from_string(&mut commands, saved_data) {
                error!("Failed to restore scene: {}", e);
            } else {
                info!("Scene state restored from saved data");
            }
            
            // Clear saved data
            editor_scene_state.saved_scene_data = None;
        }
    }
}