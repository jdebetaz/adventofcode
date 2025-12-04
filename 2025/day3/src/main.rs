use std::fs;

fn max_joltage_from_bank_part1(bank: &str) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let n = digits.len();

    if n < 2 {
        return 0;
    }

    let mut max_joltage = 0;

    // Try all pairs of positions (i, j) where i < j
    for i in 0..n {
        for j in (i + 1)..n {
            // Form a two-digit number from digits[i] and digits[j]
            let first = digits[i].to_digit(10).unwrap();
            let second = digits[j].to_digit(10).unwrap();
            let joltage = (first * 10 + second) as u64;

            max_joltage = max_joltage.max(joltage);
        }
    }

    max_joltage
}

fn max_joltage_from_bank_part2(bank: &str) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let n = digits.len();

    if n < 12 {
        return 0;
    }

    // We need to select exactly 12 batteries (keep their order)
    // To maximize, we want to keep the largest digits, especially at the front

    // Strategy: Greedily select the 12 largest digits while maintaining order
    // We need to skip (n - 12) batteries
    let skip_count = n - 12;

    let mut result = String::new();
    let mut skipped = 0;
    let mut pos = 0;

    // For each of the 12 positions we need to fill
    for selected in 0..12 {
        let remaining_to_select = 12 - selected;
        let can_skip = skip_count - skipped;

        // Look ahead and find the largest digit we can afford to select
        let mut best_digit = '0';
        let mut best_pos = pos;

        // We can look ahead at most 'can_skip + 1' positions
        let look_ahead = (can_skip + 1).min(n - pos);

        for i in 0..look_ahead {
            if digits[pos + i] > best_digit {
                best_digit = digits[pos + i];
                best_pos = pos + i;
            }
        }

        result.push(best_digit);
        skipped += best_pos - pos;
        pos = best_pos + 1;
    }

    result.parse::<u64>().unwrap_or(0)
}

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|bank| max_joltage_from_bank_part1(bank))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|bank| max_joltage_from_bank_part2(bank))
        .sum()
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

            println!("Total output joltage: {}", result);
        }
        Err(error) => {
            eprintln!("Error reading input.txt: {}", error);
        }
    }
}
