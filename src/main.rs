use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContexts, EguiPlugin, egui};
use serde::{Deserialize, Serialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use std::fs;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Selectable {
    name: String,
}

#[derive(Component)]
struct Selected;

#[derive(Component)]
struct SceneEntity {
    id: u32,
}

#[derive(Component)]
struct CustomSprite {
    texture_path: String,
}

#[derive(Component)]
struct Projectile {
    velocity: Vec2,
    lifetime: f32,
    max_lifetime: f32,
}

#[derive(Component)]
struct Shooter {
    shoot_cooldown: f32,
    max_cooldown: f32,
    projectile_speed: f32,
}

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Resource, Default)]
struct GameState {
    is_playing: bool,
}

#[derive(Resource, Default)]
struct CameraController {
    target_position: Vec2,
    follow_entity: Option<Entity>,
}

#[derive(Resource, Default)]
struct SelectedEntity {
    entity: Option<Entity>,
}

#[derive(Resource, Default)]
struct DragState {
    dragging_entity: Option<Entity>,
    drag_offset: Vec2,
    last_mouse_world_pos: Vec2,
}

#[derive(Resource, Default)]
struct SceneManager {
    next_id: u32,
    save_path: String,
    spawn_entity_type: EntityType,
    spawn_position: Vec2,
}

#[derive(Resource, Default)]
struct AssetImporter {
    available_textures: Vec<(String, Handle<Image>)>,
    selected_texture: Option<String>,
}

#[derive(Resource, Default)]
struct ShootingStats {
    projectiles_fired: u32,
    projectiles_active: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SerializableTransform {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SerializableColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SerializableSize {
    width: f32,
    height: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
enum EntityType {
    Player,
    #[default]
    Enemy,
    CustomSprite,
}

#[derive(Clone, Debug, PartialEq)]
enum ComponentType {
    Player,
    Enemy,
    Shooter,
    Collider,
    Health,
    CustomSprite,
}

impl ComponentType {
    fn name(&self) -> &'static str {
        match self {
            ComponentType::Player => "🎮 Player",
            ComponentType::Enemy => "👾 Enemy",
            ComponentType::Shooter => "🔫 Shooter",
            ComponentType::Collider => "📦 Collider",
            ComponentType::Health => "❤️ Health",
            ComponentType::CustomSprite => "🖼️ Custom Sprite",
        }
    }
    
    fn description(&self) -> &'static str {
        match self {
            ComponentType::Player => "Makes entity controllable with arrow keys",
            ComponentType::Enemy => "Marks entity as an enemy (color changing)",
            ComponentType::Shooter => "Allows entity to shoot projectiles",
            ComponentType::Collider => "Enables collision detection",
            ComponentType::Health => "Adds health/damage system",
            ComponentType::CustomSprite => "Uses custom texture from assets",
        }
    }
    
    fn all_types() -> Vec<ComponentType> {
        vec![
            ComponentType::Player,
            ComponentType::Enemy,
            ComponentType::Shooter,
            ComponentType::Collider,
            ComponentType::Health,
            ComponentType::CustomSprite,
        ]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SerializableEntity {
    id: u32,
    name: String,
    entity_type: EntityType,
    transform: SerializableTransform,
    color: SerializableColor,
    size: SerializableSize,
    texture_path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Scene {
    entities: Vec<SerializableEntity>,
    next_id: u32,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            entities: Vec::new(),
            next_id: 1,
        }
    }
}

impl From<Transform> for SerializableTransform {
    fn from(transform: Transform) -> Self {
        Self {
            x: transform.translation.x,
            y: transform.translation.y,
            z: transform.translation.z,
        }
    }
}

impl Into<Transform> for SerializableTransform {
    fn into(self) -> Transform {
        Transform::from_xyz(self.x, self.y, self.z)
    }
}

impl From<Color> for SerializableColor {
    fn from(color: Color) -> Self {
        Self {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        }
    }
}

impl Into<Color> for SerializableColor {
    fn into(self) -> Color {
        Color::rgba(self.r, self.g, self.b, self.a)
    }
}

impl From<Vec2> for SerializableSize {
    fn from(size: Vec2) -> Self {
        Self {
            width: size.x,
            height: size.y,
        }
    }
}

impl Into<Vec2> for SerializableSize {
    fn into(self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin))
        .init_resource::<SelectedEntity>()
        .init_resource::<DragState>()
        .init_resource::<AssetImporter>()
        .init_resource::<ShootingStats>()
        .init_resource::<GameState>()
        .init_resource::<CameraController>()
        .insert_resource(SceneManager {
            next_id: 1,
            save_path: "scenes/default_scene.ron".to_string(),
            spawn_entity_type: EntityType::Enemy,
            spawn_position: Vec2::new(0.0, 0.0),
        })
        .add_systems(Startup, (setup, load_default_assets))
        .add_systems(Update, player_movement)
        .add_systems(Update, player_shooting)
        .add_systems(Update, projectile_movement)
        .add_systems(Update, projectile_cleanup)
        .add_systems(Update, update_shooting_cooldowns)
        .add_systems(Update, collision_detection)
        .add_systems(Update, boundary_collision)
        .add_systems(Update, enemy_color_change)
        .add_systems(Update, mouse_interaction)
        .add_systems(Update, entity_dragging)
        .add_systems(Update, update_selection_visuals)
        .add_systems(Update, camera_movement)
        .add_systems(Update, camera_controls)
        .add_systems(Update, editor_ui)
        .run();
}

/// Setup a simple 2D world: a player and some enemies
fn setup(mut commands: Commands, mut scene_manager: ResMut<SceneManager>, mut camera_controller: ResMut<CameraController>) {
    // Create scenes directory if it doesn't exist
    if let Err(_) = fs::create_dir_all("scenes") {
        println!("Could not create scenes directory");
    }

    // 2D Camera - positioned to see the game area
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    });

    // Initialize camera controller to center view
    camera_controller.target_position = Vec2::ZERO;

    // Player (with shooting capability) - positioned at origin
    let _player_entity = commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            ..default()
        },
        Player,
        Shooter {
            shoot_cooldown: 0.0,
            max_cooldown: 0.3, // Can shoot every 0.3 seconds
            projectile_speed: 300.0,
        },
        Collider { size: Vec2::new(40.0, 40.0) },
        Health { current: 100, max: 100 },
        Selectable { name: "Player".to_string() },
        SceneEntity { id: scene_manager.next_id },
    )).id();
    scene_manager.next_id += 1;

    // Some enemies - positioned above the player
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(i as f32 * 80. - 160., 150., 0.),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                ..default()
            },
            Enemy,
            Collider { size: Vec2::new(30.0, 30.0) },
            Health { current: 50, max: 50 },
            Selectable { name: format!("Enemy {}", i + 1) },
            SceneEntity { id: scene_manager.next_id },
        ));
        scene_manager.next_id += 1;
    }
    
    println!("🎮 Setup complete - Player at (0,0), Enemies at y=150");
    println!("📷 Camera positioned at origin, entities should be visible");
}

