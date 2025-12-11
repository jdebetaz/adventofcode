use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Parse target state in [square brackets]
        let bracket_start = line.find('[').unwrap();
        let bracket_end = line.find(']').unwrap();
        let target_str = &line[bracket_start + 1..bracket_end];
        let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

        // Parse buttons in (parentheses)
        let mut buttons = Vec::new();
        let rest = &line[bracket_end + 1..];

        let mut i = 0;
        while i < rest.len() {
            if rest.chars().nth(i) == Some('(') {
                let end = rest[i..].find(')').unwrap() + i;
                let button_str = &rest[i + 1..end];
                let button: Vec<usize> = button_str
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                buttons.push(button);
                i = end + 1;
            } else {
                i += 1;
            }
        }

        // Parse joltage requirements in {curly braces}
        let mut joltage = Vec::new();
        if let Some(brace_start) = rest.find('{') {
            if let Some(brace_end) = rest.find('}') {
                let joltage_str = &rest[brace_start + 1..brace_end];
                joltage = joltage_str
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
            }
        }

        machines.push(Machine {
            target,
            buttons,
            joltage,
        });
    }

    machines
}

fn solve_machine(machine: &Machine) -> usize {
    let n_lights = machine.target.len();
    let n_buttons = machine.buttons.len();

    // Create augmented matrix [A | b] where A is the button matrix and b is the target
    let mut matrix = vec![vec![false; n_buttons + 1]; n_lights];

    for (col, button) in machine.buttons.iter().enumerate() {
        for &light in button {
            matrix[light][col] = true;
        }
    }

    for (row, &target_bit) in machine.target.iter().enumerate() {
        matrix[row][n_buttons] = target_bit;
    }

    // Gaussian elimination over GF(2)
    let mut pivot_col = vec![None; n_lights];
    let mut row = 0;

    for col in 0..n_buttons {
        let mut pivot_row = None;
        for r in row..n_lights {
            if matrix[r][col] {
                pivot_row = Some(r);
                break;
            }
        }

        if let Some(pr) = pivot_row {
            matrix.swap(row, pr);
            pivot_col[row] = Some(col);

            for r in 0..n_lights {
                if r != row && matrix[r][col] {
                    for c in 0..=n_buttons {
                        matrix[r][c] ^= matrix[row][c];
                    }
                }
            }

            row += 1;
        }
    }

    // Check for inconsistency
    for r in row..n_lights {
        if matrix[r][n_buttons] {
            return usize::MAX;
        }
    }

    // Find free variables
    let mut is_pivot = vec![false; n_buttons];
    let mut basic_vars = Vec::new();
    for r in 0..row {
        if let Some(col) = pivot_col[r] {
            is_pivot[col] = true;
            basic_vars.push((r, col));
        }
    }

    let mut free_vars = Vec::new();
    for col in 0..n_buttons {
        if !is_pivot[col] {
            free_vars.push(col);
        }
    }

    let n_free = free_vars.len();
    let mut min_presses = usize::MAX;

    // Try all combinations of free variables
    for mask in 0..(1 << n_free) {
        let mut solution = vec![false; n_buttons];

        for (i, &var) in free_vars.iter().enumerate() {
            solution[var] = (mask >> i) & 1 == 1;
        }

        for &(r, col) in basic_vars.iter().rev() {
            let mut val = matrix[r][n_buttons];
            for c in (col + 1)..n_buttons {
                if matrix[r][c] {
                    val ^= solution[c];
                }
            }
            solution[col] = val;
        }

        if solution.iter().all(|&x| x || !x) {
            let presses = solution.iter().filter(|&&x| x).count();
            min_presses = min_presses.min(presses);
        }
    }

    min_presses
}

