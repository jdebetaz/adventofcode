use std::collections::HashMap;
use std::fs;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let device = parts[0].to_string();
            let outputs: Vec<String> = parts[1].split_whitespace().map(|s| s.to_string()).collect();
            graph.insert(device, outputs);
        }
    }

    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut Vec<String>,
) -> usize {
    // If we've reached the target, we found one path
    if current == target {
        return 1;
    }

    // Check if we're in a cycle
    if visited.contains(&current.to_string()) {
        return 0;
    }

    // Mark current node as visited
    visited.push(current.to_string());

    let mut total_paths = 0;

    // Explore all neighbors
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths(graph, neighbor, target, visited);
        }
    }

    // Backtrack: remove current node from visited
    visited.pop();

    total_paths
}

// State for memoization: (current_node, visited_dac, visited_fft)
type MemoKey = (String, bool, bool);

fn count_paths_with_required_memoized(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited_dac: bool,
    visited_fft: bool,
    visited: &mut Vec<String>,
    memo: &mut HashMap<MemoKey, usize>,
) -> usize {
    // If we've reached the target, check if we've visited all required nodes
    if current == target {
        return if visited_dac && visited_fft { 1 } else { 0 };
    }

    // Check if we're in a cycle
    if visited.contains(&current.to_string()) {
        return 0;
    }

    // Check memoization
    let key = (current.to_string(), visited_dac, visited_fft);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    // Mark current node as visited
    visited.push(current.to_string());

    // Update visited flags
    let new_visited_dac = visited_dac || current == "dac";
    let new_visited_fft = visited_fft || current == "fft";

    let mut total_paths = 0;

    // Explore all neighbors
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths_with_required_memoized(
                graph,
                neighbor,
                target,
                new_visited_dac,
                new_visited_fft,
                visited,
                memo,
            );
        }
    }

    // Backtrack: remove current node from visited
    visited.pop();

    // Store in memo (only if not in current path to avoid caching cycle-dependent results)
    memo.insert(key, total_paths);

    total_paths
}

fn solve_part1(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = Vec::new();
    count_paths(graph, "you", "out", &mut visited)
}

fn solve_part2(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = Vec::new();
    let mut memo = HashMap::new();
    count_paths_with_required_memoized(graph, "svr", "out", false, false, &mut visited, &mut memo)
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
            let graph = parse_input(&input);

            match part.as_str() {
                "part1" => {
                    let result = solve_part1(&graph);
                    println!("Part 1 Result: {}", result);
                }
                "part2" => {
                    let result = solve_part2(&graph);
                    println!("Part 2 Result: {}", result);
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
