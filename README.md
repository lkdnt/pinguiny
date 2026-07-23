# Pinguiny 🐧 ⚔️

**Pinguiny** is a 2D Wave-Defense / Strategy game developed using **Rust** and the **Bevy Engine**. This game challenges players to assemble the right party, choose the right item and weapon to modifiers its hero attributes, withstand enemy waves, and manage resources (Crystals) in every wave.

## 🎮 Gameplay Features

- **Hero Selection System:** Choose a combination of two heroes (*Melee* or *Ranged*, e.g. such as Gandalf and Herobrine) before the battle begins to find the best synergy.
- **Wave System & Timer:** Face enemy waves managed by a dedicated `time_wizard` module. Enemies will spawn gradually, giving players time to prepare.
- **Dynamic Mobs:** Features a diverse ecosystem of entities:
  - **Hostile Mobs:** Enemies that attack on sight (e.g. Slime Mage, Skeleton, and Mini-Bosses like Gothmog).
  - **Neutral Mobs:** Animals like cows that will run away when attacked, or wolves that will bite back if provoked.
- **Crystal Economy:** Collect *Crystals* as the primary currency, but be careful as enemies can steal these resources from the player's vault!

## 🛠️ Tech Stack & Architecture (Headless-First)

This game is built with a solid **Data-Driven Architecture** foundation on top of Bevy's ECS (*Entity Component System*). Logic and data are strictly separated to ensure scalability:

- **Engine:** [Bevy Engine](https://bevyengine.org/)
- **Data Serialization:** Uses the `RON` (*Rusty Object Notation*) format via `serde` to define hero stats, enemies, and wave compositions without hardcoding them in Rust.
- **State Driven:** Game transitions (such as `SelectHeroes`, `InWave`, `Aftermath`, and `GameOver`) are managed using strict Bevy States.

## 📂 Main Project Structure

- `assets/` : Stores visual assets (16x16 Sprite Sheets) and `.ron` data configurations.
- `src/`
  - `core.rs` : Contains fundamental attributes like *Health*, *Mana*, *MoveSpeed*, and *DamageType*.
  - `hero.rs` & `mob.rs` : Handles component logic and character spawning from RON files.
  - `game_state.rs` : Manages game flow states and the `Economy` vault.
  - `time_wizard.rs` : The central time controller for wave durations and intermission intervals.
  - `game_input.rs` : Handles player keyboard and mouse inputs.
  - `camera.rs` : Manages camera movement and zooming based on player input.


basic attributes [x]
basic spawn system for heroes and mobs [x]
basic camera panning and zooming [x]
AI pathfinding for heroes [ ]
AI pathfinding for mobs [ ]
basic combat system [ ]
implement combat system for heroes [ ]
implement combat system for mobs [ ]
