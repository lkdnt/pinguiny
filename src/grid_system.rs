use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A priority node for the A* pathfinding algorithm
#[derive(PartialEq)]
pub struct PriorityNode {
    pub priority: f32,
    pub q: i32,
    pub r: i32,
}
// Implement Eq for PriorityNode to satisfy the requirements of BinaryHeap
impl Eq for PriorityNode {}

// Implement Ord for PriorityNode to define the ordering based on priority
impl Ord for PriorityNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order for min-heap behavior
        other
            .priority
            .partial_cmp(&self.priority)
            .unwrap_or(Ordering::Equal)
    }
}

// Implement PartialOrd for PriorityNode to allow comparison of nodes
impl PartialOrd for PriorityNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

    pub fn hex_distance(a_q: i32, a_r: i32, b_q: i32, b_r: i32) -> f32 {
        (((a_q - b_q).abs() + (a_q + a_r - b_q - b_r).abs() + (a_r - b_r).abs()) / 2) as f32
    }

    // bi-directional so we could meet in the middle, kind of.
    pub fn find_path_bidirectional(
        &self,
        start: (i32, i32),
        target: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        if !self
            .get_cell(start.0, start.1)
            .map(|c| c.walkable)
            .unwrap_or(false)
            || !self
                .get_cell(target.0, target.1)
                .map(|c| c.walkable)
                .unwrap_or(false)
        {
            return None;
        }

        if start == target {
            return Some(vec![start]);
        }

        let mut frontier_start = BinaryHeap::new();
        let mut frontier_target = BinaryHeap::new();

        frontier_start.push(PriorityNode {
            priority: 0.0,
            q: start.0,
            r: start.1,
        });
        frontier_target.push(PriorityNode {
            priority: 0.0,
            q: target.0,
            r: target.1,
        });

        let mut came_from_start: HashMap<(i32, i32), (i32, i32)> = HashMap::default();
        let mut came_from_target: HashMap<(i32, i32), (i32, i32)> = HashMap::default();

        let mut cost_so_far_start: HashMap<(i32, i32), f32> = HashMap::default();
        let mut cost_so_far_target: HashMap<(i32, i32), f32> = HashMap::default();

        cost_so_far_start.insert(start, 0.0);
        cost_so_far_target.insert(target, 0.0);

        let mut meeting_point: Option<(i32, i32)> = None;

        // Perform the conducting searches from both the start and target
        while !frontier_start.is_empty() && !frontier_target.is_empty() {
            // Expand from the start side
            if let Some(current_node) = frontier_start.pop() {
                let current = (current_node.q, current_node.r);

                if came_from_target.contains_key(&current) {
                    meeting_point = Some(current);
                    break;
                }

                for next in Self::neighbors(current.0, current.1) {
                    if let Some(cell) = self.get_cell(next.0, next.1) {
                        if cell.walkable {
                            let new_cost = cost_so_far_start[&current] + cell.cost;
                            if !cost_so_far_start.contains_key(&next)
                                || new_cost < cost_so_far_start[&next]
                            {
                                cost_so_far_start.insert(next, new_cost);
                                let priority = new_cost
                                    + Self::hex_distance(next.0, next.1, target.0, target.1);
                                frontier_start.push(PriorityNode {
                                    priority,
                                    q: next.0,
                                    r: next.1,
                                });
                                came_from_start.insert(next, current);
                            }
                        }
                    }
                }
            }

            // Expand from the target side
            if let Some(current_node) = frontier_target.pop() {
                let current = (current_node.q, current_node.r);

                if came_from_start.contains_key(&current) {
                    meeting_point = Some(current);
                    break;
                }

                for next in Self::neighbors(current.0, current.1) {
                    if let Some(cell) = self.get_cell(next.0, next.1) {
                        if cell.walkable {
                            let new_cost = cost_so_far_target[&current] + cell.cost;
                            if !cost_so_far_target.contains_key(&next)
                                || new_cost < cost_so_far_target[&next]
                            {
                                cost_so_far_target.insert(next, new_cost);
                                let priority =
                                    new_cost + Self::hex_distance(next.0, next.1, start.0, start.1);
                                frontier_target.push(PriorityNode {
                                    priority,
                                    q: next.0,
                                    r: next.1,
                                });
                                came_from_target.insert(next, current);
                            }
                        }
                    }
                }
            }
        }

        // If a meeting point was found, reconstruct the path
        if let Some(meet) = meeting_point {
            let mut path = Vec::new();

            // pull path from meeting point to start point
            let mut current = meet;
            while current != start {
                path.push(current);
                current = came_from_start[&current];
            }
            path.push(start);
            path.reverse();

            // pull path from meeting point to target point
            let mut current_target = meet;
            while current_target != target {
                current_target = came_from_target[&current_target];
                path.push(current_target);
            }

            return Some(path);
        }

        None
    }
}
