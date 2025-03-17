use bevy::prelude::*;
use crate::character::components::{Health, Stamina, Player};
use crate::ui::components::{HealthBar, StaminaBar, PlayerUI};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root UI node
    commands
        .spawn((
            Node {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            PlayerUI,
        ))
        .with_children(|parent| {
            // Health bar container
            parent
                .spawn((
                    Node {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Health bar fill
                    parent.spawn((
                        Node {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::srgb(0.8, 0.2, 0.2).into(),
                            ..default()
                        },
                        HealthBar,
                    ));
                });

            // Stamina bar container
            parent
                .spawn((
                    Node {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(20.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Stamina bar fill
                    parent.spawn((
                        Node {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.8, 0.2).into(),
                            ..default()
                        },
                        StaminaBar,
                    ));
                });
        });
}

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Node, With<HealthBar>>,
) {
    if let Ok(health) = player_query.get_single() {
        if let Ok(mut node) = health_bar_query.get_single_mut() {
            let health_percentage = (health.current / health.max) * 100.0;
            node.style.width = Val::Percent(health_percentage);
        }
    }
}

pub fn update_stamina_bar(
    player_query: Query<&Stamina, With<Player>>,
    mut stamina_bar_query: Query<&mut Node, With<StaminaBar>>,
) {
    if let Ok(stamina) = player_query.get_single() {
        if let Ok(mut node) = stamina_bar_query.get_single_mut() {
            let stamina_percentage = (stamina.current / stamina.max) * 100.0;
            node.style.width = Val::Percent(stamina_percentage);
        }
    }
}