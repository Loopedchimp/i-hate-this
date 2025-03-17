use bevy::prelude::*;
use crate::character::components::{CombatState, CombatStats};

pub fn calculate_block_effectiveness(
    combat_stats: &CombatStats,
) -> f32 {
    // Simple block effectiveness calculation based on defense stat
    // In a real game, you might consider more factors
    0.5 + (combat_stats.defense as f32 * 0.01)
}

// More defense-related systems would go here