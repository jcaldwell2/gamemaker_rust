//! User interface modules

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui_dock::{DockArea, TabViewer};

pub mod editor;
pub mod inspector;
pub mod hierarchy;
pub mod menus;
pub mod asset_browser;

use crate::components::*;
use crate::resources::*;

/// Menu UI system
pub fn menu_ui(
    mut contexts: EguiContexts,
    mut project_manager: ResMut<ProjectManager>,
    mut scene_manager: ResMut<SceneManager>,
    mut editor_state: ResMut<EditorState>,
    mut grid_settings: ResMut<GridSettings>,
    mut game_state: ResMut<GameState>,
    shooting_stats: Res<ShootingStats>,
    mut editor_scene_state: ResMut<EditorSceneState>,
    entity_query: Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
) {
    let ctx = contexts.ctx_mut();
    menus::render_menu_bar(
        ctx,
        &mut project_manager,
        &mut scene_manager,
        &mut editor_state,
        &mut grid_settings,
        &mut game_state,
        &shooting_stats,
        &mut editor_scene_state,
        &entity_query,
    );
}

/// Inspector UI system - only renders in separate windows mode
pub fn inspector_ui(
    mut contexts: EguiContexts,
    mut editor_state: ResMut<EditorState>,
    selected_entity: Res<SelectedEntity>,
    entity_query: Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
) {
    // Only render if in separate windows mode to avoid conflicts with unified panel
    if matches!(editor_state.window_layout_mode, WindowLayoutMode::SeparateWindows) && editor_state.show_inspector {
        let ctx = contexts.ctx_mut();
        egui::Window::new("Inspector")
            .open(&mut editor_state.show_inspector)
            .default_width(300.0)
            .resizable(true)
            .show(ctx, |ui| {
                inspector::render_inspector_content(ui, &selected_entity, &entity_query);
            });
    }
}

/// Hierarchy UI system - only renders in separate windows mode
pub fn hierarchy_ui(
    mut contexts: EguiContexts,
    mut editor_state: ResMut<EditorState>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    mut scene_manager: ResMut<SceneManager>,
) {
    // Only render if in separate windows mode to avoid conflicts with unified panel
    if matches!(editor_state.window_layout_mode, WindowLayoutMode::SeparateWindows) && editor_state.show_hierarchy {
        let ctx = contexts.ctx_mut();
        let mut show_hierarchy = editor_state.show_hierarchy;
        egui::Window::new("Hierarchy")
            .open(&mut show_hierarchy)
            .default_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Scene Entities");
                hierarchy::render_hierarchy_content(ui, &entity_query, &mut selected_entity, &mut commands, &editor_state, &mut scene_manager);
            });
        editor_state.show_hierarchy = show_hierarchy;
    }
}

/// New dockable UI system using egui_dock
pub fn dockable_ui_system(
    mut contexts: EguiContexts,
    mut dock_tree: ResMut<DockTree>,
    mut project_manager: ResMut<ProjectManager>,
    mut scene_manager: ResMut<SceneManager>,
    mut editor_state: ResMut<EditorState>,
    mut grid_settings: ResMut<GridSettings>,
    mut background_settings: ResMut<BackgroundSettings>,
    mut game_state: ResMut<GameState>,
    shooting_stats: Res<ShootingStats>,
    mut editor_scene_state: ResMut<EditorSceneState>,
    mut commands: Commands,
    mut selected_entity: ResMut<SelectedEntity>,
    entity_query: Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    asset_registry: Res<AssetRegistry>,
    mut asset_importer: ResMut<AssetImporter>,
    mut asset_browser_state: ResMut<AssetBrowserState>,
) {
    let ctx = contexts.ctx_mut();

    // Top menu bar with enhanced layout controls
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Standard menu items
            menus::render_menu_bar(
                ctx,
                &mut project_manager,
                &mut scene_manager,
                &mut editor_state,
                &mut grid_settings,
                &mut game_state,
                &shooting_stats,
                &mut editor_scene_state,
                &entity_query,
            );
            
            ui.separator();
            
            // Layout management controls
            ui.menu_button("Layout", |ui| {
                ui.label("Layout Presets:");
                
                if ui.button("🏢 Professional").clicked() {
                    dock_tree.reset_to_professional_layout();
                    ui.close_menu();
                }
                
                if ui.button("🎯 Minimal").clicked() {
                    *dock_tree = DockTree::create_minimal_layout();
                    ui.close_menu();
                }
                
                if ui.button("🎨 Scene Design").clicked() {
                    *dock_tree = DockTree::create_scene_design_layout();
                    ui.close_menu();
                }
                
                if ui.button("🐛 Debug").clicked() {
                    *dock_tree = DockTree::create_debug_layout();
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("🔄 Reset to Default").clicked() {
                    dock_tree.reset_to_professional_layout();
                    ui.close_menu();
                }
            });
        });
    });

    // Create a simple TabViewer that uses direct function calls
    let mut tab_viewer = DirectTabViewer;
    
    // Main dockable area
    DockArea::new(&mut dock_tree.state)
        .show(ctx, &mut tab_viewer);
}

