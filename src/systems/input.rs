//! Input handling systems

use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseScrollUnit};

use crate::components::*;
use crate::resources::*;

/// Handle player movement input
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    // Only allow player movement when game is playing and not paused
    if !game_state.playing || game_state.paused {
        return;
    }
    for mut transform in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * 200.0 * time.delta_seconds();
        }
    }
}

/// Handle mouse interaction for entity selection and manipulation
pub fn mouse_interaction(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut drag_state: ResMut<DragState>,
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, Option<&Selected>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    editor_state: Res<EditorState>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_pos = editor_state.mouse_world_position;
        let mut closest_entity = None;
        let mut closest_distance = f32::INFINITY;
        
        // Find the closest entity to the mouse cursor
        for (entity, transform, _) in entity_query.iter() {
            let distance = transform.translation.truncate().distance(mouse_pos);
            let entity_size = transform.scale.x.max(transform.scale.y) * 0.5;
            
            if distance < entity_size && distance < closest_distance {
                closest_distance = distance;
                closest_entity = Some(entity);
            }
        }
        
        // Update selection
        if let Some(new_selected) = closest_entity {
            // Remove Selected component from previously selected entity
            if let Some(old_selected) = selected_entity.entity {
                commands.entity(old_selected).remove::<Selected>();
            }
            
            // Add Selected component to new entity
            commands.entity(new_selected).insert(Selected);
            selected_entity.entity = Some(new_selected);
            
            // Start dragging
            drag_state.dragging = true;
            if let Ok((_, transform, _)) = entity_query.get(new_selected) {
                drag_state.drag_offset = transform.translation.truncate() - mouse_pos;
            }
        } else {
            // Deselect if clicking on empty space
            if let Some(old_selected) = selected_entity.entity {
                commands.entity(old_selected).remove::<Selected>();
            }
            selected_entity.entity = None;
            drag_state.dragging = false;
        }
    }
    
    if mouse_input.just_released(MouseButton::Left) {
        drag_state.dragging = false;
    }
}

/// Handle entity dragging
pub fn entity_dragging(
    drag_state: Res<DragState>,
    selected_entity: Res<SelectedEntity>,
    mut entity_query: Query<&mut Transform>,
    editor_state: Res<EditorState>,
) {
    if drag_state.dragging {
        if let Some(entity) = selected_entity.entity {
            if let Ok(mut transform) = entity_query.get_mut(entity) {
                let new_position = editor_state.mouse_world_position + drag_state.drag_offset;
                transform.translation.x = new_position.x;
                transform.translation.y = new_position.y;
            }
        }
    }
}

/// Handle camera controls with right-click and drag
pub fn camera_controls(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut camera_controller: ResMut<CameraController>,
    editor_state: Res<EditorState>,
    mut last_mouse_pos: Local<Option<Vec2>>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        *last_mouse_pos = Some(editor_state.mouse_world_position);
    }
    
    if mouse_input.pressed(MouseButton::Right) {
        if let Some(last_pos) = *last_mouse_pos {
            let current_pos = editor_state.mouse_world_position;
            let delta = last_pos - current_pos; // Invert delta for natural camera movement
            
            // Move camera in the opposite direction of mouse movement for intuitive feel
            camera_controller.target_position += delta;
            camera_controller.instant_movement = true; // Enable instant movement during drag
            camera_controller.following_entity = None; // Stop following when manually controlling
            
            *last_mouse_pos = Some(current_pos);
        }
    }
    
    if mouse_input.just_released(MouseButton::Right) {
        *last_mouse_pos = None;
    }
}

/// Handle mouse wheel zoom
pub fn handle_mouse_wheel_zoom(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_controller: ResMut<CameraController>,
) {
    for event in scroll_events.read() {
        let zoom_delta = match event.unit {
            MouseScrollUnit::Line => event.y * 0.1,
            MouseScrollUnit::Pixel => event.y * 0.01,
        };
        
        // Update target zoom with clamping
        camera_controller.target_zoom = (camera_controller.target_zoom + zoom_delta).clamp(0.1, 5.0);
    }
}