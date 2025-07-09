# Changelog

All notable changes to GameMaker Rust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-07-09

### Added
- **Dockable UI System**: Complete refactoring of the editor UI from floating windows to a professional dockable panel system
  - Integrated `egui_dock` for advanced docking capabilities
  - Implemented 11 specialized panel types: Hierarchy, Inspector, Scene View, Asset Browser, Console, Layers, Animation, Tilemap, Game View, Scripting, and Properties
  - Added professional layout management with drag-and-drop panel rearrangement
  - Implemented panel resize and flexible workspace organization
  - Added comprehensive panel state management and persistence

### Changed
- **UI Architecture**: Migrated from basic egui windows to `egui_dock::DockArea` system
- **Panel Management**: Replaced individual window toggles with unified dockable panel system
- **Editor Layout**: Transformed editor into professional IDE-like interface with customizable workspace

### Fixed
- **Compatibility**: Resolved egui_dock compatibility issues with current Bevy and egui versions
- **Lifetime Management**: Fixed lifetime and borrowing issues in UI component integration
- **Panel State**: Improved panel state synchronization and update handling

### Technical Improvements
- Enhanced UI component modularity and separation of concerns
- Improved editor state management and resource handling
- Better integration between editor panels and game engine systems
- Optimized panel rendering and update performance

### User Experience
- Professional IDE-like editor interface
- Intuitive drag-and-drop panel management
- Flexible workspace customization
- Improved visual hierarchy and organization
- Better separation between different editor functions

## [0.3.1] - 2025-07-09

### Added
- **Unified Menu Bar**: Combined menu bar and game controls into single top toolbar
- **Professional Game Controls**: Play/Pause/Stop buttons with real-time stats display
- **Enhanced Asset Browser**: Complete asset management system with Phase 2 implementation
- **Grid Toggle Controls**: Dual grid controls in View menu for rendering and settings

### Enhanced
- **UI Layout**: Streamlined interface with unified top panel and clean bottom status bar
- **Grid Rendering System**: Improved grid display with forced updates when re-enabling
- **Camera Controls**: Right-click and drag with instant movement during operations
- **Menu System**: Direct control over grid rendering through View menu options

### Fixed
- **Grid Toggle Functionality**: Fixed issue where grid wouldn't reappear after being hidden
- **Navigation Bar Flickering**: Resolved conflicting top panels that caused UI flickering
- **Grid State Management**: Enhanced grid system to properly detect when updates are needed
- **UI Panel Conflicts**: Eliminated duplicate TopBottomPanel systems causing render conflicts
- **Component Imports**: Added missing component imports for proper compilation
- **Camera Bounce**: Removed smooth interpolation during drag operations for instant response

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