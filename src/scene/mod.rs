//! Scene management and serialization

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::components::*;
use crate::resources::*;

/// Scene data structure for serialization
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scene {
    pub entities: Vec<SerializableEntity>,
    pub metadata: SceneMetadata,
}

/// Scene metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneMetadata {
    pub name: String,
    pub version: String,
    pub created_at: String,
    pub last_modified: String,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            metadata: SceneMetadata {
                name: "Untitled Scene".to_string(),
                version: "1.0".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                last_modified: chrono::Utc::now().to_rfc3339(),
            },
        }
    }
}

/// Save the current scene to a file
pub fn save_scene(
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Health>, Option<&Collision>, Option<&SpriteAsset>)>,
    save_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut scene = Scene::default();
    
    // Collect all entities
    for (_, transform, player, enemy, health, collision, sprite_asset) in entity_query.iter() {
        let entity_type = if player.is_some() {
            EntityType::Player
        } else if enemy.is_some() {
            EntityType::Enemy
        } else {
            continue; // Skip non-game entities
        };
        
        let serializable_entity = SerializableEntity {
            entity_type,
            transform: SerializableTransform::from(*transform),
            health: health.map(|h| (h.current, h.max)),
            collision_radius: collision.map(|c| c.radius),
            sprite_asset: sprite_asset.cloned(),
        };
        
        scene.entities.push(serializable_entity);
    }
    
    // Update metadata
    scene.metadata.last_modified = chrono::Utc::now().to_rfc3339();
    
    // Serialize to RON format
    let ron_string = ron::ser::to_string_pretty(&scene, ron::ser::PrettyConfig::default())?;
    
    // Ensure directory exists
    if let Some(parent) = std::path::Path::new(save_path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write to file
    fs::write(save_path, ron_string)?;
    
    println!("Scene saved to: {}", save_path);
    Ok(())
}

/// Load a scene from a file
pub fn load_scene(
    commands: &mut Commands,
    load_path: &str,
) -> Result<Scene, Box<dyn std::error::Error>> {
    // Read file
    let ron_string = fs::read_to_string(load_path)?;
    
    // Deserialize from RON format
    let scene: Scene = ron::de::from_str(&ron_string)?;
    
    // Spawn entities from scene
    for entity_data in &scene.entities {
        spawn_entity_from_data(commands, entity_data);
    }
    
    println!("Scene loaded from: {}", load_path);
    Ok(scene)
}

/// Spawn an entity from serialized data
pub fn spawn_entity_from_data(
    commands: &mut Commands,
    entity_data: &SerializableEntity,
) {
    let transform = Transform::from(entity_data.transform.clone());
    
    // Determine color and scale based on entity type and sprite asset
    let (default_color, default_scale) = match entity_data.entity_type {
        EntityType::Player => (Color::BLUE, Vec3::splat(50.0)),
        EntityType::Enemy => (Color::RED, Vec3::splat(40.0)),
        EntityType::Projectile => (Color::YELLOW, Vec3::new(5.0, 15.0, 1.0)),
    };
    
    let sprite_color = if let Some(sprite_asset) = &entity_data.sprite_asset {
        sprite_asset.get_color()
    } else {
        default_color
    };
    
    let sprite_scale = if let Some(sprite_asset) = &entity_data.sprite_asset {
        let asset_scale = sprite_asset.get_scale();
        Vec3::new(default_scale.x * asset_scale.x, default_scale.y * asset_scale.y, default_scale.z)
    } else {
        default_scale
    };
    
    match entity_data.entity_type {
        EntityType::Player => {
            let mut entity_commands = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: sprite_color,
                        ..default()
                    },
                    transform: transform.with_scale(sprite_scale),
                    ..default()
                },
                Player,
                Shooting { cooldown: 0.0 },
            ));
            
            if let Some(sprite_asset) = &entity_data.sprite_asset {
                entity_commands.insert(sprite_asset.clone());
            }
            
            if let Some((current, max)) = entity_data.health {
                entity_commands.insert(Health { current, max });
            } else {
                entity_commands.insert(Health { current: 100.0, max: 100.0 });
            }
            
            if let Some(radius) = entity_data.collision_radius {
                entity_commands.insert(Collision { radius });
            } else {
                entity_commands.insert(Collision { radius: 25.0 });
            }
        },
        EntityType::Enemy => {
            let mut entity_commands = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: sprite_color,
                        ..default()
                    },
                    transform: transform.with_scale(sprite_scale),
                    ..default()
                },
                Enemy,
            ));
            
            if let Some(sprite_asset) = &entity_data.sprite_asset {
                entity_commands.insert(sprite_asset.clone());
            }
            
            if let Some((current, max)) = entity_data.health {
                entity_commands.insert(Health { current, max });
            } else {
                entity_commands.insert(Health { current: 50.0, max: 50.0 });
            }
            
            if let Some(radius) = entity_data.collision_radius {
                entity_commands.insert(Collision { radius });
            } else {
                entity_commands.insert(Collision { radius: 20.0 });
            }
        },
        EntityType::Projectile => {
            let mut entity_commands = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: sprite_color,
                        ..default()
                    },
                    transform: transform.with_scale(sprite_scale),
                    ..default()
                },
                Projectile {
                    velocity: Vec2::new(0.0, 400.0),
                },
            ));
            
            if let Some(sprite_asset) = &entity_data.sprite_asset {
                entity_commands.insert(sprite_asset.clone());
            }
            
            if let Some(radius) = entity_data.collision_radius {
                entity_commands.insert(Collision { radius });
            } else {
                entity_commands.insert(Collision { radius: 5.0 });
            }
        },
    }
}

