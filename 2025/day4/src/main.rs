use std::fs;

fn count_adjacent_rolls(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check all 8 adjacent positions
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in directions.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        // Check bounds
        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            let r = new_row as usize;
            let c = new_col as usize;
            if grid[r][c] == '@' {
                count += 1;
            }
        }
    }

    count
}

fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let mut accessible_count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {
                let adjacent = count_adjacent_rolls(&grid, row, col);
                if adjacent < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    accessible_count
}

fn solve_part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let mut total_removed = 0;

    loop {
        // Find all accessible rolls in current state
        let mut to_remove = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '@' {
                    let adjacent = count_adjacent_rolls(&grid, row, col);
                    if adjacent < 4 {
                        to_remove.push((row, col));
                    }
                }
            }
        }

        // If no rolls can be removed, stop
        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in to_remove.iter() {
            grid[*row][*col] = '.';
        }

        total_removed += to_remove.len();
    }

    total_removed
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
