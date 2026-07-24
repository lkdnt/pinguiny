use crate::core::MoveSpeed;
use crate::grid_system::HexNavGrid;
use bevy::prelude::*;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hex_nav_grid)
            .add_systems(Update, (debug_draw_hex_grid, move_along_path));
    }
}
/// component for navigating the entity to find the path
#[derive(Component)]
pub struct NavPath {
    // we use standar vec
    pub path: Vec<(i32, i32)>,
}

pub fn move_along_path(
    time: Res<Time>,
    grid: Res<HexNavGrid>,

    mut query: Query<(Entity, &mut Transform, &MoveSpeed, &mut NavPath)>,
    mut commands: Commands,
) {
    for (entity, mut transform, speed, mut nav) in &mut query {
        if let Some(&(target_q, target_r)) = nav.path.last() {
            let target_pos = grid.hex_to_world(target_q, target_r);
            let current_pos = transform.translation.truncate();

            let direction = target_pos - current_pos;
            let distance = direction.length();

            if distance < 1.0 {
                nav.path.pop();
            } else {
                let move_delta = direction.normalize() * speed.speed * time.delta_secs();

                if move_delta.length() > distance {
                    transform.translation.x = target_pos.x;
                    transform.translation.y = target_pos.y;
                } else {
                    transform.translation.x += move_delta.x;
                    transform.translation.y += move_delta.y;
                }
            }
        } else {
            // no more path, remove the component
            commands.entity(entity).remove::<NavPath>();
        }
    }
}

fn setup_hex_nav_grid(mut commands: Commands) {
    let mut grid = HexNavGrid::new(80, 70, 16.0); // radius 10, hex size 32.0

    // wall_cells is a list of axial coordinates (q, r) that are not walkable
    let wall_cells = [
        (2, -1),
        (2, 0),
        (2, 1),
        (2, 2),
        (-3, 1),
        (-3, 3),
        (0, -3),
        (1, -3),
        (2, -3),
    ];

    for (q, r) in wall_cells {
        grid.set_cell(q, r, false, f32::INFINITY);
    }
    commands.insert_resource(grid);
}

/// Debug system to visualize the hex grid in the world
/// green for walkable, red for non-walkable
fn debug_draw_hex_grid(grid: Res<HexNavGrid>, mut gizmos: Gizmos) {
    // walkable first
    for ((q, r), cell_data) in grid.iter_cells().filter(|(_, c)| c.walkable) {
        let center = grid.hex_to_world(q, r);
        let color = Color::srgb(0.0, 1.0, 0.0); // green
        draw_hexagon_outline(&mut gizmos, center, grid.hex_size, color);
    }

    // blocked draw after walkable so it is on top
    for ((q, r), cell_data) in grid.iter_cells().filter(|(_, c)| !c.walkable) {
        let center = grid.hex_to_world(q, r);
        let color = Color::srgb(1.0, 0.0, 0.0); // red
        draw_hexagon_outline(&mut gizmos, center, grid.hex_size, color);
    }
}

fn draw_hexagon_outline(gizmos: &mut Gizmos, center: Vec2, size: f32, color: Color) {
    let mut points = [Vec2::ZERO; 6];
    for i in 0..6 {
        let angle_deg = 60.0 * i as f32 - 30.0;
        let angle_rad = angle_deg.to_radians();
        points[i] = center + Vec2::new(angle_rad.cos(), angle_rad.sin()) * size;
    }

    for i in 0..6 {
        let start = points[i];
        let end = points[(i + 1) % 6];
        gizmos.line_2d(start, end, color);
    }
}
