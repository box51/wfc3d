use crate::objs::{Tile, TileType, Bond};

const TILES_NUM : u32 = 3;

// Function to create a 3D array of Tiles
pub fn create_tile_options() -> [Tile; 3] {
    [
        Tile {
            collapsed: false,
            tile_type: TileType::Cube,
            entropy: TILES_NUM,
            bonds: vec![
                Bond {
                    id: "face".to_string(),
                    vectdir: [1, 0, 0]
                },
                Bond {
                    id: "face".to_string(),
                    vectdir: [-1, 0, 0]
                },
                Bond {
                    id: "top".to_string(),
                    vectdir: [0, 1, 0]
                },
                Bond {
                    id: "face".to_string(),
                    vectdir: [0, -1, 0]
                },
                Bond {
                    id: "face".to_string(),
                    vectdir: [0, 0, 1]
                },
                Bond {
                    id: "face".to_string(),
                    vectdir: [0, 0, -1]
                },
            ],
            starter: true
        },
        Tile {
            collapsed: false,
            tile_type: TileType::Top,
            entropy: TILES_NUM,
            bonds: vec![
                Bond {
                    id: "top".to_string(),
                    vectdir: [0, -1, 0]
                },
            ],
            starter: false
        },
        Tile {
            collapsed: false,
            tile_type: TileType::Top,
            entropy: TILES_NUM,
            bonds: vec![
                Bond {
                    id: "face".to_string(),
                    vectdir: [1, 0, 0]
                },
                Bond {
                    id: "face".to_string(),
                    vectdir: [-1, 0, 0]
                },
                Bond {
                    id: "face".to_string(),
                    vectdir: [0, -1, 0]
                },
            ],
            starter: true
        },
    ]

}
