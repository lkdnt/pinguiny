/// Time utilities and reusable timer helpers used across the game.
///
/// This module provides three building blocks:
/// 1. `GameTimer`: an internal gameplay clock (elapsed time, pause, speed).
/// 2. `CooldownTimer`: a generic per-entity one-shot timer component.
/// 3. `Timer555`: a configurable waveform helper for pulse-like behavior.
///
/// Typical flow:
/// - Register `TimeWizardPlugin` in your app.
/// - Read `Res<GameTimer>` for UI and gameplay timing decisions.
/// - Attach `CooldownTimer` to entities that need local cooldowns.
/// - Use `Timer555` for oscillating values (VFX intensity, UI pulse, etc.).
use bevy::prelude::*;

/// Plugin that initializes time resources and updates timers every frame.
///
/// What this plugin does:
/// - Initializes the `GameTimer` resource with default values.
/// - Ticks the global gameplay clock in `Update`.
/// - Ticks every entity `CooldownTimer` in `Update`.
pub struct TimeWizardPlugin;

impl Plugin for TimeWizardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTimer>()
            .add_systems(Update, (tick_game_timer, tick_all_cooldowns));
    }
}

#[derive(Resource, Debug)]
/// Internal gameplay clock used as the source of truth for game time.
///
/// Unlike pausing Bevy's global virtual time, this timer is intentionally local
/// to gameplay logic. That keeps pause/speed behavior explicit and easier to debug.
pub struct GameTimer {
    pub elapsed_time: f64, // Total elapsed game time in seconds.
    pub is_running: bool,  // Whether the timer is currently advancing.
    pub speed: f32,        // Global time scale (1.0 = normal speed, 2.0 = double speed).
}

impl Default for GameTimer {
    fn default() -> Self {
        Self {
            elapsed_time: 0.0,
            is_running: true,
            speed: 1.0,
        }
    }
}

/// Formats seconds into "MM:SS" (for example, 90.0 -> "01:30").
pub fn format_time_to_mm_ss(seconds: f64) -> String {
    let total_seconds = seconds.max(0.0) as u32; // Clamp to non-negative values.
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

pub fn global_time_to_mm_ss(timer: &GameTimer) -> String {
    format_time_to_mm_ss(timer.elapsed_time)
}

/// Generic timer component that can be attached to any entity.
/// Useful for skill cooldowns, buff/debuff durations, and one-shot delays.
#[derive(Component, Debug)]
pub struct CooldownTimer {
    pub timer: Timer,
}

impl CooldownTimer {
    /// Creates a one-shot cooldown timer with the given duration in seconds.
    ///
    /// Usage:
    /// - Spawn with `CooldownTimer::new(2.0)` for a 2-second cooldown.
    /// - Let `tick_all_cooldowns` advance it every frame.
    /// - Check completion with `is_expired()`.
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }

    /// Returns `true` when the cooldown has finished.
    pub fn is_expired(&self) -> bool {
        self.timer.is_finished()
    }

    /// Resets the cooldown to its initial duration.
    pub fn reset(&mut self) {
        self.timer.reset();
    }
}

/// Advances the global gameplay clock every frame while `GameTimer` is running.
///
/// This is an internal system registered by `TimeWizardPlugin`.
fn tick_game_timer(mut game_timer: ResMut<GameTimer>, time: Res<Time>) {
    if game_timer.is_running {
        game_timer.elapsed_time += (time.delta_secs() as f64) * (game_timer.speed as f64);
    }
}

/// Advances all entity cooldown timers every frame.
///
/// This is an internal system registered by `TimeWizardPlugin`.
fn tick_all_cooldowns(time: Res<Time>, mut query: Query<&mut CooldownTimer>) {
    for mut cooldown in &mut query {
        cooldown.timer.tick(time.delta());
    }
}

/// Pauses the internal gameplay clock.
///
/// Note: this only stops `GameTimer` progression, not the entire engine update loop.
pub fn pause_game_time(mut game_timer: ResMut<GameTimer>) {
    game_timer.is_running = false;
}

/// Resumes the internal gameplay clock.
pub fn resume_game_time(mut game_timer: ResMut<GameTimer>) {
    game_timer.is_running = true;
}

/// Adjusts gameplay clock speed.
///
/// - `1.0` is normal speed.
/// - `0.5` is half speed.
/// - `2.0` is double speed.
///
/// Negative inputs are clamped to `0.0`.
pub fn set_game_speed(mut game_timer: ResMut<GameTimer>, speed_multiplier: f32) {
    game_timer.speed = speed_multiplier.max(0.0);
}

/// Reusable pulse-style waveform helper inspired by 555 timer behavior.
///
/// Formula:
/// `output = offset + amplitude * sin(TAU * frequency * t + phase)`
///
/// Practical uses:
/// - UI pulse animation.
/// - VFX glow intensity.
/// - Rhythm-like periodic triggers.
pub struct Timer555 {
    pub frequency: f64, // Oscillation rate in cycles per second.
    pub amplitude: f32, // Peak strength of the waveform.
    pub phase: f64,     // Horizontal phase shift of the waveform.
    pub offset: f64,    // Constant baseline added to the output.
}

impl Timer555 {
    /// Creates a new waveform generator.
    ///
    /// Parameter meaning:
    /// - `frequency`: cycles per second (Hz).
    /// - `amplitude`: waveform peak magnitude.
    /// - `phase`: phase offset in radians.
    /// - `offset`: baseline added to every sample.
    pub fn new(frequency: f64, amplitude: f32, phase: f64, offset: f64) -> Self {
        Self {
            frequency,
            amplitude,
            phase,
            offset,
        }
    }

    /// Returns the waveform value at the given time in seconds.
    ///
    /// This is the raw analog-like output and can be any real number.
    pub fn value_at(&self, time_seconds: f64) -> f64 {
        let angle = std::f64::consts::TAU * self.frequency * time_seconds + self.phase;
        self.offset + (self.amplitude as f64) * angle.sin()
    }

    /// Returns the waveform value at the current game time.
    ///
    /// Convenience wrapper around `value_at(timer.elapsed_time)`.
    pub fn value_from_game_timer(&self, timer: &GameTimer) -> f64 {
        self.value_at(timer.elapsed_time)
    }

    /// Returns a normalized waveform value in the range [0.0, 1.0].
    ///
    /// Useful when you need interpolation-friendly values (alpha, lerp factor, etc.).
    pub fn normalized_01_at(&self, time_seconds: f64) -> f64 {
        let angle = std::f64::consts::TAU * self.frequency * time_seconds + self.phase;
        (angle.sin() * 0.5) + 0.5
    }

    /// Returns true when the normalized value is above the threshold.
    ///
    /// This behaves like a simple digital gate derived from the analog waveform.
    pub fn gate_at(&self, time_seconds: f64, threshold: f64) -> bool {
        self.normalized_01_at(time_seconds) >= threshold
    }

    /// Returns the period in seconds (1/frequency), or None if frequency is not positive.
    ///
    /// Example: frequency `2.0` -> period `0.5` seconds.
    pub fn period_seconds(&self) -> Option<f64> {
        if self.frequency > 0.0 {
            Some(1.0 / self.frequency)
        } else {
            None
        }
    }
}
