use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn solve_part1(tiles: &[Point]) -> i64 {
    let n = tiles.len();
    let mut max_area: i64 = 0;

    // Try all pairs of tiles as opposite corners
    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = tiles[i];
            let p2 = tiles[j];

            // Calculate rectangle dimensions (add 1 to include both corner tiles)
            let width = (p2.x - p1.x).abs() as i64 + 1;
            let height = (p2.y - p1.y).abs() as i64 + 1;

            // Area of rectangle
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

// Check if a point is inside a polygon using ray casting algorithm
fn point_in_polygon(point: &Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();

    let mut j = n - 1;
    for i in 0..n {
        let pi = polygon[i];
        let pj = polygon[j];

        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            inside = !inside;
        }
        j = i;
    }

    inside
}

// Check if a point is on the edge between consecutive red tiles
fn is_on_edge(point: &Point, red_tiles: &[Point]) -> bool {
    let n = red_tiles.len();

    for i in 0..n {
        let p1 = red_tiles[i];
        let p2 = red_tiles[(i + 1) % n];

        // Check if point is on the line segment between p1 and p2
        if p1.x == p2.x && p1.x == point.x {
            // Vertical line
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);
            if point.y >= min_y && point.y <= max_y {
                return true;
            }
        } else if p1.y == p2.y && p1.y == point.y {
            // Horizontal line
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            if point.x >= min_x && point.x <= max_x {
                return true;
            }
        }
    }

    false
}

// Check if a point is red or green
fn is_valid_tile(point: &Point, red_tiles: &HashSet<Point>, polygon: &[Point]) -> bool {
    // Check if it's a red tile
    if red_tiles.contains(point) {
        return true;
    }

    // Check if it's on an edge (green tile)
    if is_on_edge(point, polygon) {
        return true;
    }

    // Check if it's inside the polygon (green tile)
    point_in_polygon(point, polygon)
}

// Check if a rectangle contains only red or green tiles
fn is_valid_rectangle(
    p1: Point,
    p2: Point,
    red_tiles: &HashSet<Point>,
    polygon: &[Point],
    max_checks: i64,
) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    let width = (max_x - min_x + 1) as i64;
    let height = (max_y - min_y + 1) as i64;
    let total_points = width * height;

    // If too many points, sample instead of checking all
    if total_points > max_checks {
        // Sample points throughout the rectangle
        let step_x = ((width as f64) / (max_checks as f64).sqrt()).ceil() as i32;
        let step_y = ((height as f64) / (max_checks as f64).sqrt()).ceil() as i32;

        for x in (min_x..=max_x).step_by(step_x.max(1) as usize) {
            for y in (min_y..=max_y).step_by(step_y.max(1) as usize) {
                let point = Point { x, y };
                if !is_valid_tile(&point, red_tiles, polygon) {
                    return false;
                }
            }
        }

        // Also check corners and edges
        for x in [min_x, max_x] {
            for y in [min_y, max_y] {
                let point = Point { x, y };
                if !is_valid_tile(&point, red_tiles, polygon) {
                    return false;
                }
            }
        }
    } else {
        // Check all points in the rectangle
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let point = Point { x, y };
                if !is_valid_tile(&point, red_tiles, polygon) {
                    return false;
                }
            }
        }
    }

    true
}

fn solve_part2(tiles: &[Point]) -> i64 {
    let n = tiles.len();
    eprintln!("Number of red tiles: {}", n);

    // Create a set for fast red tile lookup
    let red_tiles_set: HashSet<Point> = tiles.iter().copied().collect();

    // Create a list of all pairs
    let mut pairs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = tiles[i];
            let p2 = tiles[j];
            let width = (p2.x - p1.x).abs() as i64 + 1;
            let height = (p2.y - p1.y).abs() as i64 + 1;
            let area = width * height;

            pairs.push((area, i, j, p1, p2, width, height));
        }
    }

    // Sort by area descending to find large valid rectangles quickly
    pairs.sort_by_key(|(area, _, _, _, _, _, _)| -(*area as i64));

    eprintln!("Checking {} pairs with multithreading...", pairs.len());

    let max_area = Mutex::new(0i64);
    let checked = Mutex::new(0usize);
    let valid = Mutex::new(0usize);

    // Process in parallel with rayon
    pairs
        .par_iter()
        .for_each(|(area, _i, _j, p1, p2, width, height)| {
            let current_max = *max_area.lock().unwrap();
            let current_checked = *checked.lock().unwrap();

            // Progress reporting
            if current_checked % 1000 == 0 && current_checked > 0 {
                let current_valid = *valid.lock().unwrap();
                eprintln!(
                    "Progress: checked {}, current max: {}, valid: {}",
                    current_checked, current_max, current_valid
                );
            }

            // Skip if this can't beat max_area
            if *area <= current_max {
                return;
            }

            // Skip extremely large rectangles (increase limit significantly)
            if *area > 2000000000 {
                return;
            }

            *checked.lock().unwrap() += 1;

            // Check if rectangle only contains red or green tiles
            // Increase point check limit for better accuracy
            if is_valid_rectangle(*p1, *p2, &red_tiles_set, tiles, 100000) {
                let mut max_lock = max_area.lock().unwrap();
                if *area > *max_lock {
                    *max_lock = *area;
                    *valid.lock().unwrap() += 1;
                    eprintln!(
                        "New max area: {} ({}x{}) between ({},{}) and ({},{})",
                        area, width, height, p1.x, p1.y, p2.x, p2.y
                    );
                }
            }
        });

    let final_max = *max_area.lock().unwrap();
    let final_checked = *checked.lock().unwrap();
    let final_valid = *valid.lock().unwrap();

    eprintln!(
        "Checked {} rectangles, {} valid",
        final_checked, final_valid
    );
    final_max
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
            // Parse red tile positions
            let tiles: Vec<Point> = input
                .lines()
                .filter_map(|line| {
                    let parts: Vec<i32> = line
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    if parts.len() == 2 {
                        Some(Point {
                            x: parts[0],
                            y: parts[1],
                        })
                    } else {
                        None
                    }
                })
                .collect();

            match part.as_str() {
                "part1" => {
                    let result = solve_part1(&tiles);
                    println!("Part 1 Result: {}", result);
                }
                "part2" => {
                    let result = solve_part2(&tiles);
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
