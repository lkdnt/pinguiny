use crate::core::{CameraPanCommand, CameraZoomCommand};
use crate::game_input::{CameraAction, translate_camera_input};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const PAN_SPEED: f32 = 250.0; // units per second
const ZOOM_SPEED: f32 = 0.05; // scale units per second

pub struct CameraPlugin2D;

impl Plugin for CameraPlugin2D {
    fn build(&self, app: &mut App) {
        app.add_message::<CameraPanCommand>()
            .add_message::<CameraZoomCommand>()
            .add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (translate_camera_input, move_camera_with_keys).chain(),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Camera"),
        MainCamera,
        Camera2d,
        Camera::default(),
        Projection::Orthographic(OrthographicProjection::default_2d()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CameraAction::default_input_map(),
        ActionState::<CameraAction>::default(),
    ));
}

fn move_camera_with_keys(
    time: Res<Time>,
    mut pan_reader: MessageReader<CameraPanCommand>,
    mut zoom_reader: MessageReader<CameraZoomCommand>,
    mut query: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    for (mut transform, mut projection) in &mut query {
        for pan in pan_reader.read() {
            transform.translation.x += pan.direction.x * PAN_SPEED * time.delta_secs();
            transform.translation.y += pan.direction.y * PAN_SPEED * time.delta_secs();
        }

        for zoom in zoom_reader.read() {
            if let Projection::Orthographic(proj) = &mut *projection {
                proj.scale = (proj.scale + zoom.delta * ZOOM_SPEED).clamp(0.1, 4.0);
            }
        }
    }
}
