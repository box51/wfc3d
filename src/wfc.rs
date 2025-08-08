use crate::objs::{Tile, TileType, Bond};


// Function to create a 3D array of Tiles
pub fn collapse_next_tile(tiles: Vec<Vec<Vec<Tile>>>) -> [Tile; 3] {
    // update entropy
    let x_dim = tiles.len();
    let y_dim = tiles[0].len();
    let z_dim = tiles[0][0].len();

    let mut min_entropy = usize::MAX;
    let mut min_entropy_pos = (0, 0, 0);

    for x in 0..x_dim {
        for y in 0..y_dim {
            for z in 0..z_dim {
                if !objs_3d[x][y][z].collapsed && objs_3d[x][y][z].entropy < min_entropy {
                    min_entropy = objs_3d[x][y][z].entropy;
                    min_entropy_pos = (x, y, z);
                }
            }
        }
    }

}
