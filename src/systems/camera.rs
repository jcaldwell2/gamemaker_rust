//! Camera control and positioning systems

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::*;
use crate::resources::*;

/// Handle camera movement and following
pub fn camera_movement(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut camera_controller: ResMut<CameraController>,
    entity_query: Query<&Transform, (Without<Camera>, With<Player>)>,
    time: Res<Time>,
) {
    for (mut camera_transform, mut projection) in camera_query.iter_mut() {
        // Handle entity following
        if let Some(following_entity) = camera_controller.following_entity {
            if let Ok(entity_transform) = entity_query.get(following_entity) {
                camera_controller.target_position = entity_transform.translation.truncate();
            } else {
                // Entity no longer exists, stop following
                camera_controller.following_entity = None;
            }
        }
        
        // Camera movement - instant during drag, smooth otherwise
        let current_position = camera_transform.translation.truncate();
        let target_position = camera_controller.target_position;
        
        if camera_controller.instant_movement {
            // Instant movement for dragging
            camera_transform.translation.x = target_position.x;
            camera_transform.translation.y = target_position.y;
            camera_controller.instant_movement = false; // Reset after use
        } else {
            // Smooth camera movement for normal operations
            let lerp_factor = 5.0 * time.delta_seconds();
            let new_position = current_position.lerp(target_position, lerp_factor);
            camera_transform.translation.x = new_position.x;
            camera_transform.translation.y = new_position.y;
        }
        
        // Smooth zoom
        let zoom_lerp_factor = 8.0 * time.delta_seconds();
        camera_controller.zoom = camera_controller.zoom.lerp(camera_controller.target_zoom, zoom_lerp_factor);
        projection.scale = camera_controller.zoom;
    }
}

/// Update mouse world position for editor interactions
pub fn update_mouse_world_position(
    mut editor_state: ResMut<EditorState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                    editor_state.mouse_world_position = world_position;
                }
            }
        }
    }
}