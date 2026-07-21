use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Clone, Deserialize)]
pub struct MoveSpeed {
    pub speed: f32,
}

#[derive(Component, Clone, Deserialize)]
pub enum DamageType {
    Physical(f32),
    Magic(f32),
    Hybrid { physical: f32, magic: f32 },
}

#[derive(Component, Clone, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component, Clone, Deserialize)]
pub struct Mana {
    pub current: f32,
    pub max: f32,
}
