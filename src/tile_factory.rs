// use crate::objs::{Tile, TileType, Bond};
// use rand::Rng;

// const TILES_NUM : u32 = 3;  // ideally this would be dynaimcally calculated

// // Function to create a 3D array of Tiles
// pub fn create_tile_options() -> [Tile; 3] {
//     [
//         Tile {
//             collapsed: false,
//             tile_type: TileType::Cube,
//             entropy: TILES_NUM,
//             bonds: vec![
//                 Bond {
//                     id: "f2f".to_string(),
//                     vectdir: [1, 0, 0]
//                 },
//                 Bond {
//                     id: "f2f".to_string(),
//                     vectdir: [-1, 0, 0]
//                 },
//                 Bond {
//                     id: "t2b".to_string(),
//                     vectdir: [0, 1, 0]
//                 },
//                 Bond {
//                     id: "b2t".to_string(),
//                     vectdir: [0, -1, 0]
//                 },
//                 Bond {
//                     id: "f2f".to_string(),
//                     vectdir: [0, 0, 1]
//                 },
//                 Bond {
//                     id: "f2f".to_string(),
//                     vectdir: [0, 0, -1]
//                 },
//             ],
//             starter: true
//         },
//         Tile {
//             collapsed: false,
//             tile_type: TileType::Top,
//             entropy: TILES_NUM,
//             bonds: vec![
//                 Bond {
//                     id: "t2b".to_string(),
//                     vectdir: [0, -1, 0]
//                 },
//             ],
//             starter: false
//         },
//         Tile {
//             collapsed: false,
//             tile_type: TileType::Road,
//             entropy: TILES_NUM,
//             bonds: vec![
//                 Bond {
//                     id: "f2f".to_string(),
//                     vectdir: [1, 0, 0]
//                 },
//                 Bond {
//                     id: "f2f".to_string(),
//                     vectdir: [-1, 0, 0]
//                 },
//                 Bond {
//                     id: "t2b".to_string(),
//                     vectdir: [0, -1, 0]
//                 },
//             ],
//             starter: true
//         },
//     ]

// }

// pub fn select_random_start_tile(tiles: &Vec<Tile>) -> Tile {
//     let start_tiles: Vec<Tile> = tiles.iter()
//         .filter(|tile| tile.starter)
//         .cloned()
//         .collect();

//     // if start_tiles.is_empty() {
//     //     return None;
//     // }

//     let mut rng = rand::thread_rng();
//     let random_index = rng.gen_range(0..start_tiles.len());
//     // Some(start_tiles[random_index].clone())
//     return start_tiles[random_index].clone()
// }

// pub fn update_tile_after_collapse(tile: &mut Tile, dir: [i32; 3]) {
//     tile.collapsed = true;
//     tile.entropy = 0;

//     // ESTÁS AQUI! O MELHOR É PASSAR SEMPRE OS BONDS COMO ARGUMENTO QUE FORAM USADOS; TIPO ESTA PEÇA FAZ FRONTEIRA COM X E y

//     if tile.tile_type == TileType::Top {
//         for bond in tile.bonds.iter_mut() {
//             if bond.vectdir == dir {
//                 bond.id = "b2t".to_string();
//             }
//         }
//         let possible_bonds = vec![
//             Bond {
//                 id: "t2b".to_string(),
//                 vectdir: [0, -1, 0]
//             },
//         ];
//     }
//     if tile.tile_type == TileType::Cube {
//         tile.bonds = vec![
//             Bond {
//                 id: "f2f".to_string(),
//                 vectdir: [1, 0, 0]
//             },
//             Bond {
//                 id: "f2f".to_string(),
//                 vectdir: [-1, 0, 0]
//             },
//             Bond {
//                 id: "t2b".to_string(),
//                 vectdir: [0, 1, 0]
//             },
//             Bond {
//                 id: "b2t".to_string(),
//                 vectdir: [0, -1, 0]
//             },
//             Bond {
//                 id: "f2f".to_string(),
//                 vectdir: [0, 0, 1]
//             },
//             Bond {
//                 id: "f2f".to_string(),
//                 vectdir: [0, 0, -1]
//             },
//         ];
//     }
// }