/// Load default assets and scan for available textures
fn load_default_assets(
    asset_server: Res<AssetServer>,
    mut asset_importer: ResMut<AssetImporter>,
) {
    // Create assets directory if it doesn't exist
    let _ = fs::create_dir_all("assets/sprites");
    
    // Scan for existing image files in assets/sprites
    if let Ok(entries) = fs::read_dir("assets/sprites") {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if is_image_file(file_name) {
                    let path = format!("sprites/{}", file_name);
                    let handle: Handle<Image> = asset_server.load(&path);
                    asset_importer.available_textures.push((path.clone(), handle));
                    println!("📁 Found texture: {}", path);
                }
            }
        }
    }
}

/// Check if a file is a supported image format
fn is_image_file(filename: &str) -> bool {
    let filename_lower = filename.to_lowercase();
    filename_lower.ends_with(".png") 
        || filename_lower.ends_with(".jpg") 
        || filename_lower.ends_with(".jpeg")
        || filename_lower.ends_with(".bmp")
        || filename_lower.ends_with(".tga")
        || filename_lower.ends_with(".webp")
}

/// Import a new texture file
fn import_texture(
    asset_server: &Res<AssetServer>,
    asset_importer: &mut ResMut<AssetImporter>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Copy file to assets/sprites directory
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;
    
    let dest_path = format!("assets/sprites/{}", file_name);
    fs::copy(file_path, &dest_path)?;
    
    // Load the asset
    let asset_path = format!("sprites/{}", file_name);
    let handle: Handle<Image> = asset_server.load(&asset_path);
    
    // Add to available textures
    asset_importer.available_textures.push((asset_path.clone(), handle));
    asset_importer.selected_texture = Some(asset_path.clone());
    
    println!("✅ Imported texture: {}", asset_path);
    Ok(())
}

/// Spawn a new entity at runtime
fn spawn_entity(
    commands: &mut Commands,
    scene_manager: &mut ResMut<SceneManager>,
    asset_importer: &ResMut<AssetImporter>,
    entity_type: EntityType,
    position: Vec2,
) {
    let (color, size, name) = match entity_type {
        EntityType::Player => {
            (Color::BLUE, Vec2::new(40.0, 40.0), format!("Player {}", scene_manager.next_id))
        }
        EntityType::Enemy => {
            (Color::RED, Vec2::new(30.0, 30.0), format!("Enemy {}", scene_manager.next_id))
        }
        EntityType::CustomSprite => {
            (Color::WHITE, Vec2::new(64.0, 64.0), format!("Sprite {}", scene_manager.next_id))
        }
    };

    let mut sprite_bundle = SpriteBundle {
        transform: Transform::from_xyz(position.x, position.y, 0.0),
        sprite: Sprite {
            color,
            custom_size: Some(size),
            ..default()
        },
        ..default()
    };

    // Use custom texture if it's a CustomSprite and we have a selected texture
    if entity_type == EntityType::CustomSprite {
        if let Some(texture_path) = &asset_importer.selected_texture {
            if let Some((_, handle)) = asset_importer.available_textures
                .iter()
                .find(|(path, _)| path == texture_path) 
            {
                sprite_bundle.texture = handle.clone();
            }
        }
    }

    let mut entity_commands = commands.spawn((
        sprite_bundle,
        Selectable { name },
        SceneEntity { id: scene_manager.next_id },
    ));

    // Add specific component based on entity type
    match entity_type {
        EntityType::Player => {
            entity_commands.insert((
                Player,
                Shooter {
                    shoot_cooldown: 0.0,
                    max_cooldown: 0.3,
                    projectile_speed: 300.0,
                },
                Collider { size },
                Health { current: 100, max: 100 },
            ));
        }
        EntityType::Enemy => {
            entity_commands.insert((
                Enemy,
                Collider { size },
                Health { current: 50, max: 50 },
            ));
        }
        EntityType::CustomSprite => {
            entity_commands.insert((
                Collider { size },
                Health { current: 25, max: 25 },
            ));
            if let Some(texture_path) = &asset_importer.selected_texture {
                entity_commands.insert(CustomSprite {
                    texture_path: texture_path.clone(),
                });
            }
        }
    }

    scene_manager.next_id += 1;
    println!("✅ Spawned new {:?} at {:?}", entity_type, position);
}

