//! ECS Components and serialization types

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Player component marker
#[derive(Component)]
pub struct Player;

/// Enemy component marker
#[derive(Component)]
pub struct Enemy;

/// Projectile component with velocity
#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
}

/// Shooting component with cooldown timer
#[derive(Component)]
pub struct Shooting {
    pub cooldown: f32,
}

/// Health component
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// Collision component with radius
#[derive(Component)]
pub struct Collision {
    pub radius: f32,
}

/// Selection component marker for selected entities
#[derive(Component)]
pub struct Selected;

/// Grid line component marker
#[derive(Component)]
pub struct GridLine;

/// Background image component marker
#[derive(Component)]
pub struct BackgroundImage;

/// Sprite asset component for entities with custom textures
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct SpriteAsset {
    pub asset_path: Option<String>,
    pub tint_color: [f32; 4], // RGBA values for serialization
    pub scale: [f32; 2], // Vec2 as array for serialization
}

impl Default for SpriteAsset {
    fn default() -> Self {
        Self {
            asset_path: None,
            tint_color: [1.0, 1.0, 1.0, 1.0], // White
            scale: [1.0, 1.0],
        }
    }
}

impl SpriteAsset {
    pub fn new(asset_path: Option<String>) -> Self {
        Self {
            asset_path,
            tint_color: [1.0, 1.0, 1.0, 1.0],
            scale: [1.0, 1.0],
        }
    }
    
    pub fn get_color(&self) -> Color {
        Color::rgba(self.tint_color[0], self.tint_color[1], self.tint_color[2], self.tint_color[3])
    }
    
    pub fn set_color(&mut self, color: Color) {
        self.tint_color = [color.r(), color.g(), color.b(), color.a()];
    }
    
    pub fn get_scale(&self) -> Vec2 {
        Vec2::new(self.scale[0], self.scale[1])
    }
    
    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = [scale.x, scale.y];
    }
}

/// Entity types for spawning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Enemy,
    Projectile,
}

/// Serializable transform for scene saving/loading
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableTransform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl From<Transform> for SerializableTransform {
    fn from(transform: Transform) -> Self {
        Self {
            x: transform.translation.x,
            y: transform.translation.y,
            z: transform.translation.z,
            rotation: transform.rotation.to_euler(EulerRot::ZYX).0,
            scale_x: transform.scale.x,
            scale_y: transform.scale.y,
        }
    }
}

impl From<SerializableTransform> for Transform {
    fn from(st: SerializableTransform) -> Self {
        Transform {
            translation: Vec3::new(st.x, st.y, st.z),
            rotation: Quat::from_rotation_z(st.rotation),
            scale: Vec3::new(st.scale_x, st.scale_y, 1.0),
        }
    }
}

/// Serializable entity for scene saving/loading
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableEntity {
    pub entity_type: EntityType,
    pub transform: SerializableTransform,
    pub health: Option<(f32, f32)>, // (current, max)
    pub collision_radius: Option<f32>,
    pub sprite_asset: Option<SpriteAsset>,
}