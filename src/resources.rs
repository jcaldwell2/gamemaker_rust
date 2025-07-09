//! Game resources and state management

use bevy::prelude::*;
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use egui_dock::DockState;

use crate::components::EntityType;

/// Main game state
#[derive(Resource, Default)]
pub struct GameState {
    pub paused: bool,
    pub debug_mode: bool,
    pub playing: bool,
    pub editor_mode: bool,
}

#[derive(Resource, Default)]
pub struct EditorSceneState {
    pub saved_scene_data: Option<String>, // RON serialized scene data
    pub temp_scene_data: Option<String>,  // Temporary save when entering play mode
}

/// Camera controller resource
#[derive(Resource)]
pub struct CameraController {
    pub target_position: Vec2,
    pub following_entity: Option<Entity>,
    pub zoom: f32,
    pub target_zoom: f32,
    pub instant_movement: bool, // For instant camera movement during drag
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            target_position: Vec2::ZERO,
            following_entity: None,
            zoom: 1.0,
            target_zoom: 1.0,
            instant_movement: false,
        }
    }
}

/// Selected entity resource
#[derive(Resource, Default)]
pub struct SelectedEntity {
    pub entity: Option<Entity>,
}

/// Drag state for entity manipulation
#[derive(Resource, Default)]
pub struct DragState {
    pub dragging: bool,
    pub drag_offset: Vec2,
}

/// Asset metadata for loaded assets
#[derive(Clone, Debug)]
pub struct AssetMetadata {
    pub name: String,
    pub path: String,
    pub file_size: u64,
    pub image_dimensions: Option<(u32, u32)>,
    pub import_date: String,
    pub last_modified: String,
}

/// Asset registry for managing loaded assets
#[derive(Resource, Default)]
pub struct AssetRegistry {
    pub loaded_images: HashMap<String, Handle<Image>>,
    pub asset_metadata: HashMap<String, AssetMetadata>,
    pub loading_assets: HashMap<String, Handle<Image>>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            loaded_images: HashMap::new(),
            asset_metadata: HashMap::new(),
            loading_assets: HashMap::new(),
        }
    }
    
    pub fn register_image(&mut self, path: String, handle: Handle<Image>, metadata: AssetMetadata) {
        self.loaded_images.insert(path.clone(), handle);
        self.asset_metadata.insert(path, metadata);
    }
    
    pub fn get_image(&self, path: &str) -> Option<&Handle<Image>> {
        self.loaded_images.get(path)
    }
    
    pub fn get_metadata(&self, path: &str) -> Option<&AssetMetadata> {
        self.asset_metadata.get(path)
    }
    
    pub fn is_loaded(&self, path: &str) -> bool {
        self.loaded_images.contains_key(path)
    }
    
    pub fn is_loading(&self, path: &str) -> bool {
        self.loading_assets.contains_key(path)
    }
    
    pub fn start_loading(&mut self, path: String, handle: Handle<Image>) {
        self.loading_assets.insert(path, handle);
    }
    
    pub fn finish_loading(&mut self, path: &str) {
        self.loading_assets.remove(path);
    }
    
    pub fn get_loaded_paths(&self) -> Vec<String> {
        self.loaded_images.keys().cloned().collect()
    }
}

/// Asset importer resource
#[derive(Resource, Default)]
pub struct AssetImporter {
    pub pending_imports: Vec<String>,
    pub import_queue: Vec<String>,
    pub failed_imports: Vec<(String, String)>, // (path, error_message)
}

impl AssetImporter {
    pub fn queue_import(&mut self, path: String) {
        if !self.import_queue.contains(&path) && !self.pending_imports.contains(&path) {
            self.import_queue.push(path);
        }
    }
    
    pub fn start_import(&mut self, path: String) {
        if let Some(index) = self.import_queue.iter().position(|p| p == &path) {
            self.import_queue.remove(index);
            self.pending_imports.push(path);
        }
    }
    
    pub fn complete_import(&mut self, path: &str) {
        if let Some(index) = self.pending_imports.iter().position(|p| p == path) {
            self.pending_imports.remove(index);
        }
    }
    
    pub fn fail_import(&mut self, path: String, error: String) {
        self.complete_import(&path);
        self.failed_imports.push((path, error));
    }
    
    pub fn clear_failed_imports(&mut self) {
        self.failed_imports.clear();
    }
}

/// Shooting statistics
#[derive(Resource, Default)]
pub struct ShootingStats {
    pub shots_fired: u32,
    pub hits: u32,
}

/// Project management resource
#[derive(Resource, Default)]
pub struct ProjectManager {
    pub current_project_path: Option<String>,
    pub project_name: String,
    pub unsaved_changes: bool,
}

/// Window layout mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowLayoutMode {
    SeparateWindows,
    OverlayPanels,
}

