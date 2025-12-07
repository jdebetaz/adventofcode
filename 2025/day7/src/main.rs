use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    row: i32,
    col: i32,
}

fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // Find starting position 'S'
    let mut start_col = 0;
    for (c, &ch) in grid[0].iter().enumerate() {
        if ch == 'S' {
            start_col = c as i32;
            break;
        }
    }

    // Track which splitters have been hit
    let mut split_splitters = HashSet::new();

    // Track active beams
    let mut beams = VecDeque::new();
    beams.push_back(Beam {
        row: 0,
        col: start_col,
    });

    // Track beam positions we've already queued to avoid infinite loops
    let mut queued = HashSet::new();
    queued.insert((0, start_col));

    // Process beams
    while let Some(beam) = beams.pop_front() {
        let mut current = beam;

        // Move beam downward until it hits a splitter or exits
        loop {
            current.row += 1;

            // Check if beam exits the manifold
            if current.row >= rows {
                break;
            }

            let cell = grid[current.row as usize][current.col as usize];

            if cell == '^' {
                // Only count this splitter if we haven't split it before
                split_splitters.insert((current.row, current.col));

                // Create two new beams from left and right of splitter
                let left_col = current.col - 1;
                let right_col = current.col + 1;

                if left_col >= 0 && queued.insert((current.row, left_col)) {
                    beams.push_back(Beam {
                        row: current.row,
                        col: left_col,
                    });
                }

                if right_col >= 0 && right_col < cols && queued.insert((current.row, right_col)) {
                    beams.push_back(Beam {
                        row: current.row,
                        col: right_col,
                    });
                }

                break; // Original beam stops
            }
        }
    }

    split_splitters.len()
}

fn solve_part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // Find starting position 'S'
    let mut start_col = 0;
    for (c, &ch) in grid[0].iter().enumerate() {
        if ch == 'S' {
            start_col = c as i32;
            break;
        }
    }

    use std::collections::HashMap;

    // Memoization: for each (row, col), store how many paths lead from there to exit
    let mut memo: HashMap<(i32, i32), usize> = HashMap::new();

    fn count_paths(
        grid: &Vec<Vec<char>>,
        row: i32,
        col: i32,
        rows: i32,
        cols: i32,
        memo: &mut HashMap<(i32, i32), usize>,
    ) -> usize {
        // Check memo first
        if let Some(&count) = memo.get(&(row, col)) {
            return count;
        }

        let mut current_row = row;
        let current_col = col;

        // Move downward until hitting a splitter or exiting
        loop {
            current_row += 1;

            // Exit condition - this is one complete timeline
            if current_row >= rows {
                return 1;
            }

            let cell = grid[current_row as usize][current_col as usize];

            if cell == '^' {
                let mut total = 0;

                // Left timeline
                let left_col = current_col - 1;
                if left_col >= 0 {
                    total += count_paths(grid, current_row, left_col, rows, cols, memo);
                }

                // Right timeline
                let right_col = current_col + 1;
                if right_col < cols {
                    total += count_paths(grid, current_row, right_col, rows, cols, memo);
                }

                memo.insert((row, col), total);
                return total;
            }
        }
    }

    count_paths(&grid, 0, start_col, rows, cols, &mut memo)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <part1|part2>", args[0]);
        std::process::exit(1);
    }

    let part = &args[1];

    match fs::read_to_string("input.txt") {
        Ok(input) => {
            let result = match part.as_str() {
                "part1" => solve_part1(&input),
                "part2" => solve_part2(&input),
                _ => {
                    eprintln!("Invalid part: {}. Use 'part1' or 'part2'", part);
                    std::process::exit(1);
                }
            };

            println!("Result: {}", result);
        }
        Err(error) => {
            eprintln!("Error reading input.txt: {}", error);
        }
    }
}
