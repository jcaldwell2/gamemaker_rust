//! GameMaker Rust - A 2D game engine and editor built with Bevy and egui

use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

pub mod components;
pub mod resources;
pub mod systems;
pub mod ui;
pub mod scene;
pub mod assets;
pub mod utils;

pub use components::*;
pub use resources::*;

/// Main plugin that sets up the entire game engine
pub struct GameEnginePlugin;

impl Plugin for GameEnginePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<GameState>()
            .init_resource::<EditorSceneState>()
            .init_resource::<CameraController>()
            .init_resource::<SelectedEntity>()
            .init_resource::<DragState>()
            .init_resource::<AssetImporter>()
            .init_resource::<AssetRegistry>()
            .init_resource::<AssetBrowserState>()
            .init_resource::<ShootingStats>()
            .init_resource::<ProjectManager>()
            .init_resource::<EditorState>()
            .init_resource::<GridSettings>()
            .init_resource::<GridState>()
            .init_resource::<BackgroundSettings>()
            .init_resource::<SceneManager>()
            .init_resource::<DockTree>()
            .init_resource::<LayoutManager>()
            
            // Add events
            .add_event::<MouseWheel>()
            
            // Add startup systems
            .add_systems(Startup, (
                systems::setup_engine,
                assets::load_default_assets,
            ))
            
            // Add update systems - Input and Camera
            .add_systems(Update, (
                systems::input::player_movement,
                systems::input::mouse_interaction,
                systems::input::entity_dragging,
                systems::input::camera_controls,
                systems::input::handle_mouse_wheel_zoom,
                systems::camera::camera_movement,
                systems::camera::update_mouse_world_position,
            ))
            
            // Add update systems - Gameplay
            .add_systems(Update, (
                systems::gameplay::player_shooting,
                systems::gameplay::projectile_movement,
                systems::gameplay::projectile_cleanup,
                systems::gameplay::update_shooting_cooldowns,
                systems::gameplay::collision_detection,
                systems::gameplay::boundary_collision,
                systems::gameplay::enemy_color_change,
                systems::game_controls::game_controls_system,
                systems::game_controls::handle_play_mode_transition,
            ))
            
            // Add update systems - Rendering and Editor
            .add_systems(Update, (
                systems::rendering::render_grid_overlay,
                systems::rendering::update_background_image,
                systems::rendering::update_selection_visuals,
                systems::editor::editor_update,
                systems::editor::debug_info_system,
                systems::editor::entity_spawn_system,
            ))
            
            // Add update systems - Assets and UI
            .add_systems(Update, (
                assets::handle_asset_imports,
                assets::load_background_image,
                ui::dockable_ui_system,
                ui::asset_browser::apply_asset_to_entity_system,
            ));
    }
}