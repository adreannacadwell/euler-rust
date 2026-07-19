use std::fs;

fn main() {
    let input = fs::read_to_string("grid.txt")
        .expect("failed to read grid.txt");

    let grid: Vec<Vec<u64>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().expect("invalid number"))
                .collect::<Vec<_>>()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut best: u64 = 0;

    for r in 0..rows {
        for c in 0..cols {
            // Right
            if c + 3 < cols {
                let p = grid[r][c]
                    * grid[r][c + 1]
                    * grid[r][c + 2]
                    * grid[r][c + 3];
                best = best.max(p);
            }

            // Down
            if r + 3 < rows {
                let p = grid[r][c]
                    * grid[r + 1][c]
                    * grid[r + 2][c]
                    * grid[r + 3][c];
                best = best.max(p);
            }

            // Diagonal down-right
            if r + 3 < rows && c + 3 < cols {
                let p = grid[r][c]
                    * grid[r + 1][c + 1]
                    * grid[r + 2][c + 2]
                    * grid[r + 3][c + 3];
                best = best.max(p);
            }

            // Diagonal down-left
            if r + 3 < rows && c >= 3 {
                let p = grid[r][c]
                    * grid[r + 1][c - 1]
                    * grid[r + 2][c - 2]
                    * grid[r + 3][c - 3];
                best = best.max(p);
            }
        }
    }

    println!("{}", best);
}