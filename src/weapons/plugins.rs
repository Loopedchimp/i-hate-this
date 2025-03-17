use bevy::prelude::*;
use super::systems::weapon_behavior::process_weapon_input;

#[derive(Default)]
pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, process_weapon_input);
    }
}