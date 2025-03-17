use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Spear,
    Bow,
    Crossbow,
}

#[derive(Component, Debug, Clone)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub base_damage: f32,
    pub speed: f32,
    pub range: f32,
    pub stamina_cost: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            base_damage: 10.0,
            speed: 1.0,
            range: 1.5,
            stamina_cost: 10.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct EquippedWeapon {
    pub entity: Entity,
}