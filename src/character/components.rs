use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            regen_rate: 5.0, // Per second
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub strength: u32,
    pub dexterity: u32,
    pub defense: u32,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            defense: 10,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct CombatState {
    pub is_attacking: bool,
    pub is_blocking: bool,
    pub attack_timer: Timer,
    pub block_timer: Timer,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            is_attacking: false,
            is_blocking: false,
            attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
            block_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Movement {
    pub speed: f32,
    pub direction: Vec3,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 5.0,
            direction: Vec3::ZERO,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Player;

#[derive(Component, Debug, Clone)]
pub struct Enemy;