use std::fs;

fn solve_safe_dial_part1(input: &str) -> i32 {
    let rotations: Vec<&str> = input.trim().lines().collect();

    let mut current_position = 50;
    let mut zero_count = 0;

    for rotation in rotations {
        let direction = rotation.chars().next().unwrap();
        let distance: i32 = rotation[1..].parse().unwrap();

        if direction == 'L' {
            current_position = (current_position - distance).rem_euclid(100);
        } else if direction == 'R' {
            current_position = (current_position + distance) % 100;
        }

        if current_position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn solve_safe_dial_part2(input: &str) -> i32 {
    let rotations: Vec<&str> = input.trim().lines().collect();

    let mut current_position = 50;
    let mut zero_count = 0;

    for rotation in rotations {
        let direction = rotation.chars().next().unwrap();
        let distance: i32 = rotation[1..].parse().unwrap();

        if direction == 'L' {
            // Moving left (decreasing): count each click that lands on 0
            for _ in 0..distance {
                current_position = if current_position == 0 {
                    99
                } else {
                    current_position - 1
                };
                if current_position == 0 {
                    zero_count += 1;
                }
            }
        } else if direction == 'R' {
            // Moving right (increasing): count each click that lands on 0
            for _ in 0..distance {
                current_position = if current_position == 99 {
                    0
                } else {
                    current_position + 1
                };
                if current_position == 0 {
                    zero_count += 1;
                }
            }
        }
    }

    zero_count
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
            let password = match part.as_str() {
                "part1" => solve_safe_dial_part1(&input),
                "part2" => solve_safe_dial_part2(&input),
                _ => {
                    eprintln!("Invalid part: {}. Use 'part1' or 'part2'", part);
                    std::process::exit(1);
                }
            };

            println!("Password: {}", password);
        }
        Err(error) => {
            eprintln!("Error reading input.txt: {}", error);
        }
    }
}
