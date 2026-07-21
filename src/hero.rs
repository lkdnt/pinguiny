use crate::core::{DamageType, Health, Mana, MoveSpeed};
/// here we gonna makes the hero to be ready to be used in the game, like a player character or a main character
/// we will take attributes from the core module and make a hero struct that will be used in the game
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// flag for slot of the first hero
#[derive(Component)]
pub struct FirstHero;

// flag for slot of the second hero
#[derive(Component)]
pub struct SecondHero;

/// we want to have at least 2 heroes in gameplay, so we will make a hero struct that will be used to spawn the heroes in the game
/// the idea is one attacking hero and one defending hero, so we will make a hero struct that will be used to spawn the heroes in the game
/// but it also could be 2 attacking heroes or 2 defending heroes, but for now we will make one attacking hero and one defending hero
/// we dont need to hard coded about attacking or defending, because each of them could be instructed to be attacking or defending, so we will make a hero struct that will be used to spawn the heroes in the game
/// so we will just saying that there are first heroes to spawn and the second heroes to spawn.
#[derive(Bundle, Clone, Deserialize)]
pub struct BaseHeroBundle {
    pub health: Health,
    pub move_speed: MoveSpeed,
    pub damage: DamageType,
}

#[derive(Clone, Deserialize)]
pub struct HeroDataInfo {
    pub base: BaseHeroBundle,
    pub name: String,
    pub mana: Option<Mana>,
}

#[derive(Resource, Deserialize)]
pub struct HeroList {
    pub heroes: HashMap<String, HeroDataInfo>,
}

/// party config
#[derive(Resource)]
pub struct PartyConfig {
    /// we used Option<String> so the slot could be empty (None) if only picked one hero
    pub slot_one: Option<String>, // name of the hero in slot one
    pub slot_two: Option<String>, // name of the hero in slot two
}

// start from none, so the party config could be empty if no hero is picked
// so we could pick whoever we wanted.
impl Default for PartyConfig {
    fn default() -> Self {
        Self {
            slot_one: Some("Gandalf".to_string()),
            slot_two: None,
        }
    }
}

pub fn spawn_party(mut commands: Commands, config: Res<PartyConfig>, library: Res<HeroList>) {
    // check and spawn Slot 1
    if let Some(hero_id) = &config.slot_one {
        if let Some(hero_data) = library.heroes.get(hero_id) {
            let mut entity = commands.spawn(hero_data.base.clone());
            entity.insert(FirstHero);

            entity.insert(Name::new(hero_data.name.clone()));

            if let Some(mana_component) = &hero_data.mana {
                entity.insert(mana_component.clone());
            }
            info!("Spawned hero in slot 1: {}", hero_id);
        }
    }

    // check and spawn Slot 2
    if let Some(hero_id) = &config.slot_two {
        if let Some(hero_data) = library.heroes.get(hero_id) {
            let mut entity = commands.spawn(hero_data.base.clone());
            entity.insert(SecondHero);

            entity.insert(Name::new(hero_data.name.clone()));

            if let Some(mana_component) = &hero_data.mana {
                entity.insert(mana_component.clone());
            }
            info!("Spawned hero in slot 2: {}", hero_id);
        }
    }
}