/// Delete the specified entity
fn delete_entity(
    commands: &mut Commands,
    entity: Entity,
    selected_entity: &mut ResMut<SelectedEntity>,
) {
    // Clear selection if we're deleting the selected entity
    if selected_entity.entity == Some(entity) {
        selected_entity.entity = None;
    }
    
    commands.entity(entity).despawn();
    println!("🗑️ Entity deleted");
}

/// Save the current scene to a file
fn save_scene(
    scene_manager: &SceneManager,
    query: Query<(&Transform, &Sprite, &Selectable, &SceneEntity, Option<&Player>, Option<&Enemy>, Option<&CustomSprite>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut scene = Scene::default();
    scene.next_id = scene_manager.next_id;

    for (transform, sprite, selectable, scene_entity, player, enemy, custom_sprite) in query.iter() {
        let entity_type = if player.is_some() {
            EntityType::Player
        } else if enemy.is_some() {
            EntityType::Enemy
        } else if custom_sprite.is_some() {
            EntityType::CustomSprite
        } else {
            continue; // Skip unknown entity types
        };

        let texture_path = custom_sprite.map(|cs| cs.texture_path.clone());

        let serializable_entity = SerializableEntity {
            id: scene_entity.id,
            name: selectable.name.clone(),
            entity_type,
            transform: (*transform).into(),
            color: sprite.color.into(),
            size: sprite.custom_size.unwrap_or(Vec2::new(30.0, 30.0)).into(),
            texture_path,
        };

        scene.entities.push(serializable_entity);
    }

    let ron_string = to_string_pretty(&scene, PrettyConfig::default())?;
    fs::write(&scene_manager.save_path, ron_string)?;
    println!("Scene saved to: {}", scene_manager.save_path);
    
    Ok(())
}

/// Load a scene from a file
fn load_scene(
    commands: &mut Commands,
    scene_manager: &mut ResMut<SceneManager>,
    selected_entity: &mut ResMut<SelectedEntity>,
    existing_entities: &Query<Entity, (With<SceneEntity>, Without<Camera>)>,
    asset_server: &Res<AssetServer>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear existing scene entities (but not the camera)
    for entity in existing_entities.iter() {
        commands.entity(entity).despawn();
    }
    
    // Clear selection
    selected_entity.entity = None;

    // Read and parse the scene file
    let file_content = fs::read_to_string(&scene_manager.save_path)?;
    let scene: Scene = ron::from_str(&file_content)?;

    // Update the scene manager's next_id
    scene_manager.next_id = scene.next_id;

    // Spawn entities from the scene
    for entity_data in scene.entities {
        let transform: Transform = entity_data.transform.into();
        let color: Color = entity_data.color.into();
        let size: Vec2 = entity_data.size.into();

        let mut sprite_bundle = SpriteBundle {
            transform,
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            ..default()
        };

        // Load custom texture if specified
        if let Some(texture_path) = &entity_data.texture_path {
            let handle: Handle<Image> = asset_server.load(texture_path);
            sprite_bundle.texture = handle;
        }

        let mut entity_commands = commands.spawn((
            sprite_bundle,
            Selectable { name: entity_data.name },
            SceneEntity { id: entity_data.id },
        ));

        // Add specific component based on entity type
        match entity_data.entity_type {
            EntityType::Player => {
                entity_commands.insert((
                    Player,
                    Shooter {
                        shoot_cooldown: 0.0,
                        max_cooldown: 0.3,
                        projectile_speed: 300.0,
                    },
                    Collider { size },
                    Health { current: 100, max: 100 },
                ));
            }
            EntityType::Enemy => {
                entity_commands.insert((
                    Enemy,
                    Collider { size },
                    Health { current: 50, max: 50 },
                ));
            }
            EntityType::CustomSprite => {
                entity_commands.insert((
                    Collider { size },
                    Health { current: 25, max: 25 },
                ));
                if let Some(texture_path) = entity_data.texture_path {
                    entity_commands.insert(CustomSprite { texture_path });
                }
            }
        }
    }

    println!("Scene loaded from: {}", scene_manager.save_path);
    Ok(())
}

/// Move player with arrow keys (only when game is playing)
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    // Only move if game is playing
    if !game_state.is_playing {
        return;
    }

    // Check if there's a player entity before trying to move it
    let Ok(mut transform) = query.get_single_mut() else {
        return; // No player entity exists, so just return early
    };
    
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    // Frame-rate independent movement
    transform.translation += (direction * 200.0 * time.delta_seconds()).extend(0.);
}

/// Handle player shooting (only when game is playing)
fn player_shooting(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    game_state: Res<GameState>,
    mut player_query: Query<(&Transform, &mut Shooter), With<Player>>,
    mut shooting_stats: ResMut<ShootingStats>,
) {
    // Only shoot if game is playing
    if !game_state.is_playing {
        return;
    }

    // Check if spacebar is pressed
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    // Find the player and check if they can shoot
    for (transform, mut shooter) in player_query.iter_mut() {
        if shooter.shoot_cooldown <= 0.0 {
            // Spawn projectile at player position
            let projectile_pos = transform.translation + Vec3::new(0.0, 25.0, 0.1); // Slightly above player
            
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(projectile_pos),
                    sprite: Sprite {
                        color: Color::YELLOW,
                        custom_size: Some(Vec2::new(8.0, 16.0)),
                        ..default()
                    },
                    ..default()
                },
                Projectile {
                    velocity: Vec2::new(0.0, shooter.projectile_speed), // Shoot upward
                    lifetime: 0.0,
                    max_lifetime: 3.0, // Projectile lasts 3 seconds
                },
                Collider { size: Vec2::new(8.0, 16.0) },
                Selectable { name: format!("Projectile {}", shooting_stats.projectiles_fired + 1) },
            ));

            // Set cooldown and update stats
            shooter.shoot_cooldown = shooter.max_cooldown;
            shooting_stats.projectiles_fired += 1;
            shooting_stats.projectiles_active += 1;
            
            println!("💥 Player fired projectile! Total fired: {}", shooting_stats.projectiles_fired);
        }
    }
}

