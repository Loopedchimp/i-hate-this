use bevy::prelude::*;
use crate::character::components::{CombatState, CombatStats};
use crate::weapons::components::{Weapon, EquippedWeapon};
use crate::combat::components::AttackHitbox;
use crate::combat::systems::hit_detection::HitboxOwner;

pub fn spawn_attack_hitbox(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &CombatState, &EquippedWeapon), Changed<CombatState>>,
    weapon_query: Query<&Weapon>,
) {
    for (entity, transform, state, equipped) in query.iter() {
        if state.is_attacking && state.attack_timer.percent() < 0.1 {
            if let Ok(weapon) = weapon_query.get(equipped.entity) {
                // Create a hitbox in front of the character
                let hitbox_transform = Transform::from_translation(
                    transform.translation + transform.forward() * weapon.range
                );
                
                commands.spawn((
                    hitbox_transform,
                    AttackHitbox {
                        radius: weapon.range * 0.5,
                        damage_multiplier: 1.0,
                    },
                    EquippedWeapon { entity: equipped.entity },
                    HitboxOwner(entity),
                ));
            }
        }
    }
}

pub fn cleanup_attack_hitboxes(
    mut commands: Commands,
    query: Query<(Entity, &AttackHitbox)>,
    _time: Res<Time>,
) {
    for (entity, _) in query.iter() {
        // Despawn hitboxes after one frame
        // In a real game, you'd want more sophisticated logic here
        commands.entity(entity).despawn();
    }
}