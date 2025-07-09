//! Utility functions and helpers

use bevy::prelude::*;

/// Math utilities
pub mod math {
    use super::*;
    
    /// Clamp a value between min and max
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        value.max(min).min(max)
    }
    
    /// Linear interpolation between two values
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
    
    /// Calculate distance between two 2D points
    pub fn distance_2d(a: Vec2, b: Vec2) -> f32 {
        (a - b).length()
    }
    
    /// Check if a point is within a circle
    pub fn point_in_circle(point: Vec2, center: Vec2, radius: f32) -> bool {
        distance_2d(point, center) <= radius
    }
    
    /// Normalize an angle to be between -PI and PI
    pub fn normalize_angle(angle: f32) -> f32 {
        let mut normalized = angle;
        while normalized > std::f32::consts::PI {
            normalized -= 2.0 * std::f32::consts::PI;
        }
        while normalized < -std::f32::consts::PI {
            normalized += 2.0 * std::f32::consts::PI;
        }
        normalized
    }
}

/// Color utilities
pub mod color {
    use super::*;
    
    /// Convert HSV to RGB color
    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        Color::rgb(r + m, g + m, b + m)
    }
    
    /// Create a color with alpha
    pub fn with_alpha(color: Color, alpha: f32) -> Color {
        Color::rgba(color.r(), color.g(), color.b(), alpha)
    }
    
    /// Blend two colors
    pub fn blend(color1: Color, color2: Color, factor: f32) -> Color {
        let t = factor.clamp(0.0, 1.0);
        Color::rgb(
            color1.r() * (1.0 - t) + color2.r() * t,
            color1.g() * (1.0 - t) + color2.g() * t,
            color1.b() * (1.0 - t) + color2.b() * t,
        )
    }
}

/// Transform utilities
pub mod transform {
    use super::*;
    
    /// Create a transform at a 2D position
    pub fn at_position_2d(position: Vec2) -> Transform {
        Transform::from_xyz(position.x, position.y, 0.0)
    }
    
    /// Create a transform at a 3D position
    pub fn at_position_3d(position: Vec3) -> Transform {
        Transform::from_translation(position)
    }
    
    /// Create a transform with position and scale
    pub fn with_position_and_scale(position: Vec2, scale: f32) -> Transform {
        Transform::from_xyz(position.x, position.y, 0.0).with_scale(Vec3::splat(scale))
    }
    
    /// Get the 2D position from a transform
    pub fn get_position_2d(transform: &Transform) -> Vec2 {
        transform.translation.truncate()
    }
    
    /// Set the 2D position of a transform
    pub fn set_position_2d(transform: &mut Transform, position: Vec2) {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

/// Time utilities
pub mod time {
    /// Convert seconds to milliseconds
    pub fn seconds_to_ms(seconds: f32) -> f32 {
        seconds * 1000.0
    }
    
    /// Convert milliseconds to seconds
    pub fn ms_to_seconds(ms: f32) -> f32 {
        ms / 1000.0
    }
    
    /// Format time as MM:SS
    pub fn format_time(seconds: f32) -> String {
        let minutes = (seconds / 60.0) as u32;
        let seconds = (seconds % 60.0) as u32;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

/// Debug utilities
pub mod debug {
    use super::*;
    
    /// Print entity information
    pub fn print_entity_info(
        entity: Entity,
        transform: Option<&Transform>,
        name: Option<&str>,
    ) {
        let name = name.unwrap_or("Unknown");
        if let Some(transform) = transform {
            println!(
                "Entity {} ({}): pos=({:.2}, {:.2}, {:.2}), scale=({:.2}, {:.2}, {:.2})",
                entity.index(),
                name,
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
                transform.scale.x,
                transform.scale.y,
                transform.scale.z
            );
        } else {
            println!("Entity {} ({}): no transform", entity.index(), name);
        }
    }
    
    /// Draw a debug circle (would need gizmos in a real implementation)
    pub fn debug_circle(center: Vec2, radius: f32, color: Color) {
        println!(
            "Debug Circle: center=({:.2}, {:.2}), radius={:.2}, color=({:.2}, {:.2}, {:.2})",
            center.x, center.y, radius, color.r(), color.g(), color.b()
        );
    }
}