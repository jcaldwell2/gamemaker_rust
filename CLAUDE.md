# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GameMaker Rust is a 2D game engine and editor built with Rust and Bevy. It aims to provide a GameMaker Studio-like experience with an in-app editor, ECS architecture, and support for both 2D and future 3D games.

## Commands

### Build and Run
```bash
cargo run                    # Build and run the game engine/editor
cargo build                  # Build only
cargo build --release        # Build optimized release version
```

### Development
```bash
cargo check                  # Quick syntax/type checking
cargo clippy                 # Linting
cargo fmt                    # Format code
cargo test                   # Run tests
```

## Architecture

The project follows a modular ECS (Entity Component System) architecture using Bevy:

### Core Structure
- **main.rs**: Entry point that sets up Bevy app with DefaultPlugins, EguiPlugin, and GameEnginePlugin
- **lib.rs**: Main plugin definition with all system registration and resource initialization
- **components.rs**: ECS components (Player, Enemy, Projectile, Health, etc.)
- **resources.rs**: Global resources and state management (GameState, CameraController, etc.)

### Module Organization
- **systems/**: Core game systems organized by functionality
  - `input.rs`: Player movement, mouse interaction, entity dragging, camera controls
  - `gameplay.rs`: Shooting, projectile movement, collision detection, enemy behavior
  - `camera.rs`: Camera movement and world position tracking
  - `editor.rs`: Editor functionality, entity spawning, debug info
  - `rendering.rs`: Grid overlay, background rendering, selection visuals
  - `game_controls.rs`: Game mode transitions, play/pause controls
- **ui/**: Editor UI components using egui
  - `editor.rs`: Main editor panels and interface
  - `hierarchy.rs`: Scene hierarchy view
  - `inspector.rs`: Entity inspector
  - `menus.rs`: Menu system
- **scene/**: Scene management and serialization
- **assets/**: Asset loading and management
- **utils.rs**: Utility functions

### Key Resources
- `GameState`: Main game state (paused, debug_mode, playing, editor_mode)
- `EditorSceneState`: Scene serialization data for editor
- `CameraController`: Camera position, zoom, and following logic
- `SelectedEntity`: Currently selected entity in editor
- `DragState`: Entity dragging state
- `AssetImporter`: Asset loading management
- `ProjectManager`: Project file management
- `GridSettings`/`GridState`: Editor grid configuration
- `BackgroundSettings`: Background image settings
- `SceneManager`: Scene loading/saving

### System Organization
Systems are organized into logical groups and run in Update schedule:
1. **Input & Camera**: Player movement, mouse interaction, camera controls
2. **Gameplay**: Shooting, projectile movement, collision detection
3. **Rendering & Editor**: Grid overlay, background rendering, editor UI
4. **Assets & UI**: Asset handling, menu systems

## Key Dependencies
- **bevy**: "0.13" - Main game engine framework
- **bevy_egui**: "0.25" - Immediate mode GUI for editor
- **serde**: "1.0" - Serialization framework
- **ron**: "0.8" - Rusty Object Notation for scene serialization
- **rfd**: "0.14" - File dialogs for asset importing
- **chrono**: "0.4" - Date/time handling

## Development Notes

### Editor Integration
The editor runs in-app using egui and integrates directly with the Bevy ECS. The editor can:
- Live edit entities while the game is running
- Switch between editor and play modes
- Save/load scenes using RON serialization
- Handle asset importing via file dialogs

### ECS Component System
Components are simple data structures (Player, Enemy, Projectile, Health, Collision, etc.) that get attached to entities. Systems operate on entities with specific component combinations.

### Asset Management
Assets are loaded through Bevy's asset system with custom importers. The project supports background images and sprite assets with plans for audio support.

### Scene Management
Scenes can be serialized to RON format for saving/loading. The editor maintains separate saved and temporary scene states for play mode transitions.