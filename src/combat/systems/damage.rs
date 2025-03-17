use bevy::prelude::*;
use crate::character::components::{Health, CombatStats};
use crate::weapons::components::Weapon;
use crate::combat::components::DamageEvent;

pub fn calculate_damage(
    attacker_stats: &CombatStats,
    weapon: &Weapon,
    defender_stats: &CombatStats,
    is_blocking: bool,
) -> f32 {
    let base_damage = weapon.base_damage;
    let strength_bonus = attacker_stats.strength as f32 * 0.1;
    let raw_damage = base_damage + strength_bonus;
    
    let defense_reduction = defender_stats.defense as f32 * 0.05;
    let block_reduction = if is_blocking { 0.7 } else { 0.0 };
    
    let final_damage = (raw_damage * (1.0 - defense_reduction)) * (1.0 - block_reduction);
    final_damage.max(1.0) // Minimum damage of 1
}

pub fn apply_damage(
    mut commands: Commands,
    mut query: Query<&mut Health>,
    mut damage_events: EventReader<DamageEvent>,
) {
    for event in damage_events.iter() {
        if let Ok(mut health) = query.get_mut(event.target) {
            health.current -= event.final_damage;
            
            // You could add effects, sounds, or other feedback here
            
            // Create a damage number entity (placeholder for visual feedback)
            commands.spawn(
                TransformBundle::from(Transform::from_translation(
                    Vec3::new(0.0, 1.0, 0.0)
                )),
            );
        }
    }
}

pub fn check_death(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.current <= 0.0 {
            // In a real game, you might want to handle death more gracefully
            // For now, we'll just despawn the entity
            commands.entity(entity).despawn_recursive();
        }
    }
}