// Simple TabViewer that doesn't store any references
struct DirectTabViewer;

impl TabViewer for DirectTabViewer {
    type Tab = EditorTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EditorTab::Viewport => {
                render_viewport_tab(ui);
            }
            EditorTab::Inspector => {
                render_inspector_tab(ui);
            }
            EditorTab::Hierarchy => {
                render_hierarchy_tab(ui);
            }
            EditorTab::AssetBrowser => {
                render_asset_browser_tab(ui);
            }
            EditorTab::Console => {
                render_console_tab(ui);
            }
            EditorTab::SceneSettings => {
                render_scene_settings_tab(ui);
            }
            EditorTab::GameControls => {
                render_game_controls_tab(ui);
            }
            EditorTab::EntitySpawner => {
                render_entity_spawner_tab(ui);
            }
            EditorTab::AssetManager => {
                render_asset_manager_tab(ui);
            }
            EditorTab::GridSettings => {
                render_grid_settings_tab(ui);
            }
            EditorTab::BackgroundSettings => {
                render_background_settings_tab(ui);
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            EditorTab::Viewport => "🎮 Viewport".into(),
            EditorTab::Inspector => "🔍 Inspector".into(),
            EditorTab::Hierarchy => "🌳 Hierarchy".into(),
            EditorTab::AssetBrowser => "📁 Asset Browser".into(),
            EditorTab::Console => "🖥️ Console".into(),
            EditorTab::SceneSettings => "🎬 Scene Settings".into(),
            EditorTab::GameControls => "🎮 Game Controls".into(),
            EditorTab::EntitySpawner => "➕ Entity Spawner".into(),
            EditorTab::AssetManager => "📦 Asset Manager".into(),
            EditorTab::GridSettings => "⚏ Grid Settings".into(),
            EditorTab::BackgroundSettings => "🖼️ Background Settings".into(),
        }
    }
    
    fn context_menu(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab, _surface: egui_dock::SurfaceIndex, _node: egui_dock::NodeIndex) {
        ui.label("Tab Options");
        ui.separator();
        
        if ui.button("📌 Keep Open").clicked() {
            ui.close_menu();
        }
        
        if ui.button("🔄 Reset Position").clicked() {
            ui.close_menu();
        }
        
        ui.separator();
        
        match tab {
            EditorTab::Viewport => {
                ui.label("Main game viewport");
                ui.label("Shows the rendered game world");
            }
            EditorTab::Hierarchy => {
                ui.label("Scene entity hierarchy");
                ui.label("Organize and select entities");
            }
            EditorTab::Inspector => {
                ui.label("Entity property inspector");
                ui.label("Edit selected entity properties");
            }
            EditorTab::AssetBrowser => {
                ui.label("Asset management browser");
                ui.label("Import and manage project assets");
            }
            EditorTab::Console => {
                ui.label("Debug console output");
                ui.label("View logs and debug information");
            }
            _ => {
                ui.label("Tool panel");
                ui.label("Additional editor functionality");
            }
        }
    }
}

fn render_viewport_tab(ui: &mut egui::Ui) {
    ui.heading("🎮 Game Viewport");
    ui.separator();
    
    // Main viewport content area - this is where the Bevy game world renders
    let available_rect = ui.available_rect_before_wrap();
    
    // Draw a background to show the viewport area
    ui.painter().rect_filled(
        available_rect,
        egui::Rounding::same(2.0),
        egui::Color32::from_gray(30),
    );
    
    // Add viewport instructions
    ui.allocate_ui_at_rect(available_rect, |ui| {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("🎮 Game Viewport");
                ui.small("The Bevy game world renders here");
                ui.separator();
                ui.small("Controls:");
                ui.small("• Mouse: Select/Drag entities (Editor mode)");
                ui.small("• WASD: Move player (Play mode)");
                ui.small("• Space: Shoot (Play mode)");
                ui.small("• Scroll: Zoom camera");
            });
        });
    });
}

