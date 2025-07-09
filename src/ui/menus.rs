//! Menu bar and file operations

use bevy::prelude::*;
use bevy_egui::egui;

use crate::components::*;
use crate::resources::*;
use crate::scene::*;

/// Render the main menu bar with integrated game controls
pub fn render_menu_bar(
    ctx: &egui::Context,
    project_manager: &mut ProjectManager,
    scene_manager: &mut SceneManager,
    editor_state: &mut EditorState,
    grid_settings: &mut GridSettings,
    game_state: &mut GameState,
    shooting_stats: &ShootingStats,
    editor_scene_state: &mut EditorSceneState,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
) {
    egui::TopBottomPanel::top("unified_menu_bar").show(ctx, |ui| {
        // Menu bar row
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Project").clicked() {
                    project_manager.current_project_path = None;
                    project_manager.project_name = "Untitled Project".to_string();
                    project_manager.unsaved_changes = false;
                    ui.close_menu();
                }
                
                if ui.button("Save Project").clicked() {
                    if let Some(path) = &project_manager.current_project_path {
                        // Save project logic would go here
                        println!("Saving project to: {}", path);
                        project_manager.unsaved_changes = false;
                    } else {
                        // Show save dialog
                        println!("Save As dialog would open here");
                    }
                    ui.close_menu();
                }
                
                if ui.button("Load Project").clicked() {
                    // Load project logic would go here
                    println!("Load project dialog would open here");
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Save Scene").clicked() {
                    // This would be handled by the scene system
                    println!("Saving scene to: {}", scene_manager.save_path);
                    ui.close_menu();
                }
                
                if ui.button("Load Scene").clicked() {
                    // This would be handled by the scene system
                    println!("Loading scene from: {}", scene_manager.save_path);
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    println!("Undo functionality not implemented yet");
                    ui.close_menu();
                }
                
                if ui.button("Redo").clicked() {
                    println!("Redo functionality not implemented yet");
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Copy").clicked() {
                    println!("Copy functionality not implemented yet");
                    ui.close_menu();
                }
                
                if ui.button("Paste").clicked() {
                    println!("Paste functionality not implemented yet");
                    ui.close_menu();
                }
            });
            
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut editor_state.show_inspector, "Inspector");
                ui.checkbox(&mut editor_state.show_hierarchy, "Hierarchy");
                ui.checkbox(&mut editor_state.show_scene_manager, "Scene Manager");
                ui.checkbox(&mut editor_state.show_entity_spawner, "Entity Spawner");
                ui.checkbox(&mut editor_state.show_asset_manager, "Asset Manager");
                ui.checkbox(&mut editor_state.show_asset_browser, "Asset Browser");
                ui.checkbox(&mut editor_state.show_game_controls, "Game Controls");
                ui.checkbox(&mut grid_settings.enabled, "Show Grid");
                ui.checkbox(&mut editor_state.show_grid, "Grid Settings");
                ui.checkbox(&mut editor_state.show_background, "Background Settings");
                
                ui.separator();
                
                ui.label("Window Layout:");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut editor_state.window_layout_mode, WindowLayoutMode::SeparateWindows, "Separate Windows");
                    ui.radio_value(&mut editor_state.window_layout_mode, WindowLayoutMode::OverlayPanels, "Overlay Panels");
                });
                
                ui.separator();
                
                if ui.button("Reset Camera").clicked() {
                    println!("Reset camera functionality not implemented yet");
                    ui.close_menu();
                }
                
                if ui.button("Fit to Screen").clicked() {
                    println!("Fit to screen functionality not implemented yet");
                    ui.close_menu();
                }
            });
            
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    println!("GameMaker Rust - A 2D game engine built with Bevy");
                    ui.close_menu();
                }
                
                if ui.button("Controls").clicked() {
                    println!("Controls help would be shown here");
                    ui.close_menu();
                }
            });
        });
        
        ui.separator();
        
        // Game controls toolbar row
        ui.horizontal(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(12.0, 8.0);
            
            // Play/Pause/Stop buttons
            let play_text = if game_state.playing { "⏸ Pause" } else { "▶ Play" };
            let play_color = if game_state.playing {
                egui::Color32::from_rgb(255, 165, 0) // Orange for pause
            } else {
                egui::Color32::from_rgb(0, 200, 0) // Green for play
            };
            
            if ui.add(egui::Button::new(play_text).fill(play_color)).clicked() {
                if game_state.playing {
                    // Pause the game
                    game_state.paused = !game_state.paused;
                    info!("Game {}", if game_state.paused { "paused" } else { "resumed" });
                } else {
                    // Save scene state before starting play mode
                    save_scene_state_for_play(editor_scene_state, entity_query);
                    
                    // Start playing
                    game_state.playing = true;
                    game_state.editor_mode = false;
                    game_state.paused = false;
                    info!("Game started - entering play mode");
                }
            }
            
            if ui.add(egui::Button::new("⏹ Stop").fill(egui::Color32::from_rgb(200, 0, 0))).clicked() {
                // Stop the game and return to editor mode
                game_state.playing = false;
                game_state.editor_mode = true;
                game_state.paused = false;
                info!("Game stopped - returning to editor mode");
            }
            
            ui.separator();
            
            // Mode indicator
            let mode_text = if game_state.editor_mode { "📝 Editor Mode" } else { "🎮 Play Mode" };
            ui.label(mode_text);
            
            ui.separator();
            
            // Game state indicators
            if game_state.paused {
                ui.label("⏸ PAUSED");
            }
            
            if game_state.debug_mode {
                ui.label("🐛 DEBUG");
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Stats on the right side
                ui.label(format!("Accuracy: {:.1}%",
                    if shooting_stats.shots_fired > 0 {
                        (shooting_stats.hits as f32 / shooting_stats.shots_fired as f32) * 100.0
                    } else {
                        0.0
                    }
                ));
                ui.label(format!("Hits: {}", shooting_stats.hits));
                ui.label(format!("Shots: {}", shooting_stats.shots_fired));
                ui.label("Stats:");
                
                ui.separator();
                
                ui.label(format!("({:.1}, {:.1})",
                    editor_state.mouse_world_position.x,
                    editor_state.mouse_world_position.y
                ));
                ui.label("Mouse:");
            });
        });
    });
}

/// Save scene state for play mode
fn save_scene_state_for_play(
    editor_scene_state: &mut EditorSceneState,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
) {
    // Create a temporary scene data string
    let mut scene_entities = Vec::new();
    for (_, transform, player, enemy, health, collision) in entity_query.iter() {
        let entity_type = if player.is_some() {
            crate::components::EntityType::Player
        } else if enemy.is_some() {
            crate::components::EntityType::Enemy
        } else {
            continue;
        };
        
        let serializable_entity = crate::components::SerializableEntity {
            entity_type,
            transform: crate::components::SerializableTransform::from(*transform),
            health: health.map(|h| (h.current, h.max)),
            collision_radius: collision.map(|c| c.radius),
            sprite_asset: None, // Default to None for now
        };
        
        scene_entities.push(serializable_entity);
    }
    
    let scene = crate::scene::Scene {
        entities: scene_entities,
        metadata: crate::scene::SceneMetadata {
            name: "Play Mode Save".to_string(),
            version: "1.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            last_modified: chrono::Utc::now().to_rfc3339(),
        },
    };
    
    if let Ok(scene_data) = ron::ser::to_string_pretty(&scene, ron::ser::PrettyConfig::default()) {
        editor_scene_state.saved_scene_data = Some(scene_data);
        info!("Scene state saved for play mode");
    } else {
        warn!("Failed to save scene state");
    }
}