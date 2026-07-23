use crate::core::{CameraPanCommand, CameraZoomCommand, DebugMode};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// CAMERA INPUT ACTION SEMANTICS
#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum CameraAction {
    PanUp,    // ArrowUp
    PanDown,  // ArrowDown
    PanLeft,  // ArrowLeft
    PanRight, // ArrowRight
    ZoomIn,   // [
    ZoomOut,  // ]
}

impl CameraAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::PanUp, KeyCode::ArrowUp);
        input_map.insert(Self::PanDown, KeyCode::ArrowDown);
        input_map.insert(Self::PanLeft, KeyCode::ArrowLeft);
        input_map.insert(Self::PanRight, KeyCode::ArrowRight);
        input_map.insert(Self::ZoomIn, KeyCode::BracketLeft);
        input_map.insert(Self::ZoomOut, KeyCode::BracketRight);
        input_map
    }
}

pub fn translate_camera_input(
    query: Query<&ActionState<CameraAction>>,
    mut pan_writer: MessageWriter<CameraPanCommand>,
    mut zoom_writer: MessageWriter<CameraZoomCommand>,
) {
    for action_state in &query {
        let mut direction = Vec2::ZERO;

        if action_state.pressed(&CameraAction::PanLeft) {
            direction.x -= 1.0;
        }
        if action_state.pressed(&CameraAction::PanRight) {
            direction.x += 1.0;
        }
        if action_state.pressed(&CameraAction::PanDown) {
            direction.y -= 1.0;
        }
        if action_state.pressed(&CameraAction::PanUp) {
            direction.y += 1.0;
        }

        if direction != Vec2::ZERO {
            pan_writer.write(CameraPanCommand {
                direction: direction.normalize_or_zero(),
            });
        }

        if action_state.pressed(&CameraAction::ZoomOut) {
            zoom_writer.write(CameraZoomCommand { delta: 1.0 });
        }
        if action_state.pressed(&CameraAction::ZoomIn) {
            zoom_writer.write(CameraZoomCommand { delta: -1.0 });
        }
    }
}

/// DEBUG MODE TOGGLE
pub fn toggle_debug_mode(keyboard: Res<ButtonInput<KeyCode>>, mut debug_mode: ResMut<DebugMode>) {
    if keyboard.just_pressed(KeyCode::F3) {
        debug_mode.0 = !debug_mode.0;
        info!("Debug mode toggled: {}", debug_mode.0);
    }
}
