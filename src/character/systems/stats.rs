use bevy::prelude::*;
use crate::character::components::{Stamina, CombatState};

pub fn update_stamina(
    time: Res<Time>,
    mut query: Query<&mut Stamina>,
) {
    for mut stamina in query.iter_mut() {
        stamina.current = (stamina.current + stamina.regen_rate * time.delta_secs())
            .min(stamina.max);
    }
}

pub fn reset_combat_timers(
    time: Res<Time>,
    mut query: Query<&mut CombatState>,
) {
    for mut state in query.iter_mut() {
        if state.is_attacking {
            state.attack_timer.tick(time.delta());
            if state.attack_timer.finished() {
                state.is_attacking = false;
                state.attack_timer.reset();
            }
        }
        
        if state.is_blocking {
            state.block_timer.tick(time.delta());
            if state.block_timer.finished() {
                state.is_blocking = false;
                state.block_timer.reset();
            }
        }
    }
}