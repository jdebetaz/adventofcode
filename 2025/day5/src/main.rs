use std::fs;

fn is_fresh(id: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (start, end) in ranges {
        if id >= *start && id <= *end {
            return true;
        }
    }
    false
}

fn solve_part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return 0;
    }

    // Separate ranges (contain '-') from IDs (don't contain '-')
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.contains('-') {
            let nums: Vec<&str> = trimmed.split('-').collect();
            if nums.len() == 2 {
                if let (Ok(start), Ok(end)) = (nums[0].parse::<u64>(), nums[1].parse::<u64>()) {
                    ranges.push((start, end));
                }
            }
        } else {
            if let Ok(id) = trimmed.parse::<u64>() {
                ids.push(id);
            }
        }
    }

    // Count fresh ones
    ids.iter().filter(|id| is_fresh(**id, &ranges)).count()
}

fn solve_part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return 0;
    }

    // Parse only ranges (contain '-')
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.contains('-') {
            let nums: Vec<&str> = trimmed.split('-').collect();
            if nums.len() == 2 {
                if let (Ok(start), Ok(end)) = (nums[0].parse::<u64>(), nums[1].parse::<u64>()) {
                    ranges.push((start, end));
                }
            }
        }
    }

    // Merge overlapping ranges to avoid double counting
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start position
    ranges.sort_by_key(|r| r.0);

    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    merged.push(ranges[0]);

    for i in 1..ranges.len() {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];
        let (curr_start, curr_end) = ranges[i];

        // If ranges overlap or are adjacent, merge them
        if curr_start <= last_end + 1 {
            merged[last_idx] = (last_start, last_end.max(curr_end));
        } else {
            merged.push((curr_start, curr_end));
        }
    }

    // Count total IDs in merged ranges
    let mut total = 0u64;
    for (start, end) in merged {
        total += end - start + 1;
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

    match fs::read_to_string("input.txt") {
        Ok(input) => match part.as_str() {
            "part1" => {
                let result = solve_part1(&input);
                println!("Fresh ingredients: {}", result);
            }
            "part2" => {
                let result = solve_part2(&input);
                println!("Total fresh IDs: {}", result);
            }
            _ => {
                eprintln!("Invalid part: {}. Use 'part1' or 'part2'", part);
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("Error reading input.txt: {}", error);
        }
    }
}
