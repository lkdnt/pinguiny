mod core;
mod game_input;
mod game_state;
mod hero;
mod mob;
mod time_wizard;

use avian2d::prelude::*;
use bevy::prelude::*;
use game_input::{DummyAction, Player, PlayerWalk, debug_player_walk, player_walks};
use game_state::{Economy, GameState, WaveState};
use leafwing_input_manager::prelude::*;
use time_wizard::TimeWizardPlugin;

fn main() {
    let enemy_ron = include_str!("../assets/mobs/enemy_mobs.ron");
    let mut combined_mobs: mob::MobList =
        ron::from_str(enemy_ron).expect("Failed parsing enemy mobs from RON");

    let neutral_ron = include_str!("../assets/mobs/neutral_mobs.ron");
    let neutral_mobs: mob::MobList =
        ron::from_str(neutral_ron).expect("Failed parsing neutral mobs from RON");
    combined_mobs.mobs.extend(neutral_mobs.mobs);

    // load melee heroes
    let melee_ron = include_str!("../assets/heroes/melee_heroes.ron");
    let mut heroes_list: hero::HeroList =
        ron::from_str(melee_ron).expect("Failed parsing melee heroes from RON");

    // load ranged heroes
    let ranged_ron = include_str!("../assets/heroes/ranged_heroes.ron");
    let ranged_heroes: hero::HeroList =
        ron::from_str(ranged_ron).expect("Failed parsing ranged heroes from RON");

    // combined those ron into one heroes list, so we can use it to spawn heroes in the game
    heroes_list.heroes.extend(ranged_heroes.heroes);
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(combined_mobs)
        .insert_resource(heroes_list)
        .init_resource::<mob::WaveConfig>()
        .init_resource::<hero::PartyConfig>()
        .add_systems(Startup, (hero::spawn_party, mob::spawn_mobs))
        .run();
}