fn render_console_tab(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.heading("🖥️ Console");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Clear").clicked() {
                // TODO: Clear console
            }
            if ui.button("Export").clicked() {
                // TODO: Export console log
            }
        });
    });
    
    ui.separator();
    
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .max_height(300.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                // Sample console entries with timestamps and different log levels
                ui.horizontal(|ui| {
                    ui.small("12:34:56");
                    ui.colored_label(egui::Color32::GREEN, "[INFO]");
                    ui.label("🚀 GameMaker Rust v0.3.0 initialized");
                });
                
                ui.horizontal(|ui| {
                    ui.small("12:34:56");
                    ui.colored_label(egui::Color32::BLUE, "[INFO]");
                    ui.label("🎨 Dockable UI system loaded successfully");
                });
                
                ui.horizontal(|ui| {
                    ui.small("12:34:57");
                    ui.colored_label(egui::Color32::GREEN, "[INFO]");
                    ui.label("📦 Assets loaded from assets/");
                });
                
                ui.horizontal(|ui| {
                    ui.small("12:34:57");
                    ui.colored_label(egui::Color32::YELLOW, "[DEBUG]");
                    ui.label("🔧 Entity Component System active");
                });
                
                ui.horizontal(|ui| {
                    ui.small("12:34:58");
                    ui.colored_label(egui::Color32::GREEN, "[INFO]");
                    ui.label("🎬 Scene 'default_scene.ron' loaded");
                });
                
                ui.horizontal(|ui| {
                    ui.small("12:34:59");
                    ui.colored_label(egui::Color32::LIGHT_BLUE, "[TRACE]");
                    ui.label("🎮 Entering editor mode");
                });
                
                ui.horizontal(|ui| {
                    ui.small("12:35:00");
                    ui.colored_label(egui::Color32::GRAY, "[TRACE]");
                    ui.label("📊 Rendering at 60 FPS");
                });
            });
        });
}

fn render_inspector_tab(ui: &mut egui::Ui) {
    ui.heading("🔍 Inspector");
    ui.separator();
    
    ui.collapsing("Transform", |ui| {
        ui.horizontal(|ui| {
            ui.label("Position:");
            ui.add(egui::DragValue::new(&mut 0.0f32).prefix("X: "));
            ui.add(egui::DragValue::new(&mut 0.0f32).prefix("Y: "));
        });
        ui.horizontal(|ui| {
            ui.label("Rotation:");
            ui.add(egui::DragValue::new(&mut 0.0f32).suffix("°"));
        });
        ui.horizontal(|ui| {
            ui.label("Scale:");
            ui.add(egui::DragValue::new(&mut 1.0f32).prefix("X: "));
            ui.add(egui::DragValue::new(&mut 1.0f32).prefix("Y: "));
        });
    });
    
    ui.collapsing("Components", |ui| {
        ui.label("No entity selected");
    });
}

fn render_hierarchy_tab(ui: &mut egui::Ui) {
    ui.heading("🌳 Hierarchy");
    ui.separator();
    
    ui.horizontal(|ui| {
        if ui.button("➕ Add Entity").clicked() {
            // Add entity logic
        }
        if ui.button("🗑️ Delete").clicked() {
            // Delete entity logic
        }
        if ui.button("🔄 Refresh").clicked() {
            // Refresh hierarchy logic
        }
    });
    
    ui.separator();
    
    ui.collapsing("Scene Objects", |ui| {
        ui.label("📦 Entity 1");
        ui.label("📦 Entity 2");
        ui.label("📦 Entity 3");
    });
}

fn render_asset_browser_tab(ui: &mut egui::Ui) {
    ui.heading("📁 Asset Browser");
    ui.separator();
    
    ui.horizontal(|ui| {
        if ui.button("📂 Import").clicked() {
            // Import asset logic
        }
        if ui.button("🗂️ New Folder").clicked() {
            // Create folder logic
        }
        if ui.button("🔄 Refresh").clicked() {
            // Refresh assets logic
        }
    });
    
    ui.separator();
    
    ui.collapsing("📁 Textures", |ui| {
        ui.label("🖼️ sprite1.png");
        ui.label("🖼️ sprite2.png");
    });
    
    ui.collapsing("📁 Audio", |ui| {
        ui.label("🎵 sound1.wav");
        ui.label("🎵 sound2.ogg");
    });
}

fn render_game_controls_tab(ui: &mut egui::Ui) {
    ui.heading("🎮 Game Controls");
    ui.separator();
    
    ui.horizontal(|ui| {
        if ui.button("▶️ Play").clicked() {
            // Play game logic
        }
        if ui.button("⏸️ Pause").clicked() {
            // Pause game logic
        }
        if ui.button("⏹️ Stop").clicked() {
            // Stop game logic
        }
    });
    
    ui.separator();
    
    ui.group(|ui| {
        ui.label("Game State: Editor Mode");
        ui.label("FPS: 60");
        ui.label("Entities: 0");
    });
}

