use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, egui};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(enemy_color_change)
        .add_system(editor_ui)
        .run();
}

/// Setup a simple 2D world: a player and some enemies
fn setup(mut commands: Commands) {
    // 2D Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
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
    ));

    // Some enemies
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(i as f32 * 60. - 120., 100., 0.),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                ..default()
            },
            Enemy,
        ));
    }
}

/// Move player with arrow keys
fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }
    transform.translation += (direction * 4.0).extend(0.);
}

/// Example "editor" UI panel using egui
fn editor_ui(
    mut contexts: EguiContexts,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    egui::Window::new("Scene Editor").show(contexts.ctx_mut(), |ui| {
        ui.heading("Entities in Scene");

        if let Ok(player_transform) = player_query.get_single() {
            ui.label(format!("Player at: {:?}", player_transform.translation));
        }

        ui.separator();
        for (i, transform) in enemy_query.iter().enumerate() {
            ui.label(format!("Enemy {} at: {:?}", i, transform.translation));
        }

        ui.separator();
        ui.label("Move the player with arrow keys!");
        ui.label("Extend this editor to add, remove, or modify entities.");
    });
}

/// Just for fun, enemies change color every frame
fn enemy_color_change(
    time: Res<Time>,
    mut query: Query<&mut Sprite, With<Enemy>>,
) {
    let t = time.elapsed_seconds();
    for (i, mut sprite) in query.iter_mut().enumerate() {
        let hue = (t + i as f32) % 1.0;
        sprite.color = Color::hsl(hue * 360.0, 0.8, 0.5);
    }
}
