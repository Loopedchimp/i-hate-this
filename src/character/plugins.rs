use bevy::prelude::*;
use super::systems::*;

#[derive(Default)]
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_stamina,
                reset_combat_timers,
                player_movement,
                apply_movement,
                simple_enemy_ai,
            ));
    }
}