// wfc.rs - Updated with directional connection logic
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    PosX, NegX,
    PosY, NegY,
    PosZ, NegZ,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::PosX => Direction::NegX,
            Direction::NegX => Direction::PosX,
            Direction::PosY => Direction::NegY,
            Direction::NegY => Direction::PosY,
            Direction::PosZ => Direction::NegZ,
            Direction::NegZ => Direction::PosZ,
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![
            Direction::PosX, Direction::NegX,
            Direction::PosY, Direction::NegY,
            Direction::PosZ, Direction::NegZ,
        ]
    }

    pub fn offset(&self) -> (i32, i32, i32) {
        match self {
            Direction::PosX => (1, 0, 0),
            Direction::NegX => (-1, 0, 0),
            Direction::PosY => (0, 1, 0),
            Direction::NegY => (0, -1, 0),
            Direction::PosZ => (0, 0, 1),
            Direction::NegZ => (0, 0, -1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TileType {
    pub id: usize,
    pub name: String,
    pub visual_type: String, // "cube", "corner", "road", "junction", "top", "empty"
    pub connections: HashMap<Direction, String>,
    pub weight: f32,
}

impl TileType {
    pub fn new(id: usize, name: &str, visual_type: &str,
        connections: HashMap<Direction, String>, weight: f32) -> Self {
            TileType {
                id,
                name: name.to_string(),
                visual_type: visual_type.to_string(),
                connections,
                weight,
            }
    }

    pub fn can_connect(&self, other: &TileType, dir: Direction) -> bool {
        // If I'm empty and looking up, only accept other empty blocks

        let my_conn = &self.connections[&dir];
        let other_conn = &other.connections[&dir.opposite()];

        // Connection compatibility rules
        let result = match (my_conn.as_str(), other_conn.as_str()) {
            ("none", "none") => true, // empty connections are compatible

            // None means no connection allowed
            ("none", _) | (_, "none") => false,

            // Basic connections
            ("flat", "flat") => true,
            ("male", "female") => true,
            ("female", "male") => true,

            // Special connections for roads
            ("road_end", "road_end") => true,

            // Ground connections
            ("ground", "ground") => true,
            ("ground", "flat") | ("flat", "ground") => true,

            // Same type connections
            _ => my_conn == other_conn,
        };
        result
    }
}

#[derive(Clone)]
pub struct Cell {
    pub position: (i32, i32, i32),
    pub possibilities: HashSet<usize>,
    pub collapsed: Option<usize>,
}

impl Cell {
    fn new(position: (i32, i32, i32), tile_types: &[TileType]) -> Self {
        let possibilities: HashSet<usize> = tile_types.iter().map(|t| t.id).collect();
        Cell {
            position,
            possibilities,
            collapsed: None,
        }
    }

    fn entropy(&self) -> usize {
        self.possibilities.len()
    }

    fn is_collapsed(&self) -> bool {
        self.collapsed.is_some()
    }

    fn collapse(&mut self, tile_types: &[TileType], rng: &mut impl Rng) {
        if self.possibilities.is_empty() {
            return;
        }

        // Weighted random selection
        let total_weight: f32 = self.possibilities.iter()
            .map(|&id| tile_types[id].weight)
            .sum();

        let mut random_value = rng.gen::<f32>() * total_weight;

        for &id in &self.possibilities {
            random_value -= tile_types[id].weight;
            if random_value <= 0.0 {
                self.collapsed = Some(id);
                self.possibilities.clear();
                self.possibilities.insert(id);
                break;
            }
        }
    }
}

pub struct WFCGrid {
    pub cells: HashMap<(i32, i32, i32), Cell>,
    pub tile_types: Vec<TileType>,
    pub dimensions: (usize, usize, usize),
}

impl WFCGrid {
    pub fn new(dimensions: (usize, usize, usize), tile_types: Vec<TileType>) -> Self {
        let mut cells = HashMap::new();

        // filling the grid with cells with all tile types as possibilities
        for x in 0..dimensions.0 as i32 {
            for y in 0..dimensions.1 as i32 {
                for z in 0..dimensions.2 as i32 {
                    let pos = (x, y, z);
                    cells.insert(pos, Cell::new(pos, &tile_types));
                }
            }
        }

        WFCGrid {
            cells,
            tile_types,
            dimensions,
        }
    }

    fn propagate(&mut self, start_pos: (i32, i32, i32)) {
        let mut stack = vec![start_pos];

        // local propagation loop to ease O(k) vs O(n) if computing entropy for all cells
        while let Some(pos) = stack.pop() {
            let current_cell = self.cells[&pos].clone();

            if !current_cell.is_collapsed() {
                continue;
            }

            let current_tile_id = current_cell.collapsed.unwrap();
            let current_tile = &self.tile_types[current_tile_id];

            for dir in Direction::all() {  // Loop through ALL 6 directions (up/down/left/right/forward/back)
                let offset = dir.offset();  // Get the offset for this direction (e.g., up = (0,1,0))
                let neighbor_pos = (pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2);  // Calculate neighbor position

                if let Some(neighbor) = self.cells.get(&neighbor_pos) {  // Get the neighbor cell
                    if neighbor.is_collapsed() {
                        continue;  // Skip if neighbor already has a fixed tile
                    }

                    // Now we update the neighbor's possibilities based on what we just placed
                    let old_count = neighbor.possibilities.len();
                    let mut valid_possibilities = HashSet::new();

                    // iterating over all possibilites of the neighbor (in the begining it will start with all tiles)
                    for &possible_id in &neighbor.possibilities {  // Check each possible tile for the neighbor
                        let current_tile = &self.tile_types[current_tile_id];  // The tile we just placed
                        let possible_tile = &self.tile_types[possible_id];  // A possible tile for neighbor

                        // THE KEY CHECK: Can the current tile connect to this possible neighbor tile?
                        if current_tile.can_connect(possible_tile, dir) {
                            valid_possibilities.insert(possible_id);  // Keep this possibility
                        }
                    }
                    print!("Valid possibilities for neighbor at {:?}: ", neighbor_pos);
                    for id in &valid_possibilities {
                        print!("{} ", self.tile_types[*id].name);
                    }
                    println!();

                    // Update the neighbor with only valid possibilities
                    let neighbor_mut = self.cells.get_mut(&neighbor_pos).unwrap();
                    neighbor_mut.possibilities = valid_possibilities;

                    // If we removed some possibilities, add this neighbor to the stack to propagate further
                    if neighbor_mut.possibilities.len() < old_count {
                        if !stack.contains(&neighbor_pos) {
                            stack.push(neighbor_pos);
                        }
                    }
                }
            }
        }
    }

    fn find_min_entropy_cell(&self) -> Option<(i32, i32, i32)> {
        self.cells.iter()
            .filter(|(_, cell)| !cell.is_collapsed() && cell.entropy() > 0)
            .min_by_key(|(_, cell)| cell.entropy())
            .map(|(&pos, _)| pos)
    }

    pub fn collapse_all(&mut self, rng: &mut impl Rng) -> bool {
        // Start with a random cell at the bottom layer for more stable structures
        let start_x = rng.gen_range(0..self.dimensions.0 as i32);
        let start_z = rng.gen_range(0..self.dimensions.2 as i32);
        let start_pos = (start_x, 0, start_z);

        if let Some(start_cell) = self.cells.get_mut(&start_pos) {
            start_cell.collapse(&self.tile_types, rng);
            self.propagate(start_pos);
        }

        // Continue collapsing remaining cells
        while let Some(min_pos) = self.find_min_entropy_cell() {
            let cell = self.cells.get_mut(&min_pos).unwrap();

            if cell.possibilities.is_empty() {
                return false; // Contradiction
            }

            cell.collapse(&self.tile_types, rng);
            self.propagate(min_pos);
        }

        true
    }
}
