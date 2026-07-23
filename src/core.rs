use bevy::prelude::*;
use serde::Deserialize;

/// BASIC ATTRIBUTES FOR HEROES AND MOBS
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

/// CAMERA COMMANDO
#[derive(Message, Debug, Clone, Copy)]
pub struct CameraPanCommand {
    pub direction: Vec2, // normalized direction vector for panning the camera
}

#[derive(Message, Debug, Clone, Copy)]
pub struct CameraZoomCommand {
    /// positive = zooming out, negative = zooming in
    /// because the camera's orthographic projection scale increases when zooming out and decreases when zooming in
    pub delta: f32,
}

#[derive(Resource)]
pub struct PanBounds {
    pub min: Vec2, // minimum x and y coordinates for camera panning
    pub max: Vec2, // maximum x and y coordinates for camera panning
}

/// DEBUG MODE STRUCTS
#[derive(Resource, Default)]
pub struct DebugMode(pub bool);
