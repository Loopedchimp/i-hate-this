use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct CombatSettings {
    pub friendly_fire: bool,
    pub hit_stop_duration: f32,
}

impl Default for CombatSettings {
    fn default() -> Self {
        Self {
            friendly_fire: false,
            hit_stop_duration: 0.1,
        }
    }
}