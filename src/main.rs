use bevy::prelude::*;
use combat::CombatPlugin;
use character::CharacterPlugin;
use weapons::WeaponsPlugin;
use ui::UIPlugin;

mod combat;
mod character;
mod weapons;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CharacterPlugin)
        .add_plugins(WeaponsPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(UIPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Set up camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Add lighting
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    
    // Spawn player
    let player_mesh = meshes.add(Mesh::from(shape::Capsule::default()));
    let player_material = materials.add(Color::rgb(0.2, 0.7, 0.2).into());
    
    let player = commands.spawn((
        PbrBundle {
            mesh: player_mesh.clone(),
            material: player_material,
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        character::components::Health::default(),
        character::components::Stamina::default(),
        character::components::CombatStats::default(),
        character::components::CombatState::default(),
        character::components::Movement::default(),
        character::components::Player,
    )).id();
    
    // Spawn player's weapon
    let sword = commands.spawn((
        weapons::components::Weapon {
            weapon_type: weapons::components::WeaponType::Sword,
            base_damage: 15.0,
            speed: 1.2,
            range: 1.5,
            stamina_cost: 10.0,
        },
    )).id();
    
    // Equip weapon to player
    commands.entity(player).insert(weapons::components::EquippedWeapon {
        entity: sword,
    });
    
    // Spawn enemy
    let enemy_mesh = meshes.add(Mesh::from(shape::Capsule::default()));
    let enemy_material = materials.add(Color::rgb(0.7, 0.2, 0.2).into());
    
    let enemy = commands.spawn((
        PbrBundle {
            mesh: enemy_mesh,
            material: enemy_material,
            transform: Transform::from_xyz(3.0, 1.0, 0.0),
            ..default()
        },
        character::components::Health::default(),
        character::components::Stamina::default(),
        character::components::CombatStats::default(),
        character::components::CombatState::default(),
        character::components::Movement {
            speed: 3.0,
            direction: Vec3::ZERO,
        },
        character::components::Enemy,
    )).id();
    
    // Spawn enemy's weapon
    let axe = commands.spawn((
        weapons::components::Weapon {
            weapon_type: weapons::components::WeaponType::Axe,
            base_damage: 20.0,
            speed: 0.8,
            range: 1.3,
            stamina_cost: 15.0,
        },
    )).id();
    
    // Equip weapon to enemy
    commands.entity(enemy).insert(weapons::components::EquippedWeapon {
        entity: axe,
    });
    
    // Add a simple ground plane
    let ground_mesh = meshes.add(Mesh::from(shape::Plane::from_size(10.0)));
    let ground_material = materials.add(Color::rgb(0.3, 0.3, 0.3).into());
    
    commands.spawn(PbrBundle {
        mesh: ground_mesh,
        material: ground_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}