// Game of Life, Week 12: grid, display, neighbor counts, the rule for one cell.
//
// TODO: remove the next line once every function below is used.

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

// All cells dead.
pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}
// '#' for a live cell, '.' for a dead cell, one line per row.
pub fn print_grid(grid: &Grid) {
    for row in grid.iter() {
        let mut line = String::new();
        for &cell in row.iter() {
            line.push(if cell { '#' } else { '.' });
        }
        println!("{}", line);
    }
}
// Live neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead. Careful: row - 1 on a usize of 0 panics.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count: u8 = 0;

    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            // the cell itself is not its own neighbor
            if dr == 0 && dc == 0 {
                continue;
            }

            // signed math, so row 0 minus 1 gives -1 instead of panicking
            let r = row as i32 + dr;
            let c = col as i32 + dc;

            // beyond the edge counts as dead
            if r < 0 || r >= ROWS as i32 || c < 0 || c >= COLS as i32 {
                continue;
            }

            // now it's safe to convert back and index
            if grid[r as usize][c as usize] {
                count += 1;
            }
        }
    }

    count
}
// The rule for one cell: does it live in the next generation?
pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    }
}
// Debug view: each cell's neighbor count as a digit, one line per row.
pub fn print_neighbor_counts(grid: &Grid) {
    for row in 0..ROWS {
        let mut line = String::new();
        for col in 0..COLS {
            line.push_str(&count_neighbors(grid, row, col).to_string());
        }
        println!("{}", line);
    }
}
// Set a few cells alive (hardcoded, or entered as `row col` pairs until a blank line),
// print the grid, then the neighbor counts.
pub fn play() {
    let mut grid = new_grid();

    // a vertical line of three in the middle (array indexes, counting from 0)
    grid[4][10] = true;
    grid[5][10] = true;
    grid[6][10] = true;

    println!("Grid:");
    print_grid(&grid);

    println!();
    println!("Neighbor counts:");
    print_neighbor_counts(&grid);

    // shows next_state in use without running a generation
    let centre = next_state(grid[5][10], count_neighbors(&grid, 5, 10));
    println!();
    println!("Centre cell alive in the next generation? {}", centre);
}
#[cfg(test)]
mod tests {
    use super::*;

    // Supplied by the instructor.
    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    #[test]
    fn live_4_dies() {
        assert!(!next_state(true, 4));
    }

    #[test]
    fn dead_2_stays_dead() {
        assert!(!next_state(false, 2));
    }

    #[test]
    fn live_0_dies() {
        assert!(!next_state(true, 0));
    }

    #[test]
    fn live_8_dies() {
        assert!(!next_state(true, 8));
    }

    #[test]
    fn live_3_survives() {
        assert!(next_state(true, 3));
    }
}