/// Move projectiles based on their velocity (only when game is playing)
fn projectile_movement(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut projectile_query: Query<(&mut Transform, &mut Projectile)>,
) {
    // Only move projectiles if game is playing
    if !game_state.is_playing {
        return;
    }

    for (mut transform, mut projectile) in projectile_query.iter_mut() {
        // Move projectile
        let movement = projectile.velocity * time.delta_seconds();
        transform.translation += movement.extend(0.0);
        
        // Update lifetime
        projectile.lifetime += time.delta_seconds();
    }
}

/// Clean up projectiles that have exceeded their lifetime (only when game is playing)
fn projectile_cleanup(
    mut commands: Commands,
    game_state: Res<GameState>,
    projectile_query: Query<(Entity, &Projectile)>,
    mut shooting_stats: ResMut<ShootingStats>,
) {
    // Only cleanup projectiles if game is playing
    if !game_state.is_playing {
        return;
    }

    for (entity, projectile) in projectile_query.iter() {
        if projectile.lifetime >= projectile.max_lifetime {
            commands.entity(entity).despawn();
            shooting_stats.projectiles_active = shooting_stats.projectiles_active.saturating_sub(1);
            println!("🗑️ Projectile despawned (lifetime expired)");
        }
    }
}

/// Update shooting cooldowns (only when game is playing)
fn update_shooting_cooldowns(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut shooter_query: Query<&mut Shooter>,
) {
    // Only update cooldowns if game is playing
    if !game_state.is_playing {
        return;
    }

    for mut shooter in shooter_query.iter_mut() {
        if shooter.shoot_cooldown > 0.0 {
            shooter.shoot_cooldown -= time.delta_seconds();
            shooter.shoot_cooldown = shooter.shoot_cooldown.max(0.0); // Don't go below 0
        }
    }
}

/// Handle mouse clicks for entity selection and drag initiation
fn mouse_interaction(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    selectable_query: Query<(Entity, &Transform, &Sprite), With<Selectable>>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut drag_state: ResMut<DragState>,
    mut commands: Commands,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    // Get mouse position in world coordinates
    let cursor_position = window.cursor_position();
    let world_position = if let Some(cursor_pos) = cursor_position {
        camera.viewport_to_world_2d(camera_transform, cursor_pos)
    } else {
        None
    };

    // Handle mouse press
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = world_position {
            // Clear previous selection
            if let Some(old_entity) = selected_entity.entity {
                commands.entity(old_entity).remove::<Selected>();
            }
            selected_entity.entity = None;
            drag_state.dragging_entity = None;

            // Check if we clicked on any selectable entity
            for (entity, transform, sprite) in selectable_query.iter() {
                if let Some(size) = sprite.custom_size {
                    let entity_pos = transform.translation.truncate();
                    let half_size = size / 2.0;
                    
                    // Simple AABB collision detection
                    if world_pos.x >= entity_pos.x - half_size.x
                        && world_pos.x <= entity_pos.x + half_size.x
                        && world_pos.y >= entity_pos.y - half_size.y
                        && world_pos.y <= entity_pos.y + half_size.y
                    {
                        // Select this entity
                        selected_entity.entity = Some(entity);
                        commands.entity(entity).insert(Selected);
                        
                        // Start dragging
                        drag_state.dragging_entity = Some(entity);
                        drag_state.drag_offset = entity_pos - world_pos;
                        drag_state.last_mouse_world_pos = world_pos;
                        break;
                    }
                }
            }
        }
    }

    // Handle mouse release
    if mouse_input.just_released(MouseButton::Left) {
        if drag_state.dragging_entity.is_some() {
            println!("🎯 Finished dragging entity");
            drag_state.dragging_entity = None;
        }
    }

    // Update mouse position for dragging
    if let Some(world_pos) = world_position {
        drag_state.last_mouse_world_pos = world_pos;
    }
}

/// Handle entity dragging
fn entity_dragging(
    drag_state: Res<DragState>,
    mut query: Query<&mut Transform, With<Selectable>>,
) {
    if let Some(dragging_entity) = drag_state.dragging_entity {
        if let Ok(mut transform) = query.get_mut(dragging_entity) {
            // Calculate new position based on mouse position and drag offset
            let new_position = drag_state.last_mouse_world_pos + drag_state.drag_offset;
            transform.translation.x = new_position.x;
            transform.translation.y = new_position.y;
        }
    }
}

/// Update visual indicators for selected entities
fn update_selection_visuals(
    mut selected_query: Query<&mut Sprite, (With<Selected>, Without<Enemy>)>,
    mut selected_enemies: Query<&mut Sprite, (With<Selected>, With<Enemy>)>,
    time: Res<Time>,
) {
    // Add a pulsing white outline effect to selected entities
    let pulse = (time.elapsed_seconds() * 3.0).sin() * 0.3 + 0.7;
    
    // Handle non-enemy selected entities (like player)
    for mut sprite in selected_query.iter_mut() {
        // Add a white border effect by slightly increasing brightness
        if let Color::Rgba { red, green, blue, alpha } = sprite.color {
            sprite.color = Color::rgba(
                (red + pulse * 0.3).min(1.0),
                (green + pulse * 0.3).min(1.0),
                (blue + pulse * 0.3).min(1.0),
                alpha,
            );
        }
    }
    
    // Handle selected enemies (they change color, so we need different highlighting)
    for mut sprite in selected_enemies.iter_mut() {
        // For enemies, add a white border by blending with white
        if let Color::Hsla { hue, saturation, lightness, alpha } = sprite.color {
            sprite.color = Color::hsla(hue, saturation * (1.0 - pulse * 0.5), lightness + pulse * 0.2, alpha);
        }
    }
}

