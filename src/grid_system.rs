use bevy::math::VectorSpace;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

/// THIS IS A HEXAGONAL GRID SYSTEM FOR LOGIC NAVIGATION OF ENTITIES EITHER HEROES OR MOBS
#[derive(Clone, Copy, Debug)]
pub struct CellData {
    pub walkable: bool,
    pub cost: f32,
}

impl Default for CellData {
    fn default() -> Self {
        Self {
            walkable: true,
            cost: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct HexNavGrid {
    cells: Vec<CellData>, // Store cell data in a flat vector
    radius_q: i32,        // The radius of the grid in the q direction (axial coordinate)
    radius_r: i32,        // The radius of the grid in the r direction (axial coordinate)
    width: i32,
    pub hex_size: f32, // The size of each hexagon (distance from center to any corner)
}

impl HexNavGrid {
    pub fn new(radius_q: i32, radius_r: i32, hex_size: f32) -> Self {
        let width = radius_q * 2 + 1; // Calculate the width based on the radius
        let height = radius_r * 2 + 1; // Calculate the height based on the radius
        let size = (width * height) as usize; // Total number of cells in the grid
        Self {
            cells: vec![CellData::default(); size],
            radius_q,
            radius_r,
            width,
            hex_size,
        }
    }

    /// conversion (q, r) -> index array
    fn index(&self, q: i32, r: i32) -> usize {
        let x = (q + self.radius_q) as usize;
        let y = (r + self.radius_r) as usize;
        y * self.width as usize + x
    }

    /// check if the given axial coordinates (q, r) are within the bounds of the hex grid
    pub fn in_bounds(&self, q: i32, r: i32) -> bool {
        q >= -self.radius_q && q <= self.radius_q && r >= -self.radius_r && r <= self.radius_r
    }

    pub fn get_cell(&self, q: i32, r: i32) -> Option<&CellData> {
        if !self.in_bounds(q, r) {
            return None;
        }
        self.cells.get(self.index(q, r))
    }

    pub fn set_cell(&mut self, q: i32, r: i32, walkable: bool, cost: f32) {
        if !self.in_bounds(q, r) {
            return;
        }
        let idx = self.index(q, r);
        self.cells[idx] = CellData { walkable, cost };
    }

    pub fn neighbors(q: i32, r: i32) -> [(i32, i32); 6] {
        [
            (q - 1, r + 1), // bottom-left
            (q - 1, r),     // left
            (q, r - 1),     // top-left
            (q + 1, r - 1), // top-right
            (q + 1, r),     // right
            (q, r + 1),     // bottom-right
        ]
    }

    pub fn hex_to_world(&self, q: i32, r: i32) -> Vec2 {
        let q_f = q as f32;
        let r_f = r as f32;
        let sqrt3 = 3.0_f32.sqrt();

        let x = self.hex_size * (sqrt3 * q_f + sqrt3 / 2.0 * r_f);
        let y = self.hex_size * (3.0 / 2.0 * r_f);

        Vec2::new(x, y)
    }

    pub fn world_to_hex(&self, pos: Vec2) -> (i32, i32) {
        let sqrt3 = 3.0_f32.sqrt();

        let frac_q = (sqrt3 / 3.0 * pos.x - 1.0 / 3.0 * pos.y) / self.hex_size;
        let frac_r = (2.0 / 3.0 * pos.y) / self.hex_size;

        Self::axial_round(frac_q, frac_r)
    }

    fn axial_round(frac_q: f32, frac_r: f32) -> (i32, i32) {
        let frac_s = -frac_q - frac_r;

        let mut rq = frac_q.round();
        let mut rr = frac_r.round();
        let rs = frac_s.round();

        let q_diff = (rq - frac_q).abs();
        let r_diff = (rr - frac_r).abs();
        let s_diff = (rs - frac_s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        }

        (rq as i32, rr as i32)
    }

    pub fn get_cell_at_world(&self, pos: Vec2) -> Option<&CellData> {
        let (q, r) = self.world_to_hex(pos);
        self.get_cell(q, r)
    }

    /// iterat over all cells with coordinates
    pub fn iter_cells(&self) -> impl Iterator<Item = ((i32, i32), &CellData)> {
        self.cells.iter().enumerate().map(move |(idx, cell)| {
            let x = (idx as i32) % self.width;
            let y = (idx as i32) / self.width;
            let q = x - self.radius_q;
            let r = y - self.radius_r;
            ((q, r), cell)
        })
    }
}

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hex_nav_grid)
            .add_systems(Update, debug_draw_hex_grid);
    }
}

fn setup_hex_nav_grid(mut commands: Commands) {
    let mut grid = HexNavGrid::new(85, 70, 16.0); // radius 10, hex size 32.0

    // wall_cells is a list of axial coordinates (q, r) that are not walkable
    let wall_cells = [
        (2, -1),
        (2, 0),
        (2, 1),
        (2, 2),
        (-3, 1),
        (-3, 2),
        (-3, 3),
        (0, -3),
        (1, -3),
        (2, -3),
        (-1, 2),
        (-2, 2),
        (-3, 2),
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
