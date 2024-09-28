fn assess_neighborhood(position: (i32, i32)) {
    let (x, y) = position;

    // Assess the neighborhood based on the position
    // You can add your logic here

    // Example: Print the position and its neighbors
    println!("Position: ({}, {})", x, y);
    println!("Neighbors:");
    println!("({}, {})", x - 1, y);
    println!("({}, {})", x + 1, y);
    println!("({}, {})", x, y - 1);
    println!("({}, {})", x, y + 1);
}

fn assess_neighborhood(position: (i32, i32), matrix: &mut Vec<Vec<Tile>>) {
    let (x, y) = position;

    // Assess the neighborhood based on the position
    // You can add your logic here

    // Example: Print the position and its neighbors
    println!("Position: ({}, {})", x, y);
    println!("Neighbors:");
    println!("({}, {})", x - 1, y);
    println!("({}, {})", x + 1, y);
    println!("({}, {})", x, y - 1);
    println!("({}, {})", x, y + 1);

    // Decrease entropy of the tile if it has only 1 possible bond
    if let Some(tile) = matrix.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
        if let Some(tile) = matrix.get_mut(y as usize).and_then(|row| row.get_mut(x as usize)) {
            for bond in &tile.bonds {
                let bond_direction = bond.vectdir;
                let neighbor_x = x + bond_direction[0];
                let neighbor_y = y + bond_direction[1];
                let neighbor_z = bond_direction[2];
                if let Some(neighbor_tile) = matrix.get_mut(neighbor_y as usize).and_then(|row| row.get_mut(neighbor_x as usize)) {
                    neighbor_tile.entropy -= 1;
                }
            }

        }
    }
}