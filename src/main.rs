mod camera;
mod core;
mod game_input;
mod game_state;
mod hero;
mod mob;
mod time_wizard;

use avian2d::prelude::*;
use bevy::prelude::*;
use camera::CameraPlugin2D;
//use game_input::{DummyAction, Player, PlayerWalk, debug_player_walk, player_walks};
use game_state::{Economy, GameState, WaveState};
use leafwing_input_manager::prelude::*;
use time_wizard::TimeWizardPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin2D))
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.12)))
        .insert_resource(combined_mobs())
        .insert_resource(combined_heroes())
        .init_resource::<mob::WaveConfig>()
        .init_resource::<hero::PartyConfig>()
        .add_systems(
            Startup,
            (spawn_test_sprite, hero::spawn_party, mob::spawn_mobs),
        )
        .run();
}

fn spawn_test_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("graphics/32x32 Front-View Idle-Sheet4Frames.png");

    // 1. Definisikan ukuran potongan (16x16) dan jumlah kolom/baris
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        4, // Misalnya 4 frame animasi
        1, // Misalnya 1 baris
        None,
        None,
    );

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // 2. Spawn entity menggunakan struct Sprite secara langsung
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0, // Frame pertama
            }),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn combined_mobs() -> mob::MobList {
    let enemy_ron = include_str!("../assets/mobs/enemy_mobs.ron");
    let mut combined_mobs: mob::MobList =
        ron::from_str(enemy_ron).expect("Failed parsing enemy mobs from RON");

    let neutral_ron = include_str!("../assets/mobs/neutral_mobs.ron");
    let neutral_mobs: mob::MobList =
        ron::from_str(neutral_ron).expect("Failed parsing neutral mobs from RON");
    combined_mobs.mobs.extend(neutral_mobs.mobs);
    combined_mobs
}

fn combined_heroes() -> hero::HeroList {
    // load melee heroes
    let melee_ron = include_str!("../assets/heroes/melee_heroes.ron");
    let mut combined_heroes: hero::HeroList =
        ron::from_str(melee_ron).expect("Failed parsing melee heroes from RON");

    // load ranged heroes
    let ranged_ron = include_str!("../assets/heroes/ranged_heroes.ron");
    let ranged_heroes: hero::HeroList =
        ron::from_str(ranged_ron).expect("Failed parsing ranged heroes from RON");

    // combined those ron into one heroes list, so we can use it to spawn heroes in the game
    combined_heroes.heroes.extend(ranged_heroes.heroes);
    combined_heroes
}
