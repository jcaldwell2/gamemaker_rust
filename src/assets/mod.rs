//! Enhanced asset management and loading system

use bevy::prelude::*;
use std::path::Path;
use std::fs;

use crate::components::*;
use crate::resources::*;

/// Load default assets for the engine
pub fn load_default_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_registry: ResMut<AssetRegistry>,
) {
    info!("Loading default assets...");
    
    // Try to load default assets from the assets folder
    let default_assets = [
        "sprites/player.png",
        "sprites/enemy.png",
        "sprites/projectile.png",
        "backgrounds/default.png",
    ];
    
    for asset_path in default_assets.iter() {
        if let Err(err) = try_load_asset(&asset_server, &mut asset_registry, asset_path) {
            warn!("Failed to load default asset '{}': {}", asset_path, err);
        }
    }
    
    info!("Default asset loading completed");
}

/// Try to load an asset, handling errors gracefully
fn try_load_asset(
    asset_server: &AssetServer,
    asset_registry: &mut AssetRegistry,
    asset_path: &str,
) -> Result<(), String> {
    // Check if the file exists
    let full_path = format!("assets/{}", asset_path);
    if !Path::new(&full_path).exists() {
        return Err(format!("File not found: {}", full_path));
    }
    
    // Load the asset
    let handle: Handle<Image> = asset_server.load(asset_path.to_owned());
    
    // Create metadata
    let metadata = create_asset_metadata(asset_path, &full_path)?;
    
    // Register in loading state
    asset_registry.start_loading(asset_path.to_string(), handle);
    
    info!("Started loading asset: {}", asset_path);
    Ok(())
}

/// Create metadata for an asset
fn create_asset_metadata(asset_path: &str, full_path: &str) -> Result<AssetMetadata, String> {
    let file_name = Path::new(asset_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let file_size = fs::metadata(full_path)
        .map(|m| m.len())
        .unwrap_or(0);
    
    let now = chrono::Utc::now().to_rfc3339();
    
    Ok(AssetMetadata {
        name: file_name,
        path: asset_path.to_string(),
        file_size,
        image_dimensions: None, // Will be filled when image is loaded
        import_date: now.clone(),
        last_modified: now,
    })
}

/// Handle asset importing from the import queue
pub fn handle_asset_imports(
    mut asset_importer: ResMut<AssetImporter>,
    asset_server: Res<AssetServer>,
    mut asset_registry: ResMut<AssetRegistry>,
) {
    // Process import queue
    let mut to_import = Vec::new();
    to_import.extend(asset_importer.import_queue.drain(..));
    
    for path in to_import {
        match try_load_asset(&asset_server, &mut asset_registry, &path) {
            Ok(()) => {
                asset_importer.start_import(path);
            }
            Err(err) => {
                asset_importer.fail_import(path, err);
            }
        }
    }
    
    // Check loading progress
    let mut completed_imports = Vec::new();
    
    // Collect paths to process to avoid borrowing issues
    let paths_to_process: Vec<String> = asset_importer.pending_imports.iter().cloned().collect();
    
    for path in paths_to_process {
        if let Some(handle) = asset_registry.loading_assets.get(&path) {
            match asset_server.get_load_state(handle) {
                Some(bevy::asset::LoadState::Loaded) => {
                    // Asset loaded successfully
                    if let Some(handle) = asset_registry.loading_assets.get(&path).cloned() {
                        if let Ok(metadata) = create_asset_metadata(&path, &format!("assets/{}", path)) {
                            asset_registry.register_image(path.clone(), handle, metadata);
                        }
                    }
                    asset_registry.finish_loading(&path);
                    completed_imports.push(path.clone());
                    info!("Successfully loaded asset: {}", path);
                }
                Some(bevy::asset::LoadState::Failed) => {
                    // Asset failed to load
                    asset_registry.finish_loading(&path);
                    asset_importer.fail_import(path.clone(), "Failed to load asset".to_string());
                    completed_imports.push(path.clone());
                    warn!("Failed to load asset: {}", path);
                }
                _ => {
                    // Still loading
                }
            }
        }
    }
    
    // Remove completed imports
    for path in completed_imports {
        asset_importer.complete_import(&path);
    }
}

/// Load background image with enhanced asset system
pub fn load_background_image(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_registry: ResMut<AssetRegistry>,
    background_settings: Res<BackgroundSettings>,
    background_query: Query<Entity, With<BackgroundImage>>,
) {
    if background_settings.is_changed() {
        // Remove existing background
        for entity in background_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Load new background if path is provided
        if let Some(ref path) = background_settings.image_path {
            if !path.is_empty() {
                if let Some(handle) = asset_registry.get_image(path) {
                    // Asset already loaded, use it
                    spawn_background_with_texture(
                        &mut commands,
                        handle.clone(),
                        &background_settings,
                    );
                    info!("Background image loaded from registry: {}", path);
                } else if !asset_registry.is_loading(path) {
                    // Asset not loaded, try to load it
                    match try_load_asset(&asset_server, &mut asset_registry, path) {
                        Ok(()) => {
                            // Will be handled by the loading system
                            info!("Started loading background image: {}", path);
                        }
                        Err(err) => {
                            warn!("Failed to load background image '{}': {}", path, err);
                            // Create placeholder
                            spawn_background_placeholder(&mut commands, &background_settings, path);
                        }
                    }
                } else {
                    // Asset is currently loading, create placeholder for now
                    spawn_background_placeholder(&mut commands, &background_settings, path);
                }
            }
        }
    }
}

/// Spawn background with actual texture
fn spawn_background_with_texture(
    commands: &mut Commands,
    texture_handle: Handle<Image>,
    settings: &BackgroundSettings,
) {
    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, settings.opacity),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -10.0)
                .with_scale(Vec3::splat(settings.scale)),
            visibility: if settings.enabled {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
            ..default()
        },
        BackgroundImage,
    ));
}

