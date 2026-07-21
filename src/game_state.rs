use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    SelectHeroes,
    InWave,
    Aftermath, // The state after all enemies spawned from waves been killed.
    //InBlind, // state before wave, player choose between 4 different style wave with different rewards.
    GameOver,
}

#[derive(Resource, Debug)]
pub struct WaveState {
    pub current_wave: u16,
    pub enemies_remaining: u16, // this number will increase with each wave, and will be decremented as enemies are killed
    pub wave_timer: f32,        // this timer will be used to determine how long the wave lasts
    pub intermission_timer: f32, // this timer will be used to determine how long the intermission between waves lasts
}
impl Default for WaveState {
    fn default() -> Self {
        Self {
            current_wave: 1,
            enemies_remaining: 0,
            wave_timer: 0.0,
            intermission_timer: 0.0,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Economy {
    pub crystals: u32,
}
impl Default for Economy {
    fn default() -> Self {
        Self { crystals: 0 } // starts with 0 crystals, so we could defined a starting amount of crystals in the future
    }
}
