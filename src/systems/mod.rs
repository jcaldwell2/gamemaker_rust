//! Game systems organized by functionality

use bevy::prelude::*;

pub mod input;
pub mod gameplay;
pub mod game_controls;
pub mod camera;
pub mod rendering;
pub mod editor;

use crate::components::*;
use crate::resources::*;

/// Setup the game engine with initial entities and camera
pub fn setup_engine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(50.0)),
            ..default()
        },
        Player,
        Health { current: 100.0, max: 100.0 },
        Shooting { cooldown: 0.0 },
        Collision { radius: 25.0 },
    ));

    // Spawn some enemies
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 150.0;
        let y = 200.0;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(40.0)),
                ..default()
            },
            Enemy,
            Health { current: 50.0, max: 50.0 },
            Collision { radius: 20.0 },
        ));
    }
}