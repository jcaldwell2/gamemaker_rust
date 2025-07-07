# Changelog

All notable changes to GameMaker Rust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-07-06

### Added
- **Dynamic Component Management**: Add/remove components (Player, Enemy, Shooter, Collider, Health, CustomSprite) via inspector UI
- **Collision Detection System**: AABB collision between projectiles and enemies with health/damage mechanics
- **Health System**: Entities have health points, take damage, and are destroyed when health reaches zero
- **Boundary Collision**: Entities are kept within screen bounds during gameplay
- **Component Add/Remove UI**: Unity/Godot-style component management in inspector panel

### Enhanced
- **Inspector Panel**: Now shows component management buttons with tooltips
- **Console Logging**: Added detailed feedback for component operations and collisions
- **System Architecture**: Split systems into smaller groups to handle Bevy's parameter limits

### Fixed
- **System Parameter Limits**: Resolved Bevy system parameter count limitations
- **UI Rendering**: Fixed egui central panel covering game viewport
- **Entity Visibility**: Ensured entities are properly positioned and visible

## [0.2.0] - 2025-07-06

### Added
- **Camera Focus System**: 🎯 buttons in hierarchy to focus camera on entities
- **WASD Camera Controls**: Manual camera movement in editor mode
- **Smooth Camera Movement**: Interpolated camera transitions and entity following
- **Right-click Context Menus**: Entity management via right-click in hierarchy
- **Camera Control Buttons**: Reset camera and focus player buttons in game controls

### Enhanced
- **Camera Controller Resource**: Centralized camera state management
- **Entity Positioning**: Updated spawn positions for better visibility
- **UI Layout**: Improved game controls panel with camera options

### Fixed
- **Camera Positioning**: Proper camera setup and target positioning
- **Entity Focus**: Reliable camera focusing on selected entities

## [0.1.0] - 2025-07-06

### Added
- **Core Engine**: Bevy ECS-based 2D game engine foundation
- **Editor Interface**: egui-based editor with left panel layout
- **Scene Management**: Save/load scenes using RON serialization
- **Entity System**: Player, Enemy, and CustomSprite entities with components
- **Asset Management**: Texture loading from assets/sprites directory
- **Game Controls**: Play/Pause/Stop functionality with status indicators
- **Player Movement**: Arrow key controls during gameplay
- **Shooting System**: Spacebar shooting with projectile physics and cooldowns
- **Entity Selection**: Click to select, drag to move entities in editor
- **Inspector Panel**: Edit entity properties (transform, sprite, color, size)
- **Scene Hierarchy**: List and manage all scene entities
- **Entity Spawner**: Create new entities with different types and positions
- **Visual Effects**: Enemy color animation and selection highlighting

### Technical
- **Bevy Integration**: Built on Bevy 0.13.2 game engine
- **egui UI**: Immediate-mode GUI for editor interface
- **RON Serialization**: Human-readable scene file format
- **Component Architecture**: Modular entity-component system
- **Resource Management**: Efficient state management with Bevy resources

---

## Version History Summary

- **v0.3.0**: Dynamic component management and collision detection
- **v0.2.0**: Advanced camera system with focus and smooth movement
- **v0.1.0**: Core engine foundation with basic editor and gameplay