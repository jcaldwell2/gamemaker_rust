//! Editor panels and tools

use bevy::prelude::*;
use bevy::math::EulerRot;
use bevy_egui::egui;

use crate::components::*;
use crate::resources::*;
use crate::ui::hierarchy;
use crate::ui::inspector;
use crate::ui::asset_browser;

/// Render editor panels and tools
pub fn render_editor_panels(
    ctx: &egui::Context,
    editor_state: &mut EditorState,
    grid_settings: &mut GridSettings,
    background_settings: &mut BackgroundSettings,
    scene_manager: &mut SceneManager,
    shooting_stats: &ShootingStats,
    game_state: &mut GameState,
    editor_scene_state: &mut EditorSceneState,
    commands: &mut Commands,
    selected_entity: &mut SelectedEntity,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    asset_registry: &AssetRegistry,
    asset_importer: &mut AssetImporter,
    asset_browser_state: &mut AssetBrowserState,
) {
    // Bottom panel for editor status - always visible
    egui::TopBottomPanel::bottom("editor_status").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("📂 Project:");
            ui.label("Untitled Project");
            
            ui.separator();
            
            ui.label("🎯 Selected:");
            if let Some(_) = selected_entity.entity {
                ui.label("Entity");
            } else {
                ui.label("None");
            }
            
            ui.separator();
            
            ui.label("🔧 Tools:");
            ui.label("Editor Mode");
        });
    });

    match editor_state.window_layout_mode {
        WindowLayoutMode::OverlayPanels => {
            render_overlay_panels(ctx, editor_state, grid_settings, background_settings, scene_manager, shooting_stats, game_state, editor_scene_state, commands, selected_entity, entity_query, asset_registry, asset_importer, asset_browser_state);
        },
        WindowLayoutMode::SeparateWindows => {
            render_separate_windows(ctx, editor_state, grid_settings, background_settings, scene_manager, shooting_stats, game_state, editor_scene_state, commands, selected_entity, entity_query, asset_registry, asset_importer, asset_browser_state);
        },
    }
}

/// Render all panels stacked in a single side panel for overlay mode
fn render_overlay_panels(
    ctx: &egui::Context,
    editor_state: &mut EditorState,
    grid_settings: &mut GridSettings,
    background_settings: &mut BackgroundSettings,
    scene_manager: &mut SceneManager,
    shooting_stats: &ShootingStats,
    game_state: &mut GameState,
    editor_scene_state: &mut EditorSceneState,
    commands: &mut Commands,
    selected_entity: &mut SelectedEntity,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    asset_registry: &AssetRegistry,
    asset_importer: &mut AssetImporter,
    asset_browser_state: &mut AssetBrowserState,
) {
    // Check if any panels should be shown
    let show_any_panel = editor_state.show_inspector || editor_state.show_hierarchy ||
                        editor_state.show_scene_manager || editor_state.show_entity_spawner ||
                        editor_state.show_asset_manager || editor_state.show_asset_browser ||
                        editor_state.show_game_controls || editor_state.show_grid || editor_state.show_background;

    if show_any_panel {
        egui::SidePanel::right("unified_panel")
            .default_width(350.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Inspector Panel
                    if editor_state.show_inspector {
                        ui.collapsing("Inspector", |ui| {
                            inspector::render_inspector_content(ui, selected_entity, entity_query);
                        });
                        ui.separator();
                    }

                    // Hierarchy Panel
                    if editor_state.show_hierarchy {
                        ui.collapsing("Hierarchy", |ui| {
                            hierarchy::render_hierarchy_content(ui, entity_query, selected_entity, commands, editor_state, scene_manager);
                        });
                        ui.separator();
                    }

                    // Scene Manager Panel
                    if editor_state.show_scene_manager {
                        ui.collapsing("Scene Manager", |ui| {
                            render_scene_manager_content(ui, scene_manager);
                        });
                        ui.separator();
                    }

                    // Entity Spawner Panel
                    if editor_state.show_entity_spawner {
                        ui.collapsing("Entity Spawner", |ui| {
                            render_entity_spawner_content(ui, editor_state, scene_manager);
                        });
                        ui.separator();
                    }

                    // Asset Manager Panel
                    if editor_state.show_asset_manager {
                        ui.collapsing("Asset Manager", |ui| {
                            render_asset_manager_content(ui);
                        });
                        ui.separator();
                    }
                    
                    // Asset Browser Panel
                    if editor_state.show_asset_browser {
                        ui.collapsing("Asset Browser", |ui| {
                            asset_browser::render_asset_browser_content(ui, asset_registry, asset_importer, asset_browser_state);
                        });
                        ui.separator();
                    }

                    // Game Controls Panel
                    if editor_state.show_game_controls {
                        ui.collapsing("Game Controls", |ui| {
                            render_game_controls_content(ui, game_state);
                        });
                        ui.separator();
                    }

                    // Grid Settings Panel
                    if editor_state.show_grid {
                        ui.collapsing("Grid Settings", |ui| {
                            render_grid_settings_content(ui, grid_settings);
                        });
                        ui.separator();
                    }

                    // Background Settings Panel
                    if editor_state.show_background {
                        ui.collapsing("Background Settings", |ui| {
                            render_background_settings_content(ui, background_settings);
                        });
                    }
                });
            });
    }
}

