use std::env;
use std::fs;
use std::process;

fn usage_and_exit() -> ! {
    eprintln!("Usage: cargo run -- <part1|part2>");
    process::exit(2);
}

type Grid = Vec<Vec<char>>;

fn read_grid() -> Grid {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    for line in &mut lines {
        if line.len() < width {
            line.push_str(&" ".repeat(width - line.len()));
        }
    }

    lines.iter().map(|l| l.chars().collect()).collect()
}

/// Identify all problems (column ranges)
fn find_problems(grid: &Grid) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    // Identify separator columns
    let mut is_sep = vec![false; width];
    for c in 0..width {
        is_sep[c] = (0..height).all(|r| grid[r][c] == ' ');
    }

    // Split into ranges
    let mut problems = Vec::new();
    let mut c = 0;
    while c < width {
        if is_sep[c] {
            c += 1;
            continue;
        }
        let start = c;
        while c < width && !is_sep[c] {
            c += 1;
        }
        problems.push((start, c));
    }

    problems
}

/// Solve Part 1
fn solve_part1(grid: &Grid) -> u128 {
    let height = grid.len();
    let problems = find_problems(grid);

    let mut grand: u128 = 0;

    for (start, end) in problems {
        // find operator
        let mut op = None;
        for col in start..end {
            let ch = grid[height - 1][col];
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("Missing operator in part1");

        // read numbers row by row
        let mut numbers: Vec<u128> = Vec::new();
        for row in 0..(height - 1) {
            let mut s = String::new();
            for col in start..end {
                s.push(grid[row][col]);
            }
            let st = s.trim();
            if !st.is_empty() {
                numbers.push(st.parse().unwrap());
            }
        }

        let mut acc = numbers[0];
        match op {
            '+' => {
                for &n in &numbers[1..] {
                    acc += n;
                }
            }
            '*' => {
                for &n in &numbers[1..] {
                    acc *= n;
                }
            }
            _ => unreachable!(),
        }

        grand += acc;
    }

    grand
}

/// Solve Part 2
fn solve_part2(grid: &Grid) -> u128 {
    let height = grid.len();
    let problems = find_problems(grid);

    let mut grand: u128 = 0;

    for (start, end) in problems {
        // find operator
        let mut op = None;
        for col in start..end {
            let ch = grid[height - 1][col];
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("Missing operator in part2");

        // read numbers column by column (top->bottom), then reverse order
        let mut nums: Vec<u128> = Vec::new();
        for col in start..end {
            let mut s = String::new();
            for row in 0..(height - 1) {
                s.push(grid[row][col]);
            }
            let st = s.trim();
            if !st.is_empty() {
                nums.push(st.parse().unwrap());
            }
        }

        nums.reverse(); // right-to-left order

        let mut acc = nums[0];
        match op {
            '+' => {
                for &n in &nums[1..] {
                    acc += n;
                }
            }
            '*' => {
                for &n in &nums[1..] {
                    acc *= n;
                }
            }
            _ => unreachable!(),
        }

        grand += acc;
    }

    grand
}

fn main() {
    let mut args = env::args().skip(1);
    let part = match args.next() {
        Some(x) => x,
        None => usage_and_exit(),
    };

    let grid = read_grid();

    match part.as_str() {
        "part1" => println!("{}", solve_part1(&grid)),
        "part2" => println!("{}", solve_part2(&grid)),
        _ => usage_and_exit(),
    }
}
