use bevy::prelude::*;
use super::systems::*;

#[derive(Default)]
pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, process_weapon_input);
    }
}