/// Render all panels as separate windows
fn render_separate_windows(
    ctx: &egui::Context,
    editor_state: &mut EditorState,
    grid_settings: &mut GridSettings,
    background_settings: &mut BackgroundSettings,
    scene_manager: &mut SceneManager,
    shooting_stats: &ShootingStats,
    game_state: &mut GameState,
    editor_scene_state: &mut EditorSceneState,
    commands: &mut Commands,
    selected_entity: &mut SelectedEntity,
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    asset_registry: &AssetRegistry,
    asset_importer: &mut AssetImporter,
    asset_browser_state: &mut AssetBrowserState,
) {
    // Inspector Window
    if editor_state.show_inspector {
        egui::Window::new("Inspector")
            .open(&mut editor_state.show_inspector)
            .default_width(300.0)
            .resizable(true)
            .show(ctx, |ui| {
                inspector::render_inspector_content(ui, selected_entity, entity_query);
            });
    }

    // Hierarchy Window
    if editor_state.show_hierarchy {
        let mut show_hierarchy = editor_state.show_hierarchy;
        egui::Window::new("Hierarchy")
            .open(&mut show_hierarchy)
            .default_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                hierarchy::render_hierarchy_content(ui, entity_query, selected_entity, commands, editor_state, scene_manager);
            });
        editor_state.show_hierarchy = show_hierarchy;
    }

    // Scene Manager Window
    if editor_state.show_scene_manager {
        egui::Window::new("Scene Manager")
            .open(&mut editor_state.show_scene_manager)
            .default_width(300.0)
            .resizable(true)
            .show(ctx, |ui| {
                render_scene_manager_content(ui, scene_manager);
            });
    }

    // Entity Spawner Window
    if editor_state.show_entity_spawner {
        let mut show_entity_spawner = editor_state.show_entity_spawner;
        egui::Window::new("Entity Spawner")
            .open(&mut show_entity_spawner)
            .default_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                render_entity_spawner_content(ui, editor_state, scene_manager);
            });
        editor_state.show_entity_spawner = show_entity_spawner;
    }

    // Asset Manager Window
    if editor_state.show_asset_manager {
        egui::Window::new("Asset Manager")
            .open(&mut editor_state.show_asset_manager)
            .default_width(300.0)
            .resizable(true)
            .show(ctx, |ui| {
                render_asset_manager_content(ui);
            });
    }
    
    // Asset Browser Window
    if editor_state.show_asset_browser {
        egui::Window::new("Asset Browser")
            .open(&mut editor_state.show_asset_browser)
            .default_width(350.0)
            .default_height(400.0)
            .resizable(true)
            .show(ctx, |ui| {
                asset_browser::render_asset_browser_content(ui, asset_registry, asset_importer, asset_browser_state);
            });
    }

    // Game Controls Window
    if editor_state.show_game_controls {
        egui::Window::new("Game Controls")
            .open(&mut editor_state.show_game_controls)
            .default_width(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                render_game_controls_content(ui, game_state);
            });
    }

    // Grid Settings Window
    if editor_state.show_grid {
        egui::Window::new("Grid Settings")
            .open(&mut editor_state.show_grid)
            .default_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                render_grid_settings_content(ui, grid_settings);
            });
    }

    // Background Settings Window
    if editor_state.show_background {
        egui::Window::new("Background Settings")
            .open(&mut editor_state.show_background)
            .default_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                render_background_settings_content(ui, background_settings);
            });
    }
}



