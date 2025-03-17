use bevy::prelude::*;
use crate::weapons::components::{Weapon, EquippedWeapon};
use crate::character::components::{CombatState, Stamina};

pub fn process_weapon_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut CombatState, &mut Stamina, &EquippedWeapon)>,
    weapon_query: Query<&Weapon>,
) {
    for (mut state, mut stamina, equipped) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && !state.is_attacking && !state.is_blocking {
            if let Ok(weapon) = weapon_query.get(equipped.entity) {
                if stamina.current >= weapon.stamina_cost {
                    state.is_attacking = true;
                    state.attack_timer.reset();
                    stamina.current -= weapon.stamina_cost;
                }
            }
        }
        
        if keyboard_input.just_pressed(KeyCode::ShiftLeft) && !state.is_attacking && !state.is_blocking {
            state.is_blocking = true;
            state.block_timer.reset();
        }
    }
}