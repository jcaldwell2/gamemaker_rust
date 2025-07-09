//! Asset Browser UI Panel

use bevy::prelude::*;
use bevy_egui::egui;
use std::path::PathBuf;

use crate::resources::{AssetRegistry, AssetImporter, AssetBrowserState};
use crate::components::SpriteAsset;

/// Asset browser UI panel content
pub fn render_asset_browser_content(
    ui: &mut egui::Ui,
    asset_registry: &AssetRegistry,
    asset_importer: &mut AssetImporter,
    browser_state: &mut AssetBrowserState,
) {
    // Toolbar
    ui.horizontal(|ui| {
        if ui.button("Import Asset").clicked() {
            browser_state.show_import_dialog = true;
        }
        
        if ui.button("Refresh").clicked() {
            // Trigger asset registry refresh
            info!("Refreshing asset registry");
        }
        
        ui.separator();
        
        ui.label("Filter:");
        ui.text_edit_singleline(&mut browser_state.filter_text);
    });
    
    ui.separator();
    
    // Import dialog
    if browser_state.show_import_dialog {
        import_dialog_ui(ui, asset_importer, browser_state);
        ui.separator();
    }
    
    // Asset list
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            asset_list_ui(ui, asset_registry, browser_state);
        });
        
    // Asset details
    if let Some(selected) = &browser_state.selected_asset {
        ui.separator();
        asset_details_ui(ui, asset_registry, selected);
    }
}

/// Import dialog UI
fn import_dialog_ui(
    ui: &mut egui::Ui,
    asset_importer: &mut AssetImporter,
    browser_state: &mut AssetBrowserState,
) {
    ui.group(|ui| {
        ui.label("Import Asset");
        
        ui.horizontal(|ui| {
            ui.label("Path:");
            ui.text_edit_singleline(&mut browser_state.import_path);
            
            if ui.button("Browse").clicked() {
                // Open file dialog
                if let Some(path) = open_file_dialog() {
                    browser_state.import_path = path;
                }
            }
        });
        
        ui.horizontal(|ui| {
            if ui.button("Import").clicked() {
                if !browser_state.import_path.is_empty() {
                    // Queue the asset for import
                    asset_importer.queue_import(browser_state.import_path.clone());
                    browser_state.import_path.clear();
                    browser_state.show_import_dialog = false;
                }
            }
            
            if ui.button("Cancel").clicked() {
                browser_state.show_import_dialog = false;
                browser_state.import_path.clear();
            }
        });
    });
}

/// Asset list UI
fn asset_list_ui(
    ui: &mut egui::Ui,
    asset_registry: &AssetRegistry,
    browser_state: &mut AssetBrowserState,
) {
    for (asset_path, metadata) in &asset_registry.asset_metadata {
        // Apply filter
        if !browser_state.filter_text.is_empty() 
            && !asset_path.to_lowercase().contains(&browser_state.filter_text.to_lowercase()) {
            continue;
        }
        
        let is_selected = browser_state.selected_asset.as_ref() == Some(asset_path);
        
        ui.horizontal(|ui| {
            // Asset thumbnail placeholder
            let thumbnail_size = egui::Vec2::new(32.0, 32.0);
            let (rect, _) = ui.allocate_exact_size(thumbnail_size, egui::Sense::click());
            
            // Draw thumbnail background
            ui.painter().rect_filled(
                rect,
                egui::Rounding::same(4.0),
                if is_selected {
                    egui::Color32::from_rgb(100, 150, 255)
                } else {
                    egui::Color32::from_rgb(60, 60, 60)
                },
            );
            
            // Asset name and info
            ui.vertical(|ui| {
                let path_buf = PathBuf::from(asset_path);
                let file_name = path_buf
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Unknown");
                
                if ui.selectable_label(is_selected, file_name).clicked() {
                    browser_state.selected_asset = Some(asset_path.clone());
                }
                
                if let Some((width, height)) = metadata.image_dimensions {
                    ui.label(format!("{}x{}", width, height));
                } else {
                    ui.label("Unknown dimensions");
                }
                ui.label(format!("{:.1} KB", metadata.file_size as f32 / 1024.0));
            });
        });
        
        ui.separator();
    }
    
    // Show message if no assets
    if asset_registry.asset_metadata.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label("No assets loaded\nClick 'Import Asset' to add assets");
        });
    }
}

/// Asset details UI
fn asset_details_ui(
    ui: &mut egui::Ui,
    asset_registry: &AssetRegistry,
    selected_asset: &str,
) {
    if let Some(metadata) = asset_registry.asset_metadata.get(selected_asset) {
        ui.group(|ui| {
            ui.label("Asset Details");
            
            ui.horizontal(|ui| {
                ui.label("Path:");
                ui.label(selected_asset);
            });
            
            ui.horizontal(|ui| {
                ui.label("Size:");
                if let Some((width, height)) = metadata.image_dimensions {
                    ui.label(format!("{}x{}", width, height));
                } else {
                    ui.label("Unknown dimensions");
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("File Size:");
                ui.label(format!("{:.1} KB", metadata.file_size as f32 / 1024.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Format:");
                ui.label("Image"); // Default format since we don't store this in metadata
            });
            
            ui.horizontal(|ui| {
                ui.label("Imported:");
                ui.label(&metadata.import_date);
            });
            
            ui.separator();
            
            // Asset actions
            ui.horizontal(|ui| {
                if ui.button("Apply to Selected").clicked() {
                    info!("Applying asset {} to selected entity", selected_asset);
                    // This would be handled by the main editor system
                }
                
                if ui.button("Remove").clicked() {
                    info!("Removing asset {}", selected_asset);
                    // This would remove the asset from the registry
                }
            });
        });
    }
}

/// Open file dialog for asset import
fn open_file_dialog() -> Option<String> {
    use rfd::FileDialog;
    
    let file = FileDialog::new()
        .add_filter("Image Files", &["png", "jpg", "jpeg", "gif", "bmp"])
        .set_directory("assets/sprites")
        .pick_file();
    
    file.map(|path| path.to_string_lossy().to_string())
}

/// System to apply assets to selected entities
pub fn apply_asset_to_entity_system(
    mut commands: Commands,
    asset_registry: Res<AssetRegistry>,
    browser_state: Res<AssetBrowserState>,
    selected_entity: Option<Res<crate::resources::SelectedEntity>>,
) {
    // This system would handle applying selected assets to entities
    // Implementation would depend on how entity selection is handled
    if let (Some(selected_entity), Some(selected_asset)) = (selected_entity, &browser_state.selected_asset) {
        if let Some(_metadata) = asset_registry.asset_metadata.get(selected_asset) {
            info!("Would apply asset {} to entity {:?}", selected_asset, selected_entity.entity);
            
            // Create SpriteAsset component
            let sprite_asset = SpriteAsset {
                asset_path: Some(selected_asset.clone()),
                tint_color: [1.0, 1.0, 1.0, 1.0], // White tint
                scale: [1.0, 1.0], // Default scale
            };
            
            // Apply to entity (placeholder - would need actual entity reference)
            // commands.entity(selected_entity.entity).insert(sprite_asset);
        }
    }
}