/// Render scene manager content
fn render_scene_manager_content(ui: &mut egui::Ui, scene_manager: &SceneManager) {
    ui.horizontal(|ui| {
        ui.label("Save Path:");
        ui.label(&scene_manager.save_path);
    });
    
    ui.separator();
    
    if ui.button("Save Scene").clicked() {
        println!("Saving scene to: {}", scene_manager.save_path);
    }
    
    if ui.button("Load Scene").clicked() {
        println!("Loading scene from: {}", scene_manager.save_path);
    }
    
    if ui.button("New Scene").clicked() {
        println!("Creating new scene");
    }
}

/// Render entity spawner content
pub fn render_entity_spawner_content(ui: &mut egui::Ui, editor_state: &EditorState, scene_manager: &mut SceneManager) {
    ui.horizontal(|ui| {
        ui.label("Type:");
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", scene_manager.spawn_entity_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Player, "Player");
                ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Enemy, "Enemy");
                ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Projectile, "Projectile");
            });
    });
    
    ui.horizontal(|ui| {
        ui.label("Position:");
    });
    
    ui.horizontal(|ui| {
        ui.label("X:");
        ui.add(egui::DragValue::new(&mut scene_manager.spawn_position.x)
            .speed(1.0));
    });
    
    ui.horizontal(|ui| {
        ui.label("Y:");
        ui.add(egui::DragValue::new(&mut scene_manager.spawn_position.y)
            .speed(1.0));
    });
    
    ui.horizontal(|ui| {
        ui.label("Z:");
        ui.add(egui::DragValue::new(&mut scene_manager.spawn_z)
            .speed(0.1)
            .clamp_range(-1000.0..=1000.0));
    });
    
    if ui.button("Spawn Entity").clicked() {
        scene_manager.should_spawn = true;
    }
    
    if ui.button("Spawn at Mouse").clicked() {
        scene_manager.spawn_position = editor_state.mouse_world_position;
        scene_manager.should_spawn = true;
    }
}

/// Render asset manager content
pub fn render_asset_manager_content(ui: &mut egui::Ui) {
    ui.label("Assets:");
    ui.separator();
    ui.label("No assets loaded");
    ui.separator();
    
    if ui.button("Import Asset").clicked() {
        println!("Asset import dialog would open here");
    }
    
    if ui.button("Refresh Assets").clicked() {
        println!("Refreshing asset list");
    }
}

