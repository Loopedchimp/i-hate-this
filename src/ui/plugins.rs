use bevy::prelude::*;
use super::systems::*;

#[derive(Default)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (
                update_health_bar,
                update_stamina_bar,
            ));
    }
}