/// Focus camera on a specific entity
fn focus_camera_on_entity(
    camera_controller: &mut ResMut<CameraController>,
    entity: Entity,
    transform_query: &Query<&Transform, With<SceneEntity>>,
) {
    if let Ok(transform) = transform_query.get(entity) {
        camera_controller.target_position = transform.translation.truncate();
        camera_controller.follow_entity = Some(entity);
        println!("🎯 Focusing camera on entity at {:?}", camera_controller.target_position);
    }
}

/// Camera movement system - smoothly moves camera to target position
fn camera_movement(
    time: Res<Time>,
    mut camera_controller: ResMut<CameraController>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<SceneEntity>)>,
    entity_query: Query<&Transform, (With<SceneEntity>, Without<Camera2d>)>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    // If following an entity, update target position
    if let Some(follow_entity) = camera_controller.follow_entity {
        if let Ok(entity_transform) = entity_query.get(follow_entity) {
            camera_controller.target_position = entity_transform.translation.truncate();
        } else {
            // Entity no longer exists, stop following
            camera_controller.follow_entity = None;
        }
    }

    // Smoothly move camera towards target position
    let current_pos = camera_transform.translation.truncate();
    let target_pos = camera_controller.target_position;
    let distance = target_pos - current_pos;
    
    if distance.length() > 1.0 {
        let move_speed = 500.0; // Camera movement speed
        let movement = distance.normalize() * move_speed * time.delta_seconds();
        let new_pos = current_pos + movement;
        camera_transform.translation = new_pos.extend(camera_transform.translation.z);
    } else {
        // Close enough, snap to target
        camera_transform.translation = target_pos.extend(camera_transform.translation.z);
    }
}

/// Manual camera controls with WASD (only when game is not playing)
fn camera_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut camera_controller: ResMut<CameraController>,
) {
    // Only allow manual camera control when game is paused/stopped
    if game_state.is_playing {
        return;
    }

    let mut direction = Vec2::ZERO;
    let camera_speed = 300.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        // Stop following entity when manually controlling camera
        camera_controller.follow_entity = None;
        
        // Move camera target position
        let movement = direction.normalize() * camera_speed * time.delta_seconds();
        camera_controller.target_position += movement;
    }
}