/// Render game controls content
pub fn render_game_controls_content(ui: &mut egui::Ui, game_state: &mut GameState) {
    ui.heading("Game Controls");
    
    ui.horizontal(|ui| {
        ui.label("Current State:");
        if game_state.playing {
            if game_state.paused {
                ui.colored_label(egui::Color32::YELLOW, "⏸ PAUSED");
            } else {
                ui.colored_label(egui::Color32::GREEN, "▶ PLAYING");
            }
        } else {
            ui.colored_label(egui::Color32::GRAY, "⏹ STOPPED");
        }
    });
    
    ui.horizontal(|ui| {
        ui.label("Mode:");
        if game_state.editor_mode {
            ui.colored_label(egui::Color32::BLUE, "📝 Editor");
        } else {
            ui.colored_label(egui::Color32::RED, "🎮 Game");
        }
    });
    
    ui.separator();
    
    // Control buttons
    ui.horizontal(|ui| {
        let play_text = if game_state.playing { "⏸ Pause" } else { "▶ Play" };
        if ui.button(play_text).clicked() {
            if game_state.playing {
                // Pause the game
                game_state.paused = !game_state.paused;
                info!("Game {}", if game_state.paused { "paused" } else { "resumed" });
            } else {
                // Start playing
                game_state.playing = true;
                game_state.editor_mode = false;
                game_state.paused = false;
                info!("Game started - entering play mode");
            }
        }
        
        if ui.button("⏹ Stop").clicked() {
            // Stop the game and return to editor mode
            game_state.playing = false;
            game_state.editor_mode = true;
            game_state.paused = false;
            info!("Game stopped - returning to editor mode");
        }
    });
    
    ui.separator();
    
    // Debug controls
    ui.horizontal(|ui| {
        if ui.button("Toggle Debug (F1)").clicked() {
            game_state.debug_mode = !game_state.debug_mode;
            info!("Debug mode: {}", game_state.debug_mode);
        }
        
        if game_state.debug_mode {
            ui.colored_label(egui::Color32::YELLOW, "🐛 DEBUG ON");
        }
    });
    
    ui.separator();
    
    // Scene state info
    ui.label("Scene State:");
    ui.small("• Scene data is automatically saved when entering play mode");
    ui.small("• Stop button restores the scene to its saved state");
    ui.small("• Pause freezes the game but keeps current state");
    
    ui.separator();
    
    ui.label("Keyboard Controls:");
    ui.small("• P: Toggle Pause");
    ui.small("• F1: Toggle Debug Mode");
    ui.small("• WASD: Move player (in play mode)");
    ui.small("• Space: Shoot (in play mode)");
    ui.small("• Mouse: Select/Drag entities (in editor mode)");
    ui.small("• Scroll: Zoom camera");
}

/// Render grid settings content
pub fn render_grid_settings_content(ui: &mut egui::Ui, grid_settings: &mut GridSettings) {
    ui.checkbox(&mut grid_settings.enabled, "Show Grid");
    
    ui.horizontal(|ui| {
        ui.label("Spacing:");
        ui.add(egui::DragValue::new(&mut grid_settings.spacing)
            .speed(1.0)
            .clamp_range(10.0..=200.0));
    });
    
    ui.horizontal(|ui| {
        ui.label("Thickness:");
        ui.add(egui::DragValue::new(&mut grid_settings.thickness)
            .speed(0.1)
            .clamp_range(0.1..=5.0));
    });
    
    ui.horizontal(|ui| {
        ui.label("Opacity:");
        ui.add(egui::Slider::new(&mut grid_settings.opacity, 0.0..=1.0));
    });
    
    ui.horizontal(|ui| {
        ui.label("Color:");
        let mut color = [
            grid_settings.color.r(),
            grid_settings.color.g(),
            grid_settings.color.b(),
        ];
        if ui.color_edit_button_rgb(&mut color).changed() {
            grid_settings.color = Color::rgb(color[0], color[1], color[2]);
        }
    });
}

/// Render background settings content
pub fn render_background_settings_content(ui: &mut egui::Ui, background_settings: &mut BackgroundSettings) {
    ui.checkbox(&mut background_settings.enabled, "Show Background");
    
    ui.horizontal(|ui| {
        ui.label("Image Path:");
        if let Some(ref mut path) = background_settings.image_path {
            ui.text_edit_singleline(path);
        } else {
            let mut temp_path = String::new();
            if ui.text_edit_singleline(&mut temp_path).changed() && !temp_path.is_empty() {
                background_settings.image_path = Some(temp_path);
            }
        }
    });
    
    ui.horizontal(|ui| {
        ui.label("Opacity:");
        ui.add(egui::Slider::new(&mut background_settings.opacity, 0.0..=1.0));
    });
    
    ui.horizontal(|ui| {
        ui.label("Scale:");
        ui.add(egui::DragValue::new(&mut background_settings.scale)
            .speed(0.1)
            .clamp_range(0.1..=5.0));
    });
}
