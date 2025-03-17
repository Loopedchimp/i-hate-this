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
            .add_systems(Update, (
                spawn_attack_hitbox,
                cleanup_attack_hitboxes,
                detect_hits,
                apply_damage,
                check_death,
            ).chain());
    }
}