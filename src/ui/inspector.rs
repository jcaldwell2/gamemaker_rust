//! Inspector panel for entity properties

use bevy::prelude::*;
use bevy::math::EulerRot;
use bevy_egui::egui;

use crate::components::*;
use crate::resources::*;

/// Render the inspector panel
pub fn render_inspector(
    ctx: &egui::Context,
    selected_entity: &SelectedEntity,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    editor_state: &mut EditorState,
) {
    let render_content = |ui: &mut egui::Ui| {
        if let Some(entity) = selected_entity.entity {
            if let Ok((_, transform, player, enemy, health, collision)) = entity_query.get(entity) {
                ui.separator();
                
                // Entity ID and type
                ui.horizontal(|ui| {
                    ui.label("Entity ID:");
                    ui.label(format!("{}", entity.index()));
                });
                
                if player.is_some() {
                    ui.label("Type: Player");
                } else if enemy.is_some() {
                    ui.label("Type: Enemy");
                } else {
                    ui.label("Type: Unknown");
                }
                
                ui.separator();
                
                // Transform section (read-only for now to avoid query conflicts)
                ui.collapsing("Transform", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        ui.label(format!("({:.2}, {:.2}, {:.2})",
                            transform.translation.x,
                            transform.translation.y,
                            transform.translation.z
                        ));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Scale:");
                        ui.label(format!("({:.2}, {:.2})",
                            transform.scale.x,
                            transform.scale.y
                        ));
                    });
                    
                    let rotation_degrees = transform.rotation.to_euler(EulerRot::ZYX).0.to_degrees();
                    ui.horizontal(|ui| {
                        ui.label("Rotation:");
                        ui.label(format!("{:.1}°", rotation_degrees));
                    });
                    
                    ui.small("Note: Use mouse dragging to move entities");
                });
                
                // Health section
                if let Some(health) = health {
                    ui.separator();
                    ui.collapsing("Health", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Current:");
                            ui.label(format!("{:.1}", health.current));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Max:");
                            ui.label(format!("{:.1}", health.max));
                        });
                        
                        let health_ratio = health.current / health.max;
                        ui.add(egui::ProgressBar::new(health_ratio)
                            .text(format!("{:.1}/{:.1}", health.current, health.max)));
                    });
                }
                
                // Collision section
                if let Some(collision) = collision {
                    ui.separator();
                    ui.collapsing("Collision", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Radius:");
                            ui.label(format!("{:.1}", collision.radius));
                        });
                    });
                }
                
            } else {
                ui.label("Selected entity no longer exists");
            }
        } else {
            ui.label("No entity selected");
            ui.separator();
            ui.small("Click on an entity to inspect its properties");
            ui.separator();
            ui.small("Keyboard shortcuts:");
            ui.small("• Tab: Toggle Inspector");
            ui.small("• H: Toggle Hierarchy");
            ui.small("• G: Toggle Grid Settings");
        }
    };

    // This function is now only called from the unified system in editor.rs
    // Window management is handled there to avoid conflicts
}

/// Render inspector content without window management - for use by unified panel system
pub fn render_inspector_content(
    ui: &mut egui::Ui,
    selected_entity: &SelectedEntity,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
) {
    if let Some(entity) = selected_entity.entity {
        if let Ok((_, transform, player, enemy, health, collision)) = entity_query.get(entity) {
            ui.separator();
            
            // Entity ID and type
            ui.horizontal(|ui| {
                ui.label("Entity ID:");
                ui.label(format!("{}", entity.index()));
            });
            
            if player.is_some() {
                ui.label("Type: Player");
            } else if enemy.is_some() {
                ui.label("Type: Enemy");
            } else {
                ui.label("Type: Unknown");
            }
            
            ui.separator();
            
            // Transform section (read-only for now to avoid query conflicts)
            ui.collapsing("Transform", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Position:");
                    ui.label(format!("({:.2}, {:.2}, {:.2})",
                        transform.translation.x,
                        transform.translation.y,
                        transform.translation.z
                    ));
                });
                
                ui.horizontal(|ui| {
                    ui.label("Scale:");
                    ui.label(format!("({:.2}, {:.2})",
                        transform.scale.x,
                        transform.scale.y
                    ));
                });
                
                let rotation_degrees = transform.rotation.to_euler(EulerRot::ZYX).0.to_degrees();
                ui.horizontal(|ui| {
                    ui.label("Rotation:");
                    ui.label(format!("{:.1}°", rotation_degrees));
                });
                
                ui.small("Note: Use mouse dragging to move entities");
            });
            
            // Health section
            if let Some(health) = health {
                ui.separator();
                ui.collapsing("Health", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Current:");
                        ui.label(format!("{:.1}", health.current));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Max:");
                        ui.label(format!("{:.1}", health.max));
                    });
                    
                    let health_ratio = health.current / health.max;
                    ui.add(egui::ProgressBar::new(health_ratio)
                        .text(format!("{:.1}/{:.1}", health.current, health.max)));
                });
            }
            
            // Collision section
            if let Some(collision) = collision {
                ui.separator();
                ui.collapsing("Collision", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Radius:");
                        ui.label(format!("{:.1}", collision.radius));
                    });
                });
            }
            
        } else {
            ui.label("Selected entity no longer exists");
        }
    } else {
        ui.label("No entity selected");
        ui.separator();
        ui.small("Click on an entity to inspect its properties");
        ui.separator();
        ui.small("Keyboard shortcuts:");
        ui.small("• Tab: Toggle Inspector");
        ui.small("• H: Toggle Hierarchy");
        ui.small("• G: Toggle Grid Settings");
    }
}