fn solve_machine_part2(machine: &Machine) -> usize {
    let n_counters = machine.joltage.len();
    let n_buttons = machine.buttons.len();

    // Create matrix where A[i][j] = 1 if button j affects counter i
    let mut matrix = vec![vec![0i64; n_buttons + 1]; n_counters];

    for (col, button) in machine.buttons.iter().enumerate() {
        for &counter in button {
            if counter < n_counters {
                matrix[counter][col] = 1;
            }
        }
    }

    for (row, &target_val) in machine.joltage.iter().enumerate() {
        matrix[row][n_buttons] = target_val;
    }

    // Gaussian elimination
    let mut pivot_col = vec![None; n_counters];
    let mut row = 0;

    for col in 0..n_buttons {
        let mut pivot_row = None;
        for r in row..n_counters {
            if matrix[r][col] != 0 {
                pivot_row = Some(r);
                break;
            }
        }

        if let Some(pr) = pivot_row {
            matrix.swap(row, pr);
            pivot_col[row] = Some(col);

            // Don't divide - just eliminate using cross-multiplication
            // to avoid losing information with integer division
            for r in 0..n_counters {
                if r != row && matrix[r][col] != 0 {
                    // Eliminate: row[r] = row[r] * pivot - row[pivot] * factor
                    let pivot = matrix[row][col];
                    let factor = matrix[r][col];
                    for c in 0..=n_buttons {
                        matrix[r][c] = matrix[r][c] * pivot - matrix[row][c] * factor;
                    }
                }
            }

            row += 1;
        }
    }

    // Check for inconsistency
    for r in row..n_counters {
        if matrix[r][n_buttons] != 0 {
            return usize::MAX;
        }
    }

    // Identify basic and free variables
    let mut is_basic = vec![false; n_buttons];
    let mut basic_vars = Vec::new();
    for r in 0..row {
        if let Some(col) = pivot_col[r] {
            is_basic[col] = true;
            basic_vars.push((r, col));
        }
    }

    let mut free_vars = Vec::new();
    for col in 0..n_buttons {
        if !is_basic[col] {
            free_vars.push(col);
        }
    }

    let n_free = free_vars.len();

    // If no free variables, unique solution
    if n_free == 0 {
        let mut solution = vec![0i64; n_buttons];
        for &(r, col) in &basic_vars {
            // Divide by the pivot coefficient
            let pivot = matrix[r][col];
            if pivot == 0 {
                return usize::MAX;
            }
            let rhs = matrix[r][n_buttons];
            if rhs % pivot != 0 {
                return usize::MAX; // Not an integer solution
            }
            solution[col] = rhs / pivot;
        }

        if solution.iter().all(|&x| x >= 0) {
            return solution.iter().sum::<i64>() as usize;
        } else {
            return usize::MAX;
        }
    }

    // For systems with free variables, use brute force with pruning
    let sum_joltage: i64 = machine.joltage.iter().sum();

    // Use sum of joltages as bound - this is necessary for correctness
    let max_bound = sum_joltage;

    // Calculate individual bounds for each free variable
    let mut free_var_bounds = vec![max_bound; n_free];

    let mut min_presses = usize::MAX;
    let mut iteration_count = 0;
    // Dynamically adjust max iterations based on problem size
    let estimated_space: i64 = free_var_bounds.iter().product();
    let max_iterations = if estimated_space < 1_000_000 {
        estimated_space as usize * 2
    } else {
        1_000_000_000 // 1 billion iterations max
    };

    fn search(
        free_vars: &[usize],
        free_var_bounds: &[i64],
        idx: usize,
        current: &mut Vec<i64>,
        matrix: &[Vec<i64>],
        basic_vars: &[(usize, usize)],
        n_buttons: usize,
        min_presses: &mut usize,
        iteration_count: &mut usize,
        max_iterations: usize,
    ) -> bool {
        if *iteration_count >= max_iterations {
            return false; // Timeout
        }
        *iteration_count += 1;
        if idx == free_vars.len() {
            let mut solution = vec![0i64; n_buttons];

            for (i, &var) in free_vars.iter().enumerate() {
                solution[var] = current[i];
            }

            for &(r, col) in basic_vars.iter().rev() {
                let pivot = matrix[r][col];
                if pivot == 0 {
                    return true; // Skip invalid configuration
                }
                let mut val = matrix[r][n_buttons];
                for c in (col + 1)..n_buttons {
                    val -= matrix[r][c] * solution[c];
                }
                // Check if divisible
                if val % pivot != 0 {
                    return true; // Not a valid integer solution
                }
                solution[col] = val / pivot;
            }

            if solution.iter().all(|&x| x >= 0) {
                let total = solution.iter().sum::<i64>() as usize;
                *min_presses = (*min_presses).min(total);
            }
            return true;
        }

        // Pruning: limit by current minimum and per-variable bound
        let current_sum: i64 = current[..idx].iter().sum();
        let var_bound = free_var_bounds[idx];
        let upper = if *min_presses == usize::MAX {
            var_bound
        } else {
            (*min_presses as i64)
                .saturating_sub(current_sum)
                .min(var_bound)
        };

        for val in 0..=upper {
            current[idx] = val;

            // Early pruning
            if *min_presses != usize::MAX {
                let partial_sum: i64 = current[..=idx].iter().sum();
                if partial_sum >= *min_presses as i64 {
                    break;
                }
            }

            if !search(
                free_vars,
                free_var_bounds,
                idx + 1,
                current,
                matrix,
                basic_vars,
                n_buttons,
                min_presses,
                iteration_count,
                max_iterations,
            ) {
                return false; // Propagate timeout
            }
        }
        true
    }

    let completed = search(
        &free_vars,
        &free_var_bounds,
        0,
        &mut vec![0i64; n_free],
        &matrix,
        &basic_vars,
        n_buttons,
        &mut min_presses,
        &mut iteration_count,
        max_iterations,
    );

    if !completed {
        return usize::MAX;
    }

    min_presses
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
            let machines = parse_input(&input);

            match part.as_str() {
                "part1" => {
                    println!("=== Part 1: Indicator Lights ===");
                    let mut total_part1 = 0;
                    for machine in machines.iter() {
                        let min_presses = solve_machine(machine);
                        if min_presses != usize::MAX {
                            total_part1 += min_presses;
                        }
                    }
                    println!("Part 1 Total: {}", total_part1);
                }
                "part2" => {
                    println!("=== Part 2: Joltage Counters ===");

                    // Use multithreading for Part 2
                    let machines_arc = Arc::new(machines);
                    let results = Arc::new(Mutex::new(vec![0usize; machines_arc.len()]));

                    let num_threads = 8;
                    let mut handles = vec![];

                    for thread_id in 0..num_threads {
                        let machines_clone = Arc::clone(&machines_arc);
                        let results_clone = Arc::clone(&results);

                        let handle = thread::spawn(move || {
                            for i in (thread_id..machines_clone.len()).step_by(num_threads) {
                                let min_presses = solve_machine_part2(&machines_clone[i]);
                                let mut results = results_clone.lock().unwrap();
                                results[i] = min_presses;
                            }
                        });

                        handles.push(handle);
                    }

                    // Wait for all threads to complete
                    for handle in handles {
                        handle.join().unwrap();
                    }

                    let results = results.lock().unwrap();
                    let mut total_part2 = 0;
                    let mut unsolvable = Vec::new();

                    for (i, &min_presses) in results.iter().enumerate() {
                        if min_presses == usize::MAX {
                            unsolvable.push(i + 1);
                        } else {
                            total_part2 += min_presses;
                        }
                    }

                    if !unsolvable.is_empty() {
                        println!(
                            "Warning: {} machines unsolvable: {:?}",
                            unsolvable.len(),
                            unsolvable
                        );
                    }
                    println!("Part 2 Total: {}", total_part2);
                }
                _ => {
                    eprintln!("Invalid part: {}. Use 'part1' or 'part2'", part);
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            eprintln!("Error reading input.txt: {}", error);
        }
    }
}
