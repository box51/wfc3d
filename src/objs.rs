#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Top,
    Cube,
    Road,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bond {
    pub id: String,
    pub vectdir: [i32; 3]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub collapsed: bool,
    pub tile_type: TileType,
    pub entropy: u32,
    pub bonds: Vec<Bond>,
    pub starter: bool
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            collapsed: false,
            tile_type: TileType::Top, // Default type
            entropy: 0,
            bonds: [
                Bond {
                    id: "1".to_string(),
                    vectdir: [1, 0, 0]
                }
            ].to_vec(),
            starter: false
        }
    }
}

// Define the Position struct
#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
