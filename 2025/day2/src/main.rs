use std::fs;

fn is_invalid_id_part1(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must have even length to be a repeated pattern
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];

    // Check if first half equals second half (repeated exactly twice)
    first_half == second_half
}

fn is_invalid_id_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths (from 1 to len/2)
    // Pattern must repeat at least twice
    for pattern_len in 1..=len / 2 {
        // Check if the length is divisible by pattern length
        if len % pattern_len != 0 {
            continue;
        }

        let pattern = &s[..pattern_len];
        let mut is_valid_pattern = true;

        // Check if the entire string is made of repetitions of this pattern
        for i in (pattern_len..len).step_by(pattern_len) {
            if &s[i..i + pattern_len] != pattern {
                is_valid_pattern = false;
                break;
            }
        }

        if is_valid_pattern {
            return true;
        }
    }

    false
}

fn solve(input: &str, check_fn: fn(u64) -> bool) -> u64 {
    let ranges: Vec<&str> = input.trim().split(',').collect();
    let mut total = 0u64;

    for range in ranges {
        let parts: Vec<&str> = range.trim().split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for id in start..=end {
            if check_fn(id) {
                total += id;
            }
        }
    }

    total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <part1|part2>", args[0]);
        std::process::exit(1);
    }

    let part = &args[1];

    let check_fn = match part.as_str() {
        "part1" => is_invalid_id_part1 as fn(u64) -> bool,
        "part2" => is_invalid_id_part2 as fn(u64) -> bool,
        _ => {
            eprintln!("Invalid part: {}. Use 'part1' or 'part2'", part);
            std::process::exit(1);
        }
    };

    match fs::read_to_string("input.txt") {
        Ok(input) => {
            let result = solve(&input, check_fn);
            println!("Sum of invalid IDs: {}", result);
        }
        Err(error) => {
            eprintln!("Error reading input.txt: {}", error);
        }
    }
}
