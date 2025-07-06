# 🦀 GameMaker Rust — Modular 2D/3D Game Engine & Editor

![Bevy Logo](https://raw.githubusercontent.com/bevyengine/bevy/main/assets/bevy_logo_dark_big.png)

**GameMaker Rust** is a modular game engine and in-app editor built in [Rust](https://www.rust-lang.org/) using [Bevy](https://bevyengine.org/).  
The goal is to become a GameMaker Studio-like engine for indie developers: easy to use, lightning-fast, and cross-platform (desktop & web), with a clear path from 2D to 3D games.

---

## 🚀 Project Goals

- **2D Game Engine:** Robust 2D rendering, input, physics, and asset pipeline.
- **Visual Editor:** In-app scene, asset, and entity editor using an immediate-mode GUI (`egui`).
- **Extensible Core:** Clean ECS architecture, plugin-ready, and eventually scripting support.
- **3D Ready:** Foundation for 3D features (camera, meshes, lighting) for future expansion.
- **Export Anywhere:** Target Windows, macOS, Linux, and WebAssembly.

---

## 🏗️ Architecture



+-----------------------+
| Editor UI (egui) | ← Scene tree, entity inspector, asset browser
+----------+------------+
|
v
+----------+------------+
| Engine Core (Bevy) | ← ECS, systems, resources, asset management
+----------+------------+
|
v
+----------+------------+
| Renderer (wgpu) | ← 2D (sprites, tilemaps), 3D-ready for later
+-----------------------+





- **Editor UI:** Built using `egui`, appears in-app, interacts with Bevy ECS for live editing.
- **Engine Core:** Handles scenes, components, systems (input, rendering, etc.).
- **Renderer:** Abstracted via Bevy/wgpu, supports both 2D and future 3D.

---

## ✨ Features

- [x] **2D Sprite Rendering**
- [x] **Basic Editor UI** with entity/position listing
- [x] **Keyboard Input for Player Movement**
- [x] **Live Scene Updates** (edit entities while running)
- [ ] **Entity Selection & Manipulation**
- [ ] **Asset Import Pipeline** (images, audio, etc.)
- [ ] **Scene Save/Load (JSON/RON)**
- [ ] **Animation & Audio Support**
- [ ] **Plugin System**
- [ ] **3D Support (future)**

---

## 📦 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/)
- Recommended: [VS Code](https://code.visualstudio.com/) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Clone & Run

```bash
git clone https://github.com/your-username/gamemaker_rust.git
cd gamemaker_rust
cargo run




🛣️ Roadmap / Current Plans
[ ] Entity Selection: Click/select and modify entities in the editor panel.

[ ] Scene Serialization: Save/load the scene (JSON or RON).

[ ] Asset Importing: Drag/drop or load sprites, backgrounds, audio.

[ ] Scene Hierarchy/Inspector: View and edit entities/components visually.

[ ] Mouse Controls: Move entities with drag-and-drop.

[ ] Animation: Add sprite animation and timelines.

[ ] Plugin/Script Support: (Optional) Add scripting (Lua, Rhai, or WASM).

[ ] 3D Features: Add camera, meshes, and lights for 3D support.




🤝 Contributing
Learning Rust or Bevy? Perfect!
This project is beginner-friendly. Issues, pull requests, and feedback are welcome.

To contribute:

Fork this repo and clone your fork.

Create a new branch (git checkout -b feature/my-feature).

Make your changes and commit.

Push to your fork and open a Pull Request.

📚 Learning Resources
The Rust Book

Bevy Engine Book

egui Docs

Rustlings Practice

Awesome Bevy

