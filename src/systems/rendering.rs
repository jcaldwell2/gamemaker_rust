//! Rendering systems for visual effects and overlays

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::*;
use crate::resources::*;

/// Render grid overlay
pub fn render_grid_overlay(
    mut commands: Commands,
    grid_settings: Res<GridSettings>,
    mut grid_state: ResMut<GridState>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    grid_line_query: Query<Entity, With<GridLine>>,
) {
    if !grid_settings.enabled {
        // Remove all grid lines if grid is disabled
        for entity in grid_line_query.iter() {
            commands.entity(entity).despawn();
        }
        // Reset grid state when disabled so it will regenerate when re-enabled
        grid_state.last_settings_hash = 0;
        return;
    }
    
    if let Ok((camera_transform, projection)) = camera_query.get_single() {
        if let Ok(window) = window_query.get_single() {
            let camera_pos = camera_transform.translation.truncate();
            let zoom = projection.scale;
            
            // Check if we need to update the grid
            let needs_update = grid_state.needs_update(camera_pos, zoom, &grid_settings);
            let has_no_grid_lines = grid_line_query.is_empty();
            
            // Force update if grid is enabled but no lines exist
            if needs_update || has_no_grid_lines {
                // Remove existing grid lines
                for entity in grid_line_query.iter() {
                    commands.entity(entity).despawn();
                }
                
                // Calculate visible area
                let window_size = Vec2::new(window.width(), window.height());
                let visible_size = window_size * zoom;
                let half_visible = visible_size * 0.5;
                
                let min_x = camera_pos.x - half_visible.x;
                let max_x = camera_pos.x + half_visible.x;
                let min_y = camera_pos.y - half_visible.y;
                let max_y = camera_pos.y + half_visible.y;
                
                let spacing = grid_settings.spacing;
                let color = grid_settings.color.with_a(grid_settings.opacity);
                
                // Create vertical lines
                let start_x = (min_x / spacing).floor() * spacing;
                let mut x = start_x;
                while x <= max_x {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color,
                                ..default()
                            },
                            transform: Transform::from_xyz(x, camera_pos.y, 100.0)
                                .with_scale(Vec3::new(grid_settings.thickness, visible_size.y, 1.0)),
                            ..default()
                        },
                        GridLine,
                    ));
                    x += spacing;
                }
                
                // Create horizontal lines
                let start_y = (min_y / spacing).floor() * spacing;
                let mut y = start_y;
                while y <= max_y {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color,
                                ..default()
                            },
                            transform: Transform::from_xyz(camera_pos.x, y, 100.0)
                                .with_scale(Vec3::new(visible_size.x, grid_settings.thickness, 1.0)),
                            ..default()
                        },
                        GridLine,
                    ));
                    y += spacing;
                }
            }
        }
    }
}

/// Update background image visibility and properties
pub fn update_background_image(
    background_settings: Res<BackgroundSettings>,
    mut background_query: Query<(&mut Sprite, &mut Transform, &mut Visibility), With<BackgroundImage>>,
) {
    for (mut sprite, mut transform, mut visibility) in background_query.iter_mut() {
        if background_settings.enabled && background_settings.image_path.is_some() {
            *visibility = Visibility::Visible;
            sprite.color = sprite.color.with_a(background_settings.opacity);
            transform.scale = Vec3::splat(background_settings.scale);
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// Update visual indicators for selected entities
pub fn update_selection_visuals(
    mut selected_query: Query<&mut Sprite, (With<Selected>, Without<GridLine>, Without<BackgroundImage>)>,
    mut unselected_query: Query<&mut Sprite, (Without<Selected>, Without<GridLine>, Without<BackgroundImage>, Or<(With<Player>, With<Enemy>)>)>,
) {
    // Highlight selected entities
    for mut sprite in selected_query.iter_mut() {
        // Add a slight brightness to selected entities
        let current_color = sprite.color;
        sprite.color = Color::rgb(
            (current_color.r() + 0.2).min(1.0),
            (current_color.g() + 0.2).min(1.0),
            (current_color.b() + 0.2).min(1.0),
        );
    }
    
    // Reset unselected entities to normal colors
    for mut sprite in unselected_query.iter_mut() {
        // This would need to store original colors to properly reset
        // For now, we'll just ensure they're not overly bright
        let current_color = sprite.color;
        if current_color.r() > 0.8 && current_color.g() > 0.8 && current_color.b() > 0.8 {
            sprite.color = Color::rgb(
                (current_color.r() - 0.2).max(0.0),
                (current_color.g() - 0.2).max(0.0),
                (current_color.b() - 0.2).max(0.0),
            );
        }
    }
}