/// Editor UI with left panel layout and game viewport on right
fn editor_ui(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut selected_entity: ResMut<SelectedEntity>,
    mut scene_manager: ResMut<SceneManager>,
    mut asset_importer: ResMut<AssetImporter>,
    mut shooting_stats: ResMut<ShootingStats>,
    mut game_state: ResMut<GameState>,
    mut camera_controller: ResMut<CameraController>,
    asset_server: Res<AssetServer>,
    mut query_set: ParamSet<(
        Query<(Entity, &mut Transform, &mut Sprite, &Selectable), With<SceneEntity>>,
        Query<(&Transform, &Sprite, &Selectable, &SceneEntity, Option<&Player>, Option<&Enemy>, Option<&CustomSprite>)>,
    )>,
    existing_entities: Query<Entity, (With<SceneEntity>, Without<Camera>)>,
) {
    // Left Side Panel - All editor tools in vertical layout
    egui::SidePanel::left("editor_panel")
        .resizable(true)
        .default_width(350.0)
        .width_range(300.0..=500.0)
        .show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Game Controls at the top
                ui.group(|ui| {
                    ui.heading("🎮 Game Controls");
                    
                    ui.horizontal(|ui| {
                        let button_text = if game_state.is_playing { "⏸️ Pause" } else { "▶️ Play" };
                        if ui.button(button_text).clicked() {
                            game_state.is_playing = !game_state.is_playing;
                            println!("Game {}", if game_state.is_playing { "started" } else { "paused" });
                        }
                        
                        if ui.button("⏹️ Stop").clicked() {
                            game_state.is_playing = false;
                            // Reset all projectiles and stats
                            shooting_stats.projectiles_fired = 0;
                            shooting_stats.projectiles_active = 0;
                            println!("Game stopped and reset");
                        }
                    });
                    
                    let status = if game_state.is_playing { "🟢 Running" } else { "🔴 Paused" };
                    ui.label(format!("Status: {}", status));
                    
                    ui.separator();
                    ui.label("📷 Camera Controls:");
                    ui.horizontal(|ui| {
                        if ui.button("🏠 Reset Camera").clicked() {
                            camera_controller.target_position = Vec2::ZERO;
                            camera_controller.follow_entity = None;
                            println!("📷 Camera reset to origin");
                        }
                        if ui.button("🎯 Focus Player").clicked() {
                            // Find the first player entity and focus on it
                            let entities_query = query_set.p0();
                            for (entity, transform, _, selectable) in entities_query.iter() {
                                if selectable.name.contains("Player") {
                                    camera_controller.target_position = transform.translation.truncate();
                                    camera_controller.follow_entity = Some(entity);
                                    println!("🎯 Focusing camera on Player");
                                    break;
                                }
                            }
                        }
                    });
                });

                ui.add_space(10.0);

                // Scene Manager Panel
                ui.group(|ui| {
                    ui.heading("🗂️ Scene Manager");
                    
                    ui.horizontal(|ui| {
                        ui.label("File:");
                        ui.text_edit_singleline(&mut scene_manager.save_path);
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("💾 Save Scene").clicked() {
                            let scene_entities = query_set.p1();
                            match save_scene(&scene_manager, scene_entities) {
                                Ok(_) => println!("✅ Scene saved successfully!"),
                                Err(e) => eprintln!("❌ Failed to save scene: {}", e),
                            }
                        }
                        if ui.button("📁 Load Scene").clicked() {
                            match load_scene(&mut commands, &mut scene_manager, &mut selected_entity, &existing_entities, &asset_server) {
                                Ok(_) => println!("✅ Scene loaded successfully!"),
                                Err(e) => eprintln!("❌ Failed to load scene: {}", e),
                            }
                        }
                    });
                    
                    if ui.button("🆕 New Scene").clicked() {
                        for entity in existing_entities.iter() {
                            commands.entity(entity).despawn();
                        }
                        selected_entity.entity = None;
                        scene_manager.next_id = 1;
                        shooting_stats.projectiles_fired = 0;
                        shooting_stats.projectiles_active = 0;
                        println!("✅ New scene created!");
                    }
                    
                    ui.separator();
                    ui.label(format!("🎯 Projectiles Fired: {}", shooting_stats.projectiles_fired));
                    ui.label(format!("🎯 Active Projectiles: {}", shooting_stats.projectiles_active));
                    
                    if ui.button("🔄 Reset Stats").clicked() {
                        shooting_stats.projectiles_fired = 0;
                        shooting_stats.projectiles_active = 0;
                    }
                });

                ui.add_space(10.0);
                
                // Asset Manager Panel
                ui.group(|ui| {
                    ui.heading("🎨 Asset Manager");
                    
                    ui.label("💡 Add .png/.jpg files to assets/sprites/");
                    ui.label("💡 Restart editor to load new assets");
                    
                    if !asset_importer.available_textures.is_empty() {
                        ui.separator();
                        ui.label("Available Textures:");
                        
                        let available_textures = asset_importer.available_textures.clone();
                        let mut texture_to_select: Option<String> = None;
                        
                        egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                            for (texture_path, _handle) in &available_textures {
                                let is_selected = asset_importer.selected_texture.as_ref() == Some(texture_path);
                                if ui.selectable_label(is_selected, texture_path.split('/').last().unwrap_or(texture_path)).clicked() {
                                    texture_to_select = Some(texture_path.clone());
                                }
                            }
                        });
                        
                        if let Some(selected_texture) = texture_to_select {
                            asset_importer.selected_texture = Some(selected_texture);
                        }
                        
                        if let Some(selected) = &asset_importer.selected_texture {
                            ui.label(format!("Selected: {}", selected.split('/').last().unwrap_or(selected)));
                        }
                    } else {
                        ui.label("No textures found");
                    }
                });

                ui.add_space(10.0);
                
                // Entity Spawner Panel
                ui.group(|ui| {
                    ui.heading("🛠️ Entity Spawner");
                    
                    ui.label("Quick Spawn:");
                    ui.horizontal(|ui| {
                        if ui.button("🎮 Player").clicked() {
                            spawn_entity(&mut commands, &mut scene_manager, &asset_importer, EntityType::Player, Vec2::ZERO);
                        }
                        if ui.button("👾 Enemy").clicked() {
                            spawn_entity(&mut commands, &mut scene_manager, &asset_importer, EntityType::Enemy, Vec2::new(100.0, 0.0));
                        }
                    });
                    
                    if ui.button("🖼️ Custom Sprite").clicked() {
                        spawn_entity(&mut commands, &mut scene_manager, &asset_importer, EntityType::CustomSprite, Vec2::new(-100.0, 0.0));
                    }
                    
                    ui.separator();
                    ui.label("Custom Spawn:");
                    
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Player, "🎮");
                        ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::Enemy, "👾");
                        ui.selectable_value(&mut scene_manager.spawn_entity_type, EntityType::CustomSprite, "🖼️");
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        ui.add(egui::DragValue::new(&mut scene_manager.spawn_position.x).prefix("X:").speed(5.0));
                        ui.add(egui::DragValue::new(&mut scene_manager.spawn_position.y).prefix("Y:").speed(5.0));
                    });
                    
                    if ui.button("✨ Spawn Entity").clicked() {
                        let entity_type = scene_manager.spawn_entity_type.clone();
                        let position = scene_manager.spawn_position;
                        spawn_entity(&mut commands, &mut scene_manager, &asset_importer, entity_type, position);
                    }
                });

                ui.add_space(10.0);
                
                // Hierarchy Panel
                ui.group(|ui| {
                    ui.heading("📋 Scene Hierarchy");
                    
                    let mut entity_to_delete: Option<Entity> = None;
                    let mut entity_to_focus: Option<Entity> = None;
                    let selectable_query = query_set.p0();
                    
                    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        for (entity, transform, _sprite, selectable) in selectable_query.iter() {
                            let is_selected = selected_entity.entity == Some(entity);
                            
                            ui.horizontal(|ui| {
                                // Left click to select
                                if ui.selectable_label(is_selected, &selectable.name).clicked() {
                                    if let Some(old_entity) = selected_entity.entity {
                                        commands.entity(old_entity).remove::<Selected>();
                                    }
                                    selected_entity.entity = Some(entity);
                                    commands.entity(entity).insert(Selected);
                                }
                                
                                // Focus button
                                if ui.small_button("🎯").on_hover_text("Focus camera on this entity").clicked() {
                                    entity_to_focus = Some(entity);
                                }
                                
                                // Delete button
                                if ui.small_button("🗑️").on_hover_text("Delete entity").clicked() {
                                    entity_to_delete = Some(entity);
                                }
                            });
                            
                            // Right-click context menu
                            ui.horizontal(|ui| {
                                let response = ui.selectable_label(false, "");
                                response.context_menu(|ui| {
                                    if ui.button("🎯 Focus Camera").clicked() {
                                        entity_to_focus = Some(entity);
                                        ui.close_menu();
                                    }
                                    if ui.button("📍 Show Position").clicked() {
                                        println!("📍 {} position: {:?}", selectable.name, transform.translation.truncate());
                                        ui.close_menu();
                                    }
                                    ui.separator();
                                    if ui.button("🗑️ Delete").clicked() {
                                        entity_to_delete = Some(entity);
                                        ui.close_menu();
                                    }
                                });
                            });
                        }
                    });
                    
                    if let Some(entity) = entity_to_delete {
                        delete_entity(&mut commands, entity, &mut selected_entity);
                    }
                    
                    if let Some(entity) = entity_to_focus {
                        let transform_query = query_set.p0();
                        if let Ok((_, transform, _, _)) = transform_query.get(entity) {
                            camera_controller.target_position = transform.translation.truncate();
                            camera_controller.follow_entity = Some(entity);
                            println!("🎯 Focusing camera on entity at {:?}", camera_controller.target_position);
                        }
                    }
                });

                ui.add_space(10.0);
                
                // Inspector Panel
                ui.group(|ui| {
                    ui.heading("⚙️ Inspector");
                    
                    if let Some(selected_entity_id) = selected_entity.entity {
                        let mut selectable_query = query_set.p0();
                        if let Ok((_, mut transform, mut sprite, selectable)) = selectable_query.get_mut(selected_entity_id) {
                            ui.strong(&selectable.name);
                            ui.separator();
                            
                            // Transform
                            ui.group(|ui| {
                                ui.label("🌍 Transform");
                                ui.horizontal(|ui| {
                                    ui.add(egui::DragValue::new(&mut transform.translation.x).prefix("X: ").speed(1.0));
                                    ui.add(egui::DragValue::new(&mut transform.translation.y).prefix("Y: ").speed(1.0));
                                });
                            });
                            
                            ui.add_space(5.0);
                            
                            // Sprite
                            ui.group(|ui| {
                                ui.label("🎨 Sprite");
                                
                                let is_enemy = selectable.name.contains("Enemy");
                                if !is_enemy {
                                    ui.label("Color:");
                                    let mut color = [
                                        sprite.color.r(),
                                        sprite.color.g(),
                                        sprite.color.b(),
                                        sprite.color.a(),
                                    ];
                                    if ui.color_edit_button_rgba_unmultiplied(&mut color).changed() {
                                        sprite.color = Color::rgba(color[0], color[1], color[2], color[3]);
                                    }
                                }
                                
                                if let Some(mut size) = sprite.custom_size {
                                    ui.label("Size:");
                                    ui.horizontal(|ui| {
                                        if ui.add(egui::DragValue::new(&mut size.x).prefix("W:").speed(1.0).clamp_range(10.0..=150.0)).changed() {
                                            sprite.custom_size = Some(size);
                                        }
                                        if ui.add(egui::DragValue::new(&mut size.y).prefix("H:").speed(1.0).clamp_range(10.0..=150.0)).changed() {
                                            sprite.custom_size = Some(size);
                                        }
                                    });
                                }
                            });
                            
                            ui.add_space(5.0);
                            
                            // Component Management (Simplified)
                            ui.group(|ui| {
                                ui.label("🔧 Component Management");
                                ui.label("Add components to selected entity:");
                                
                                ui.horizontal_wrapped(|ui| {
                                    if ui.small_button("+ 🎮 Player").on_hover_text("Makes entity controllable with arrow keys").clicked() {
                                        commands.entity(selected_entity_id).insert(Player);
                                        println!("✅ Added Player component to {}", selectable.name);
                                    }
                                    if ui.small_button("+ 👾 Enemy").on_hover_text("Marks entity as an enemy (color changing)").clicked() {
                                        commands.entity(selected_entity_id).insert(Enemy);
                                        println!("✅ Added Enemy component to {}", selectable.name);
                                    }
                                    if ui.small_button("+ 🔫 Shooter").on_hover_text("Allows entity to shoot projectiles").clicked() {
                                        commands.entity(selected_entity_id).insert(Shooter {
                                            shoot_cooldown: 0.0,
                                            max_cooldown: 0.3,
                                            projectile_speed: 300.0,
                                        });
                                        println!("✅ Added Shooter component to {}", selectable.name);
                                    }
                                });
                                
                                ui.horizontal_wrapped(|ui| {
                                    if ui.small_button("+ 📦 Collider").on_hover_text("Enables collision detection").clicked() {
                                        let size = sprite.custom_size.unwrap_or(Vec2::new(30.0, 30.0));
                                        commands.entity(selected_entity_id).insert(Collider { size });
                                        println!("✅ Added Collider component to {}", selectable.name);
                                    }
                                    if ui.small_button("+ ❤️ Health").on_hover_text("Adds health/damage system").clicked() {
                                        commands.entity(selected_entity_id).insert(Health { current: 100, max: 100 });
                                        println!("✅ Added Health component to {}", selectable.name);
                                    }
                                    if ui.small_button("+ 🖼️ Custom Sprite").on_hover_text("Uses custom texture from assets").clicked() {
                                        if let Some(texture_path) = &asset_importer.selected_texture {
                                            commands.entity(selected_entity_id).insert(CustomSprite {
                                                texture_path: texture_path.clone(),
                                            });
                                            println!("✅ Added CustomSprite component to {}", selectable.name);
                                        } else {
                                            println!("⚠️ No texture selected for CustomSprite component");
                                        }
                                    }
                                });
                                
                                ui.separator();
                                ui.label("Remove components:");
                                ui.horizontal_wrapped(|ui| {
                                    if ui.small_button("- 🎮 Player").clicked() {
                                        commands.entity(selected_entity_id).remove::<Player>();
                                        println!("🗑️ Removed Player component from {}", selectable.name);
                                    }
                                    if ui.small_button("- 👾 Enemy").clicked() {
                                        commands.entity(selected_entity_id).remove::<Enemy>();
                                        println!("🗑️ Removed Enemy component from {}", selectable.name);
                                    }
                                    if ui.small_button("- 🔫 Shooter").clicked() {
                                        commands.entity(selected_entity_id).remove::<Shooter>();
                                        println!("🗑️ Removed Shooter component from {}", selectable.name);
                                    }
                                });
                                
                                ui.horizontal_wrapped(|ui| {
                                    if ui.small_button("- 📦 Collider").clicked() {
                                        commands.entity(selected_entity_id).remove::<Collider>();
                                        println!("🗑️ Removed Collider component from {}", selectable.name);
                                    }
                                    if ui.small_button("- ❤️ Health").clicked() {
                                        commands.entity(selected_entity_id).remove::<Health>();
                                        println!("🗑️ Removed Health component from {}", selectable.name);
                                    }
                                    if ui.small_button("- 🖼️ Custom Sprite").clicked() {
                                        commands.entity(selected_entity_id).remove::<CustomSprite>();
                                        println!("🗑️ Removed CustomSprite component from {}", selectable.name);
                                    }
                                });
                            });
                            
                            ui.add_space(5.0);
                            
                            ui.horizontal(|ui| {
                                if ui.button("Deselect").clicked() {
                                    commands.entity(selected_entity_id).remove::<Selected>();
                                    selected_entity.entity = None;
                                }
                                
                                if ui.button("🗑️ Delete").clicked() {
                                    delete_entity(&mut commands, selected_entity_id, &mut selected_entity);
                                }
                            });
                            
                        } else {
                            ui.label("Selected entity not found");
                        }
                    } else {
                        ui.label("No entity selected");
                        ui.separator();
                        ui.label("💡 Controls:");
                        ui.label("• Click entities to select");
                        ui.label("• Drag entities to move (editor mode)");
                        ui.label("• WASD moves camera (editor mode)");
                        ui.label("• Arrow keys move player (play mode)");
                        ui.label("• SPACEBAR shoots (play mode)");
                        ui.label("• 🎯 button or right-click to focus camera");
                        ui.label("• Use Play/Pause buttons above");
                    }
                });
            });
        });

    // No central panel - let Bevy render the game in the remaining space
    // The game viewport will automatically fill the space not used by the left panel
}