impl Default for WindowLayoutMode {
    fn default() -> Self {
        Self::SeparateWindows
    }
}

/// Editor state resource
#[derive(Resource)]
pub struct EditorState {
    pub show_inspector: bool,
    pub show_hierarchy: bool,
    pub show_grid: bool,
    pub show_background: bool,
    pub show_scene_manager: bool,
    pub show_game_controls: bool,
    pub show_asset_manager: bool,
    pub show_asset_browser: bool,
    pub show_entity_spawner: bool,
    pub mouse_world_position: Vec2,
    pub window_layout_mode: WindowLayoutMode,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            show_inspector: false,
            show_hierarchy: false,
            show_grid: true,
            show_background: false,
            show_scene_manager: false,
            show_game_controls: false,
            show_asset_manager: false,
            show_asset_browser: false,
            show_entity_spawner: false,
            mouse_world_position: Vec2::ZERO,
            window_layout_mode: WindowLayoutMode::default(),
        }
    }
}

/// Grid settings resource
#[derive(Resource)]
pub struct GridSettings {
    pub enabled: bool,
    pub spacing: f32,
    pub color: Color,
    pub opacity: f32,
    pub thickness: f32,
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            spacing: 50.0,
            color: Color::WHITE,
            opacity: 0.3,
            thickness: 1.0,
        }
    }
}

impl Hash for GridSettings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.enabled.hash(state);
        (self.spacing as u32).hash(state);
        ((self.color.r() * 255.0) as u8).hash(state);
        ((self.color.g() * 255.0) as u8).hash(state);
        ((self.color.b() * 255.0) as u8).hash(state);
        ((self.opacity * 255.0) as u8).hash(state);
        (self.thickness as u32).hash(state);
    }
}

/// Grid state for tracking changes
#[derive(Resource, Default)]
pub struct GridState {
    pub last_camera_position: Vec2,
    pub last_zoom: f32,
    pub last_settings_hash: u64,
}

impl GridState {
    pub fn needs_update(&mut self, camera_pos: Vec2, zoom: f32, settings: &GridSettings) -> bool {
        let mut hasher = DefaultHasher::new();
        settings.hash(&mut hasher);
        let current_hash = hasher.finish();
        
        let position_changed = (camera_pos - self.last_camera_position).length() > 10.0;
        let zoom_changed = (zoom - self.last_zoom).abs() > 0.01;
        let settings_changed = current_hash != self.last_settings_hash;
        
        if position_changed || zoom_changed || settings_changed {
            self.last_camera_position = camera_pos;
            self.last_zoom = zoom;
            self.last_settings_hash = current_hash;
            true
        } else {
            false
        }
    }
}

/// Background settings resource
#[derive(Resource)]
pub struct BackgroundSettings {
    pub enabled: bool,
    pub image_path: Option<String>,
    pub opacity: f32,
    pub scale: f32,
}

impl Default for BackgroundSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            image_path: None,
            opacity: 0.5,
            scale: 1.0,
        }
    }
}

/// Asset browser state
#[derive(Resource, Default)]
pub struct AssetBrowserState {
    pub show_browser: bool,
    pub selected_asset: Option<String>,
    pub filter_text: String,
    pub show_import_dialog: bool,
    pub import_path: String,
}

/// Scene manager resource
#[derive(Resource)]
pub struct SceneManager {
    pub next_id: u32,
    pub save_path: String,
    pub spawn_entity_type: EntityType,
    pub spawn_position: Vec2,
    pub spawn_z: f32,
    pub should_spawn: bool,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self {
            next_id: 0,
            save_path: "scenes/default_scene.ron".to_string(),
            spawn_entity_type: EntityType::Player,
            spawn_position: Vec2::ZERO,
            spawn_z: 0.0,
            should_spawn: false,
        }
    }
}

// Dockable UI System
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorTab {
    Viewport,
    Inspector,
    Hierarchy,
    AssetBrowser,
    Console,
    SceneSettings,
    GameControls,
    EntitySpawner,
    AssetManager,
    GridSettings,
    BackgroundSettings,
}

impl std::fmt::Display for EditorTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorTab::Viewport => write!(f, "Viewport"),
            EditorTab::Inspector => write!(f, "Inspector"),
            EditorTab::Hierarchy => write!(f, "Hierarchy"),
            EditorTab::AssetBrowser => write!(f, "Asset Browser"),
            EditorTab::Console => write!(f, "Console"),
            EditorTab::SceneSettings => write!(f, "Scene Settings"),
            EditorTab::GameControls => write!(f, "Game Controls"),
            EditorTab::EntitySpawner => write!(f, "Entity Spawner"),
            EditorTab::AssetManager => write!(f, "Asset Manager"),
            EditorTab::GridSettings => write!(f, "Grid Settings"),
            EditorTab::BackgroundSettings => write!(f, "Background Settings"),
        }
    }
}