/// Spawn background placeholder when texture is not available
fn spawn_background_placeholder(
    commands: &mut Commands,
    settings: &BackgroundSettings,
    path: &str,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.5, 0.5, 0.5, settings.opacity),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -10.0)
                .with_scale(Vec3::splat(settings.scale * 100.0)),
            visibility: if settings.enabled {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
            ..default()
        },
        BackgroundImage,
    ));
    
    info!("Background image placeholder created for: {}", path);
}

/// Import asset from file path
pub fn import_asset(
    asset_path: &str,
    asset_importer: &mut AssetImporter,
) -> Result<(), String> {
    // Validate file extension
    let path = Path::new(asset_path);
    if let Some(extension) = path.extension() {
        match extension.to_str() {
            Some("png") | Some("jpg") | Some("jpeg") => {
                // Valid image format
                asset_importer.queue_import(asset_path.to_string());
                Ok(())
            }
            _ => Err(format!("Unsupported file format: {:?}", extension)),
        }
    } else {
        Err("File has no extension".to_string())
    }
}

/// Get asset handle by path
pub fn get_asset_handle(
    asset_registry: &AssetRegistry,
    asset_path: &str,
) -> Option<Handle<Image>> {
    asset_registry.get_image(asset_path).cloned()
}

/// Check if asset is loaded
pub fn is_asset_loaded(
    asset_registry: &AssetRegistry,
    asset_path: &str,
) -> bool {
    asset_registry.is_loaded(asset_path)
}

/// Asset loading utilities
pub mod utils {
    use super::*;
    
    /// Check if an asset is loaded
    pub fn is_asset_loaded<T: Asset>(
        asset_server: &AssetServer,
        handle: &Handle<T>,
    ) -> bool {
        matches!(asset_server.get_load_state(handle), Some(bevy::asset::LoadState::Loaded))
    }
    
    /// Get asset loading progress
    pub fn get_loading_progress(
        asset_server: &AssetServer,
        handles: &[UntypedHandle],
    ) -> f32 {
        if handles.is_empty() {
            return 1.0;
        }
        
        let loaded_count = handles.iter()
            .filter(|handle| {
                matches!(asset_server.get_load_state(*handle), Some(bevy::asset::LoadState::Loaded))
            })
            .count();
        
        loaded_count as f32 / handles.len() as f32
    }
    
    /// Validate image file format
    pub fn is_valid_image_format(path: &str) -> bool {
        if let Some(extension) = Path::new(path).extension() {
            matches!(extension.to_str(), Some("png") | Some("jpg") | Some("jpeg"))
        } else {
            false
        }
    }
    
    /// Get file size in bytes
    pub fn get_file_size(path: &str) -> Result<u64, std::io::Error> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }
    
    /// Format file size for display
    pub fn format_file_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = size as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}