/// Just for fun, enemies change color every frame (but not if selected, and only when game is playing)
fn enemy_color_change(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<&mut Sprite, (With<Enemy>, Without<Selected>)>,
) {
    // Only animate colors if game is playing
    if !game_state.is_playing {
        return;
    }

    let t = time.elapsed_seconds();
    for (i, mut sprite) in query.iter_mut().enumerate() {
        let hue = (t + i as f32) % 1.0;
        sprite.color = Color::hsl(hue * 360.0, 0.8, 0.5);
    }
}

/// Collision detection between projectiles and enemies
fn collision_detection(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut shooting_stats: ResMut<ShootingStats>,
    projectile_query: Query<(Entity, &Transform, &Collider), With<Projectile>>,
    mut enemy_query: Query<(Entity, &Transform, &Collider, &mut Health, &Selectable), (With<Enemy>, Without<Projectile>)>,
) {
    // Only check collisions if game is playing
    if !game_state.is_playing {
        return;
    }

    for (projectile_entity, projectile_transform, projectile_collider) in projectile_query.iter() {
        let projectile_pos = projectile_transform.translation.truncate();
        let projectile_half_size = projectile_collider.size / 2.0;

        for (enemy_entity, enemy_transform, enemy_collider, mut enemy_health, enemy_selectable) in enemy_query.iter_mut() {
            let enemy_pos = enemy_transform.translation.truncate();
            let enemy_half_size = enemy_collider.size / 2.0;

            // AABB collision detection
            if projectile_pos.x + projectile_half_size.x >= enemy_pos.x - enemy_half_size.x
                && projectile_pos.x - projectile_half_size.x <= enemy_pos.x + enemy_half_size.x
                && projectile_pos.y + projectile_half_size.y >= enemy_pos.y - enemy_half_size.y
                && projectile_pos.y - projectile_half_size.y <= enemy_pos.y + enemy_half_size.y
            {
                // Collision detected!
                println!("💥 Projectile hit {}!", enemy_selectable.name);
                
                // Damage the enemy
                enemy_health.current -= 25;
                
                // Remove the projectile
                commands.entity(projectile_entity).despawn();
                shooting_stats.projectiles_active = shooting_stats.projectiles_active.saturating_sub(1);
                
                // Check if enemy is destroyed
                if enemy_health.current <= 0 {
                    println!("💀 {} destroyed!", enemy_selectable.name);
                    commands.entity(enemy_entity).despawn();
                } else {
                    println!("🩸 {} health: {}/{}", enemy_selectable.name, enemy_health.current, enemy_health.max);
                }
                
                break; // Projectile can only hit one enemy
            }
        }
    }
}

/// Boundary collision to keep entities within screen bounds
fn boundary_collision(
    game_state: Res<GameState>,
    mut query: Query<(&mut Transform, &Collider), (With<SceneEntity>, Without<Projectile>)>,
) {
    // Only apply boundary collision if game is playing
    if !game_state.is_playing {
        return;
    }

    let boundary = 400.0; // Screen boundary limit
    
    for (mut transform, collider) in query.iter_mut() {
        let half_size = collider.size / 2.0;
        
        // Keep entities within bounds
        if transform.translation.x - half_size.x < -boundary {
            transform.translation.x = -boundary + half_size.x;
        }
        if transform.translation.x + half_size.x > boundary {
            transform.translation.x = boundary - half_size.x;
        }
        if transform.translation.y - half_size.y < -boundary {
            transform.translation.y = -boundary + half_size.y;
        }
        if transform.translation.y + half_size.y > boundary {
            transform.translation.y = boundary - half_size.y;
        }
    }
}