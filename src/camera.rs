use crate::core::DebugMode;
use crate::core::{CameraPanCommand, CameraZoomCommand, PanBounds};
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
                (
                    translate_camera_input,
                    move_camera_with_keys,
                    clamp_camera_pan,
                )
                    .chain(),
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
    commands.insert_resource(PanBounds {
        min: Vec2::new(-1200.0, -1200.0),
        max: Vec2::new(1200.0, 1200.0),
    });
}

fn move_camera_with_keys(
    time: Res<Time>,
    debug_mode: Res<DebugMode>,
    mut pan_reader: MessageReader<CameraPanCommand>,
    mut zoom_reader: MessageReader<CameraZoomCommand>,
    mut query: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    let max_zoom = if debug_mode.0 { 4.0 } else { 2.0 };

    for (mut transform, mut projection) in &mut query {
        for pan in pan_reader.read() {
            transform.translation.x += pan.direction.x * PAN_SPEED * time.delta_secs();
            transform.translation.y += pan.direction.y * PAN_SPEED * time.delta_secs();
        }

        for zoom in zoom_reader.read() {
            if let Projection::Orthographic(proj) = &mut *projection {
                proj.scale = (proj.scale + zoom.delta * ZOOM_SPEED).clamp(0.1, max_zoom);
            }
        }
    }
}

fn clamp_camera_pan(
    windows: Query<&Window>,
    bounds: Res<PanBounds>,
    mut query: Query<(&mut Transform, &Projection), With<MainCamera>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    for (mut transform, projection) in &mut query {
        if let Projection::Orthographic(proj) = projection {
            let half_viewport = Vec2::new(window.width(), window.height()) * 0.5 * proj.scale;

            let min_x = bounds.min.x + half_viewport.x;
            let max_x = bounds.max.x - half_viewport.x;
            let min_y = bounds.min.y + half_viewport.y;
            let max_y = bounds.max.y - half_viewport.y;

            transform.translation.x = if min_x <= max_x {
                transform.translation.x.clamp(min_x, max_x)
            } else {
                (bounds.min.x + bounds.max.x) * 0.5
            };

            transform.translation.y = if min_y <= max_y {
                transform.translation.y.clamp(min_y, max_y)
            } else {
                (bounds.min.y + bounds.max.y) * 0.5
            };
        }
    }
}
