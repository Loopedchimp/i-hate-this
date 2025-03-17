use bevy::prelude::*;
use crate::character::components::{Movement, Player};

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Movement, &mut Transform), With<Player>>,
) {
    for (mut movement, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        
        // Normalize direction vector if it's not zero
        if direction != Vec3::ZERO {
            direction = direction.normalize();
            
            // Rotate to face the direction of movement
            let target = transform.translation + direction;
            transform.look_at(target, Vec3::Y);
        }
        
        movement.direction = direction;
    }
}

pub fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&Movement, &mut Transform)>,
) {
    for (movement, mut transform) in query.iter_mut() {
        if movement.direction != Vec3::ZERO {
            transform.translation += movement.direction * movement.speed * time.delta_seconds();
        }
    }
}