fn render_entity_spawner_tab(ui: &mut egui::Ui) {
    ui.heading("➕ Entity Spawner");
    ui.separator();
    
    ui.horizontal(|ui| {
        if ui.button("👤 Player").clicked() {
            // Spawn player logic
        }
        if ui.button("👹 Enemy").clicked() {
            // Spawn enemy logic
        }
        if ui.button("🏠 Object").clicked() {
            // Spawn object logic
        }
    });
    
    ui.separator();
    
    ui.collapsing("Spawn Settings", |ui| {
        ui.horizontal(|ui| {
            ui.label("Position:");
            ui.add(egui::DragValue::new(&mut 0.0f32).prefix("X: "));
            ui.add(egui::DragValue::new(&mut 0.0f32).prefix("Y: "));
        });
    });
}

fn render_asset_manager_tab(ui: &mut egui::Ui) {
    ui.heading("📦 Asset Manager");
    ui.separator();
    
    ui.group(|ui| {
        ui.label("Memory Usage: 45.2 MB");
        ui.label("Loaded Assets: 23");
        ui.label("Cache Size: 12.1 MB");
    });
    
    ui.separator();
    
    ui.horizontal(|ui| {
        if ui.button("🔄 Reload All").clicked() {
            // Reload assets logic
        }
        if ui.button("🗑️ Clear Cache").clicked() {
            // Clear cache logic
        }
    });
}

fn render_grid_settings_tab(ui: &mut egui::Ui) {
    ui.heading("⚏ Grid Settings");
    ui.separator();
    
    ui.checkbox(&mut true, "Show Grid");
    ui.checkbox(&mut false, "Snap to Grid");
    
    ui.separator();
    
    ui.horizontal(|ui| {
        ui.label("Grid Size:");
        ui.add(egui::DragValue::new(&mut 32.0f32).suffix("px"));
    });
    
    ui.horizontal(|ui| {
        ui.label("Opacity:");
        ui.add(egui::Slider::new(&mut 0.5f32, 0.0..=1.0));
    });
}

fn render_background_settings_tab(ui: &mut egui::Ui) {
    ui.heading("🖼️ Background Settings");
    ui.separator();
    
    ui.horizontal(|ui| {
        if ui.button("📂 Load Image").clicked() {
            // Load background image logic
        }
        if ui.button("🗑️ Remove").clicked() {
            // Remove background logic
        }
    });
    
    ui.separator();
    
    ui.collapsing("Position", |ui| {
        ui.horizontal(|ui| {
            ui.label("X:");
            ui.add(egui::DragValue::new(&mut 0.0f32));
        });
        ui.horizontal(|ui| {
            ui.label("Y:");
            ui.add(egui::DragValue::new(&mut 0.0f32));
        });
    });
    
    ui.collapsing("Scale", |ui| {
        ui.horizontal(|ui| {
            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut 1.0f32, 0.1..=5.0));
        });
    });
}

fn render_scene_settings_tab(ui: &mut egui::Ui) {
    ui.heading("🎬 Scene Settings");
    ui.separator();
    
    ui.horizontal(|ui| {
        ui.label("📂 Save Path:");
        ui.label("scenes/default_scene.ron");
    });
    
    ui.separator();
    
    ui.horizontal(|ui| {
        if ui.button("💾 Save Scene").clicked() {
            info!("Saving scene to: scenes/default_scene.ron");
        }
        
        if ui.button("📁 Load Scene").clicked() {
            info!("Loading scene from: scenes/default_scene.ron");
        }
        
        if ui.button("🆕 New Scene").clicked() {
            info!("Creating new scene");
        }
    });
    
    ui.separator();
    
    ui.label("Scene Properties:");
    ui.small("• Entity Count: Active entities in scene");
    ui.small("• Last Modified: When scene was last saved");
    ui.small("• File Size: Current scene file size");
}

/// Editor panels UI system
pub fn editor_panels_ui(
    mut contexts: EguiContexts,
    mut editor_state: ResMut<EditorState>,
    mut grid_settings: ResMut<GridSettings>,
    mut background_settings: ResMut<BackgroundSettings>,
    mut scene_manager: ResMut<SceneManager>,
    shooting_stats: Res<ShootingStats>,
    mut game_state: ResMut<GameState>,
    mut editor_scene_state: ResMut<EditorSceneState>,
    mut commands: Commands,
    mut selected_entity: ResMut<SelectedEntity>,
    entity_query: Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
    asset_registry: Res<AssetRegistry>,
    mut asset_importer: ResMut<AssetImporter>,
    mut asset_browser_state: ResMut<AssetBrowserState>,
) {
    let ctx = contexts.ctx_mut();
    editor::render_editor_panels(
        ctx,
        &mut editor_state,
        &mut grid_settings,
        &mut background_settings,
        &mut scene_manager,
        &shooting_stats,
        &mut game_state,
        &mut editor_scene_state,
        &mut commands,
        &mut selected_entity,
        &entity_query,
        &asset_registry,
        &mut asset_importer,
        &mut asset_browser_state,
    );
}