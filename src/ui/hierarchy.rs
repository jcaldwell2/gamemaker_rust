//! Hierarchy panel for entity management

use bevy::prelude::*;
use bevy_egui::egui;

use crate::components::*;
use crate::resources::*;

/// Render the hierarchy panel
pub fn render_hierarchy(
    ctx: &egui::Context,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    selected_entity: &mut SelectedEntity,
    commands: &mut Commands,
    editor_state: &mut EditorState,
    scene_manager: &mut SceneManager,
) {
    let mut render_content = |ui: &mut egui::Ui| {
        ui.separator();
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (entity, transform, player, enemy, health, collision) in entity_query.iter() {
                // Skip camera and other non-game entities
                if player.is_none() && enemy.is_none() {
                    continue;
                }
                
                let entity_name = if player.is_some() {
                    format!("Player ({})", entity.index())
                } else if enemy.is_some() {
                    format!("Enemy ({})", entity.index())
                } else {
                    format!("Entity ({})", entity.index())
                };
                
                let is_selected = selected_entity.entity == Some(entity);
                
                ui.horizontal(|ui| {
                    // Entity name button
                    let button = egui::Button::new(&entity_name)
                        .fill(if is_selected { 
                            egui::Color32::from_rgb(100, 100, 150) 
                        } else { 
                            egui::Color32::TRANSPARENT 
                        });
                    
                    if ui.add(button).clicked() {
                        selected_entity.entity = Some(entity);
                    }
                    
                    // Delete button
                    if ui.small_button("🗑").clicked() {
                        if selected_entity.entity == Some(entity) {
                            selected_entity.entity = None;
                        }
                        commands.entity(entity).despawn();
                    }
                });
                
                // Show entity info in a smaller font
                if is_selected {
                    ui.indent("entity_info", |ui| {
                        ui.small(format!("Pos: ({:.1}, {:.1}, {:.1})", 
                            transform.translation.x, 
                            transform.translation.y,
                            transform.translation.z
                        ));
                        
                        if let Some(health) = health {
                            ui.small(format!("Health: {:.1}/{:.1}", health.current, health.max));
                        }
                        
                        if let Some(collision) = collision {
                            ui.small(format!("Collision Radius: {:.1}", collision.radius));
                        }
                    });
                }
                
                ui.separator();
            }
        });
        
        ui.separator();
        
        // Entity creation section
        ui.collapsing("Create Entity", |ui| {
            ui.horizontal(|ui| {
                ui.label("Type:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", scene_manager.spawn_entity_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Player, "Player");
                        ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Enemy, "Enemy");
                    });
            });
            
            ui.horizontal(|ui| {
                if ui.button("Spawn at Origin").clicked() {
                    scene_manager.spawn_position = Vec2::ZERO;
                    scene_manager.should_spawn = true;
                }
                
                if ui.button("Spawn at Mouse").clicked() {
                    scene_manager.spawn_position = editor_state.mouse_world_position;
                    scene_manager.should_spawn = true;
                }
            });
        });
    };

    // This function is now only called from the unified system in editor.rs
    // Window management is handled there to avoid conflicts
}

/// Render hierarchy content without window management - for use by unified panel system
pub fn render_hierarchy_content(
    ui: &mut egui::Ui,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    selected_entity: &mut SelectedEntity,
    commands: &mut Commands,
    editor_state: &EditorState,
    scene_manager: &mut SceneManager,
) {
    ui.separator();
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        for (entity, transform, player, enemy, health, collision) in entity_query.iter() {
            // Skip camera and other non-game entities
            if player.is_none() && enemy.is_none() {
                continue;
            }
            
            let entity_name = if player.is_some() {
                format!("Player ({})", entity.index())
            } else if enemy.is_some() {
                format!("Enemy ({})", entity.index())
            } else {
                format!("Entity ({})", entity.index())
            };
            
            let is_selected = selected_entity.entity == Some(entity);
            
            ui.horizontal(|ui| {
                // Entity name button
                let button = egui::Button::new(&entity_name)
                    .fill(if is_selected {
                        egui::Color32::from_rgb(100, 100, 150)
                    } else {
                        egui::Color32::TRANSPARENT
                    });
                
                if ui.add(button).clicked() {
                    selected_entity.entity = Some(entity);
                }
                
                // Delete button
                if ui.small_button("🗑").clicked() {
                    if selected_entity.entity == Some(entity) {
                        selected_entity.entity = None;
                    }
                    commands.entity(entity).despawn();
                }
            });
            
            // Show entity info in a smaller font
            if is_selected {
                ui.indent("entity_info", |ui| {
                    ui.small(format!("Pos: ({:.1}, {:.1}, {:.1})",
                        transform.translation.x,
                        transform.translation.y,
                        transform.translation.z
                    ));
                    
                    if let Some(health) = health {
                        ui.small(format!("Health: {:.1}/{:.1}", health.current, health.max));
                    }
                    
                    if let Some(collision) = collision {
                        ui.small(format!("Collision Radius: {:.1}", collision.radius));
                    }
                });
            }
            
            ui.separator();
        }
    });
    
    ui.separator();
    
    // Entity creation section
    ui.collapsing("Create Entity", |ui| {
        ui.horizontal(|ui| {
            ui.label("Type:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", scene_manager.spawn_entity_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Player, "Player");
                    ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Enemy, "Enemy");
                });
        });
        
        ui.horizontal(|ui| {
            if ui.button("Spawn at Origin").clicked() {
                scene_manager.spawn_position = Vec2::ZERO;
                scene_manager.should_spawn = true;
            }
            
            if ui.button("Spawn at Mouse").clicked() {
                scene_manager.spawn_position = editor_state.mouse_world_position;
                scene_manager.should_spawn = true;
            }
        });
    });
}