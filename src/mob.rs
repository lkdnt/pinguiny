use crate::core::{DamageType, Health, Mana, MoveSpeed};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// flag for enemy mob
/// these mobs are hostile and will attack the player on sight
#[derive(Component)]
pub struct Enemy;

/// flag for neutral mob
/// these mobs are not hostile
/// but some of them could be hostile if they are provoked
/// some other would run away if they are provoked
#[derive(Component)]
pub struct Neutral;

#[derive(Component, Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum MobType {
    // hostile
    SlimeMage, // a slime that can cast spells, need mana doesnt need name
    Skeleton,  // a skeleton that can attack with a sword, doesnt need mana doesnt need name
    MiniBoss,  // a mini boss that can attack with a sword and magic, need mana but needs name

    // neutral
    Cow,  // a cow is neutral will run away if provoked, doesnt need mana and doesnt need name
    Wolf, // a wolf is neutral will attack if provoked, doesnt need mana and doesnt need name
}

/// basic bundle for mobs
#[derive(Bundle, Clone, Deserialize)]
pub struct BaseMobBundle {
    pub mob_type: MobType,
    pub health: Health,
    pub move_speed: MoveSpeed,
}

#[derive(Clone, Deserialize)]
pub struct MobDataInfo {
    pub base: BaseMobBundle,
    pub name: Option<String>,
    pub damage: Option<DamageType>,
    pub mana: Option<Mana>,
}

#[derive(Resource, Deserialize)]
pub struct MobList {
    pub mobs: HashMap<String, MobDataInfo>,
}

#[derive(Resource)]
pub struct WaveConfig {
    pub mobs_to_spawn: Vec<String>, // List of mob IDs to spawn in this wave.
}

impl Default for WaveConfig {
    fn default() -> Self {
        Self {
            mobs_to_spawn: vec![
                "SlimeMage".to_string(),
                "Skeleton".to_string(),
                "Gothmog".to_string(),
            ],
        }
    }
}

pub fn spawn_mobs(mut commands: Commands, config: Res<WaveConfig>, library: Res<MobList>) {
    for mob_id in &config.mobs_to_spawn {
        if let Some(mob_data) = library.mobs.get(mob_id) {
            let mut entity = commands.spawn(mob_data.base.clone());

            match mob_data.base.mob_type {
                MobType::SlimeMage | MobType::Skeleton | MobType::MiniBoss => {
                    entity.insert(Enemy);
                }
                MobType::Cow | MobType::Wolf => {
                    entity.insert(Neutral);
                }
            }

            if let Some(name_component) = &mob_data.name {
                entity.insert(Name::new(name_component.clone()));
            }

            if let Some(damage_component) = &mob_data.damage {
                entity.insert(damage_component.clone());
            }

            if let Some(mana_component) = &mob_data.mana {
                entity.insert(mana_component.clone());
            }

            info!("Spawned mob: {}", mob_id);
        } else {
            warn!("Mob ID '{}' not found in MobList. Skipping spawn.", mob_id);
        }
    }
}
