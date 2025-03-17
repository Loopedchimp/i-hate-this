use bevy::prelude::*;
use super::systems::*;
use super::resources::CombatSettings;
use super::components::DamageEvent;

#[derive(Default)]
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CombatSettings>()
            .add_event::<DamageEvent>()
            .add_systems(Update, spawn_attack_hitbox)
            .add_systems(Update, cleanup_attack_hitboxes.after(spawn_attack_hitbox))
            .add_systems(Update, detect_hits.after(cleanup_attack_hitboxes))
            .add_systems(Update, apply_damage.after(detect_hits))
            .add_systems(Update, check_death.after(apply_damage));
    }
}