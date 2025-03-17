use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct AttackHitbox {
    pub radius: f32,
    pub damage_multiplier: f32,
}

impl Default for AttackHitbox {
    fn default() -> Self {
        Self {
            radius: 1.0,
            damage_multiplier: 1.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct DamageEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub weapon: Entity,
    pub raw_damage: f32,
    pub final_damage: f32,
}