/// Spawn a new entity at the specified position
pub fn spawn_entity(
    commands: &mut Commands,
    entity_type: EntityType,
    position: Vec2,
    z_position: Option<f32>,
) {
    let z = z_position.unwrap_or(0.0);
    let transform = Transform::from_xyz(position.x, position.y, z);
    
    let entity_data = SerializableEntity {
        entity_type,
        transform: SerializableTransform::from(transform),
        health: match entity_type {
            EntityType::Player => Some((100.0, 100.0)),
            EntityType::Enemy => Some((50.0, 50.0)),
            EntityType::Projectile => None,
        },
        collision_radius: match entity_type {
            EntityType::Player => Some(25.0),
            EntityType::Enemy => Some(20.0),
            EntityType::Projectile => Some(5.0),
        },
        sprite_asset: None, // Default to no custom sprite
    };
    
    spawn_entity_from_data(commands, &entity_data);
}

/// Save the current scene to a RON string
pub fn save_scene_to_string(
    entity_query: &Query<(Entity, &Transform, Option<&Player>, Option<&Enemy>, Option<&Projectile>, Option<&Health>, Option<&Collision>, Option<&SpriteAsset>), (Without<Camera>, Without<GridLine>, Without<BackgroundImage>)>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut scene = Scene::default();
    
    // Collect all entities
    for (_, transform, player, enemy, projectile, health, collision, sprite_asset) in entity_query.iter() {
        let entity_type = if player.is_some() {
            EntityType::Player
        } else if enemy.is_some() {
            EntityType::Enemy
        } else if projectile.is_some() {
            EntityType::Projectile
        } else {
            continue; // Skip non-game entities
        };
        
        let serializable_entity = SerializableEntity {
            entity_type,
            transform: SerializableTransform::from(*transform),
            health: health.map(|h| (h.current, h.max)),
            collision_radius: collision.map(|c| c.radius),
            sprite_asset: sprite_asset.cloned(),
        };
        
        scene.entities.push(serializable_entity);
    }
    
    // Update metadata
    scene.metadata.last_modified = chrono::Utc::now().to_rfc3339();
    
    // Serialize to RON format
    let ron_string = ron::ser::to_string_pretty(&scene, ron::ser::PrettyConfig::default())?;
    
    Ok(ron_string)
}

/// Load a scene from a RON string
pub fn load_scene_from_string(
    commands: &mut Commands,
    ron_string: &str,
) -> Result<Scene, Box<dyn std::error::Error>> {
    // Deserialize from RON format
    let scene: Scene = ron::de::from_str(ron_string)?;
    
    // Spawn entities from scene
    for entity_data in &scene.entities {
        spawn_entity_from_data(commands, entity_data);
    }
    
    Ok(scene)
}