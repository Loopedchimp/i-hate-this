use bevy::prelude::*;
use crate::character::components::{Movement, CombatState, Enemy, Player};
use crate::weapons::components::EquippedWeapon;
use rand::prelude::*;

pub fn simple_enemy_ai(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(Entity, &mut Movement, &mut CombatState, &mut Transform, &EquippedWeapon), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, mut movement, mut combat_state, mut transform, _) in enemy_query.iter_mut() {
            // Get direction to player
            let direction = player_transform.translation - transform.translation;
            let distance = direction.length();
            
            // If close enough to attack
            if distance < 2.0 && !combat_state.is_attacking && !combat_state.is_blocking {
                // 10% chance to attack per second
                if random::<f32>() < 0.1 * time.delta_seconds() {
                    combat_state.is_attacking = true;
                    combat_state.attack_timer.reset();
                }
                
                // Look at player
                if direction != Vec3::ZERO {
                    let look_target = transform.translation + direction;
                    transform.look_at(look_target, Vec3::Y);
                }
                
                // Stop moving
                movement.direction = Vec3::ZERO;
            } else {
                // Move towards player
                if direction != Vec3::ZERO {
                    // Look at player
                    let look_target = transform.translation + direction;
                    transform.look_at(look_target, Vec3::Y);
                    
                    // Set movement direction
                    movement.direction = direction.normalize();
                }
            }
        }
    }
}