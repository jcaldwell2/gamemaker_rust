//! Gameplay systems for combat, movement, and interactions

use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Handle player shooting
pub fn player_shooting(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Shooting), With<Player>>,
    mut shooting_stats: ResMut<ShootingStats>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    // Only run when game is playing and not paused
    if !game_state.playing || game_state.paused {
        return;
    }
    for (transform, mut shooting) in player_query.iter_mut() {
        if shooting.cooldown > 0.0 {
            shooting.cooldown -= time.delta_seconds();
        }
        
        if keyboard_input.pressed(KeyCode::Space) && shooting.cooldown <= 0.0 {
            // Spawn projectile
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::YELLOW,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y + 30.0,
                        transform.translation.z,
                    ).with_scale(Vec3::new(5.0, 15.0, 1.0)),
                    ..default()
                },
                Projectile {
                    velocity: Vec2::new(0.0, 400.0),
                },
                Collision { radius: 5.0 },
            ));
            
            shooting.cooldown = 0.3; // 300ms cooldown
            shooting_stats.shots_fired += 1;
        }
    }
}

/// Handle projectile movement
pub fn projectile_movement(
    mut projectile_query: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    // Only run when game is playing and not paused
    if !game_state.playing || game_state.paused {
        return;
    }
    for (mut transform, projectile) in projectile_query.iter_mut() {
        transform.translation.x += projectile.velocity.x * time.delta_seconds();
        transform.translation.y += projectile.velocity.y * time.delta_seconds();
    }
}

/// Clean up projectiles that are off-screen
pub fn projectile_cleanup(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (entity, transform) in projectile_query.iter() {
        // Remove projectiles that are far off-screen
        if transform.translation.y > 1000.0 || transform.translation.y < -1000.0 ||
           transform.translation.x > 1000.0 || transform.translation.x < -1000.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Update shooting cooldowns
pub fn update_shooting_cooldowns(
    mut shooting_query: Query<&mut Shooting>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    // Only run when game is playing and not paused
    if !game_state.playing || game_state.paused {
        return;
    }
    for mut shooting in shooting_query.iter_mut() {
        if shooting.cooldown > 0.0 {
            shooting.cooldown -= time.delta_seconds();
        }
    }
}

/// Handle collision detection between projectiles and enemies
pub fn collision_detection(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Collision), With<Projectile>>,
    mut enemy_query: Query<(Entity, &Transform, &Collision, &mut Health), With<Enemy>>,
    mut shooting_stats: ResMut<ShootingStats>,
    game_state: Res<GameState>,
) {
    // Only run when game is playing and not paused
    if !game_state.playing || game_state.paused {
        return;
    }
    for (projectile_entity, projectile_transform, projectile_collision) in projectile_query.iter() {
        for (enemy_entity, enemy_transform, enemy_collision, mut enemy_health) in enemy_query.iter_mut() {
            let distance = projectile_transform.translation.distance(enemy_transform.translation);
            let collision_distance = projectile_collision.radius + enemy_collision.radius;
            
            if distance < collision_distance {
                // Hit detected
                enemy_health.current -= 25.0;
                shooting_stats.hits += 1;
                
                // Remove projectile
                commands.entity(projectile_entity).despawn();
                
                // Remove enemy if health <= 0
                if enemy_health.current <= 0.0 {
                    commands.entity(enemy_entity).despawn();
                }
                
                break; // Projectile can only hit one enemy
            }
        }
    }
}

/// Handle boundary collision for entities
pub fn boundary_collision(
    mut entity_query: Query<&mut Transform, With<Collision>>,
    game_state: Res<GameState>,
) {
    // Only run when game is playing and not paused
    if !game_state.playing || game_state.paused {
        return;
    }
    let boundary = 400.0;
    
    for mut transform in entity_query.iter_mut() {
        // Keep entities within boundaries
        transform.translation.x = transform.translation.x.clamp(-boundary, boundary);
        transform.translation.y = transform.translation.y.clamp(-boundary, boundary);
    }
}

/// Change enemy color based on health
pub fn enemy_color_change(
    mut enemy_query: Query<(&Health, &mut Sprite), With<Enemy>>,
    game_state: Res<GameState>,
) {
    // Only run when game is playing and not paused
    if !game_state.playing || game_state.paused {
        return;
    }
    for (health, mut sprite) in enemy_query.iter_mut() {
        let health_ratio = health.current / health.max;
        
        if health_ratio > 0.7 {
            sprite.color = Color::RED;
        } else if health_ratio > 0.3 {
            sprite.color = Color::ORANGE;
        } else {
            sprite.color = Color::rgb(0.5, 0.0, 0.0); // Dark red
        }
    }
}