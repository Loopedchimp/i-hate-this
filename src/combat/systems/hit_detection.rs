use bevy::prelude::*;
use crate::character::components::{Health, CombatStats, CombatState, Player, Enemy};
use crate::weapons::components::{Weapon, EquippedWeapon};
use crate::combat::components::{AttackHitbox, DamageEvent};
use crate::combat::systems::damage::calculate_damage;

#[derive(Component)]
pub struct HitboxOwner(pub Entity);

pub fn detect_hits(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &GlobalTransform, &AttackHitbox, &EquippedWeapon, &HitboxOwner)>,
    potential_target_query: Query<(Entity, &GlobalTransform, &Health, &CombatStats, &CombatState, Option<&Player>, Option<&Enemy>)>,
    weapon_query: Query<&Weapon>,
    attacker_query: Query<&CombatStats>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (hitbox_entity, hitbox_transform, hitbox, equipped, owner) in hitbox_query.iter() {
        let weapon = weapon_query.get(equipped.entity).unwrap();
        
        for (target_entity, target_transform, _, target_stats, target_state, is_player, is_enemy) in potential_target_query.iter() {
            // Skip self-damage
            if target_entity == owner.0 {
                continue;
            }
            
            // Calculate distance
            let distance = hitbox_transform.translation().distance(target_transform.translation());
            
            if distance <= hitbox.radius {
                // Hit detected!
                
                // Get attacker stats
                if let Ok(attacker_stats) = attacker_query.get(owner.0) {
                    // Calculate damage
                    let final_damage = calculate_damage(
                        attacker_stats,
                        weapon,
                        target_stats,
                        target_state.is_blocking,
                    );
                    
                    // Send damage event
                    damage_events.send(DamageEvent {
                        attacker: owner.0,
                        target: target_entity,
                        weapon: equipped.entity,
                        raw_damage: weapon.base_damage,
                        final_damage,
                    });
                    
                    // In a real game, you might want to add hit effects, sounds, etc. here
                }
            }
        }
        
        // Cleanup hitbox (simplified; in a real game this would be more sophisticated)
        commands.entity(hitbox_entity).despawn();
    }
}