#[derive(Resource)]
pub struct DockTree {
    pub state: DockState<EditorTab>,
}

impl Default for DockTree {
    fn default() -> Self {
        Self::create_professional_layout()
    }
}

impl DockTree {
    /// Creates a professional 4-quadrant layout similar to Unity/Unreal Engine
    pub fn create_professional_layout() -> Self {
        use egui_dock::*;
        
        // Create the main dock state starting with viewport as the center
        let mut dock_state = DockState::new(vec![EditorTab::Viewport]);
        
        // Add essential panels in a logical order
        // Left side: Hierarchy and Entity Spawner
        dock_state.push_to_focused_leaf(EditorTab::Hierarchy);
        dock_state.push_to_focused_leaf(EditorTab::EntitySpawner);
        
        // Right side: Inspector and Asset Browser
        dock_state.push_to_focused_leaf(EditorTab::Inspector);
        dock_state.push_to_focused_leaf(EditorTab::AssetBrowser);
        
        // Bottom: Console and Game Controls
        dock_state.push_to_focused_leaf(EditorTab::Console);
        dock_state.push_to_focused_leaf(EditorTab::GameControls);
        
        // Additional panels as floating tabs
        dock_state.push_to_focused_leaf(EditorTab::SceneSettings);
        dock_state.push_to_focused_leaf(EditorTab::GridSettings);
        dock_state.push_to_focused_leaf(EditorTab::BackgroundSettings);
        dock_state.push_to_focused_leaf(EditorTab::AssetManager);
        
        Self { state: dock_state }
    }
    
    /// Creates a minimal layout for focused work
    pub fn create_minimal_layout() -> Self {
        use egui_dock::*;
        
        let mut dock_state = DockState::new(vec![EditorTab::Viewport]);
        dock_state.push_to_focused_leaf(EditorTab::Hierarchy);
        dock_state.push_to_focused_leaf(EditorTab::Inspector);
        dock_state.push_to_focused_leaf(EditorTab::Console);
        
        Self { state: dock_state }
    }
    
    /// Creates a debugging focused layout
    pub fn create_debug_layout() -> Self {
        use egui_dock::*;
        
        let mut dock_state = DockState::new(vec![EditorTab::Viewport]);
        dock_state.push_to_focused_leaf(EditorTab::Hierarchy);
        dock_state.push_to_focused_leaf(EditorTab::Inspector);
        dock_state.push_to_focused_leaf(EditorTab::Console);
        dock_state.push_to_focused_leaf(EditorTab::GameControls);
        dock_state.push_to_focused_leaf(EditorTab::AssetManager);
        
        Self { state: dock_state }
    }
    
    /// Creates a scene design focused layout
    pub fn create_scene_design_layout() -> Self {
        use egui_dock::*;
        
        let mut dock_state = DockState::new(vec![EditorTab::Viewport]);
        dock_state.push_to_focused_leaf(EditorTab::Hierarchy);
        dock_state.push_to_focused_leaf(EditorTab::AssetBrowser);
        dock_state.push_to_focused_leaf(EditorTab::Inspector);
        dock_state.push_to_focused_leaf(EditorTab::SceneSettings);
        dock_state.push_to_focused_leaf(EditorTab::GridSettings);
        dock_state.push_to_focused_leaf(EditorTab::BackgroundSettings);
        
        Self { state: dock_state }
    }
    
    /// Reset to the default professional layout
    pub fn reset_to_professional_layout(&mut self) {
        *self = Self::create_professional_layout();
    }
}

/// Layout management resource for saving and loading dock layouts
#[derive(Resource, Default)]
pub struct LayoutManager {
    pub layouts_directory: String,
    pub current_layout_name: String,
    pub available_layouts: Vec<String>,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            layouts_directory: "layouts".to_string(),
            current_layout_name: "default".to_string(),
            available_layouts: vec![
                "Professional".to_string(),
                "Minimal".to_string(),
                "Scene Design".to_string(),
                "Debug".to_string(),
            ],
        }
    }
    
    pub fn save_layout(&self, _dock_tree: &DockTree, layout_name: &str) {
        // In a full implementation, this would serialize the dock tree to disk
        info!("Saving layout: {}", layout_name);
    }
    
    pub fn load_layout(&self, layout_name: &str) -> Option<DockTree> {
        // In a full implementation, this would load the dock tree from disk
        info!("Loading layout: {}", layout_name);
        match layout_name {
            "Professional" => Some(DockTree::create_professional_layout()),
            "Minimal" => Some(DockTree::create_minimal_layout()),
            "Scene Design" => Some(DockTree::create_scene_design_layout()),
            "Debug" => Some(DockTree::create_debug_layout()),
            _ => None,
        }
    }
}