use std::collections::HashSet;
use std::fs;

type Shape = Vec<(i32, i32)>;

fn parse_shape(lines: &[&str]) -> Shape {
    let mut shape = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                shape.push((row as i32, col as i32));
            }
        }
    }
    normalize_shape(&shape)
}

fn normalize_shape(shape: &Shape) -> Shape {
    if shape.is_empty() {
        return shape.clone();
    }
    let min_row = shape.iter().map(|(r, _)| *r).min().unwrap();
    let min_col = shape.iter().map(|(_, c)| *c).min().unwrap();
    let mut normalized: Shape = shape
        .iter()
        .map(|(r, c)| (r - min_row, c - min_col))
        .collect();
    normalized.sort();
    normalized
}

fn rotate_90(shape: &Shape) -> Shape {
    // (r, c) -> (c, -r)
    let rotated: Shape = shape.iter().map(|(r, c)| (*c, -r)).collect();
    normalize_shape(&rotated)
}

fn flip_horizontal(shape: &Shape) -> Shape {
    // (r, c) -> (r, -c)
    let flipped: Shape = shape.iter().map(|(r, c)| (*r, -c)).collect();
    normalize_shape(&flipped)
}

fn get_all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations = HashSet::new();
    let mut current = shape.clone();

    for _ in 0..4 {
        orientations.insert(current.clone());
        orientations.insert(flip_horizontal(&current));
        current = rotate_90(&current);
    }

    orientations.into_iter().collect()
}

fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<(usize, usize, Vec<usize>)>) {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    let mut shapes: std::collections::HashMap<usize, Shape> = std::collections::HashMap::new();

    // Parse shapes section
    for part in &parts {
        let lines: Vec<&str> = part.lines().collect();
        if lines.is_empty() {
            continue;
        }

        // Check if it's a shape definition (starts with "digit:")
        if let Some(first_line) = lines.first() {
            if let Some(colon_pos) = first_line.find(':') {
                let potential_idx = &first_line[..colon_pos];
                if let Ok(idx) = potential_idx.trim().parse::<usize>() {
                    // This is a shape definition
                    let shape_lines: Vec<&str> = if first_line.len() > colon_pos + 1
                        && !first_line[colon_pos + 1..].trim().is_empty()
                    {
                        // Shape starts on same line
                        let mut sl = vec![first_line[colon_pos + 1..].trim()];
                        sl.extend(lines[1..].iter().copied());
                        sl
                    } else {
                        lines[1..].to_vec()
                    };
                    let shape = parse_shape(&shape_lines);
                    shapes.insert(idx, shape);
                }
            }
        }
    }

    // Convert shapes to vector with all orientations
    let max_shape_idx = shapes.keys().max().copied().unwrap_or(0);
    let mut all_shapes: Vec<Vec<Shape>> = Vec::new();
    for i in 0..=max_shape_idx {
        if let Some(shape) = shapes.get(&i) {
            all_shapes.push(get_all_orientations(shape));
        } else {
            all_shapes.push(vec![]);
        }
    }

    // Parse regions
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

    for part in &parts {
        let lines: Vec<&str> = part.lines().collect();
        for line in lines {
            // Check if it's a region definition (contains "x" and ":")
            if let Some(x_pos) = line.find('x') {
                if let Some(colon_pos) = line.find(':') {
                    if x_pos < colon_pos {
                        let width: usize = line[..x_pos].trim().parse().unwrap_or(0);
                        let height: usize = line[x_pos + 1..colon_pos].trim().parse().unwrap_or(0);
                        let counts: Vec<usize> = line[colon_pos + 1..]
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        if width > 0 && height > 0 {
                            regions.push((width, height, counts));
                        }
                    }
                }
            }
        }
    }

    (all_shapes, regions)
}

fn can_place(
    grid: &[Vec<bool>],
    shape: &Shape,
    start_row: i32,
    start_col: i32,
    width: usize,
    height: usize,
) -> bool {
    for (dr, dc) in shape {
        let r = start_row + dr;
        let c = start_col + dc;
        if r < 0 || c < 0 || r >= height as i32 || c >= width as i32 {
            return false;
        }
        if grid[r as usize][c as usize] {
            return false;
        }
    }
    true
}

fn place_shape(grid: &mut [Vec<bool>], shape: &Shape, start_row: i32, start_col: i32) {
    for (dr, dc) in shape {
        let r = (start_row + dr) as usize;
        let c = (start_col + dc) as usize;
        grid[r][c] = true;
    }
}

fn remove_shape(grid: &mut [Vec<bool>], shape: &Shape, start_row: i32, start_col: i32) {
    for (dr, dc) in shape {
        let r = (start_row + dr) as usize;
        let c = (start_col + dc) as usize;
        grid[r][c] = false;
    }
}

// Precompute all valid placements for each shape and orientation
struct Placements {
    // For each shape index, list of (orientation_ref, row, col)
    data: Vec<Vec<(usize, i32, i32)>>,
}

fn precompute_placements(all_shapes: &[Vec<Shape>], width: usize, height: usize) -> Placements {
    let mut data = Vec::new();

    for shape_orientations in all_shapes {
        let mut shape_placements = Vec::new();
        for (orient_idx, orientation) in shape_orientations.iter().enumerate() {
            let max_row = orientation.iter().map(|(r, _)| *r).max().unwrap_or(0);
            let max_col = orientation.iter().map(|(_, c)| *c).max().unwrap_or(0);

            for r in 0..=(height as i32 - max_row - 1) {
                for c in 0..=(width as i32 - max_col - 1) {
                    shape_placements.push((orient_idx, r, c));
                }
            }
        }
        data.push(shape_placements);
    }

    Placements { data }
}

fn solve(
    grid: &mut Vec<Vec<bool>>,
    width: usize,
    height: usize,
    pieces: &mut Vec<usize>,
    piece_idx: usize,
    all_shapes: &[Vec<Shape>],
    placements: &Placements,
    last_placement: &mut Vec<usize>,
) -> bool {
    if piece_idx >= pieces.len() {
        return true;
    }

    let shape_idx = pieces[piece_idx];

    // If this piece is the same type as the previous one, start from the last placement index
    // to avoid duplicate orderings
    let start_idx = if piece_idx > 0 && pieces[piece_idx - 1] == shape_idx {
        last_placement[piece_idx - 1]
    } else {
        0
    };

    let shape_placements = &placements.data[shape_idx];
    let orientations = &all_shapes[shape_idx];

    for (placement_idx, &(orient_idx, r, c)) in shape_placements.iter().enumerate().skip(start_idx)
    {
        let orientation = &orientations[orient_idx];
        if can_place(grid, orientation, r, c, width, height) {
            place_shape(grid, orientation, r, c);
            last_placement[piece_idx] = placement_idx;

            if solve(
                grid,
                width,
                height,
                pieces,
                piece_idx + 1,
                all_shapes,
                placements,
                last_placement,
            ) {
                return true;
            }

            remove_shape(grid, orientation, r, c);
        }
    }

    false
}

fn can_fit_region(
    width: usize,
    height: usize,
    counts: &[usize],
    all_shapes: &[Vec<Shape>],
) -> bool {
    let mut grid = vec![vec![false; width]; height];

    // Build list of shape indices to place (one entry per piece)
    let mut pieces: Vec<usize> = Vec::new();
    for (shape_idx, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            pieces.push(shape_idx);
        }
    }

    // Sort by shape size (larger first) for better pruning
    pieces.sort_by(|&a, &b| {
        let size_a = all_shapes[a].first().map(|s| s.len()).unwrap_or(0);
        let size_b = all_shapes[b].first().map(|s| s.len()).unwrap_or(0);
        size_b.cmp(&size_a)
    });

    // Quick area check
    let total_area: usize = pieces
        .iter()
        .map(|&idx| all_shapes[idx].first().map(|s| s.len()).unwrap_or(0))
        .sum();

    if total_area > width * height {
        return false;
    }

    if pieces.is_empty() {
        return true;
    }

    let placements = precompute_placements(all_shapes, width, height);
    let mut last_placement = vec![0; pieces.len()];

    solve(
        &mut grid,
        width,
        height,
        &mut pieces,
        0,
        all_shapes,
        &placements,
        &mut last_placement,
    )
}

fn solve_part1(all_shapes: &[Vec<Shape>], regions: &[(usize, usize, Vec<usize>)]) -> usize {
    let mut count = 0;
    for (width, height, counts) in regions {
        if can_fit_region(*width, *height, counts, all_shapes) {
            count += 1;
        }
    }
    count
}

fn solve_part2(_all_shapes: &[Vec<Shape>], _regions: &[(usize, usize, Vec<usize>)]) -> String {
    // Part 2 is awarded for completing Part 1 - puzzle complete!
    "Puzzle complete!".to_string()
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
            let (all_shapes, regions) = parse_input(&input);

            match part.as_str() {
                "part1" => {
                    let result = solve_part1(&all_shapes, &regions);
                    println!("{}", result);
                }
                "part2" => {
                    let message = solve_part2(&all_shapes, &regions);
                    println!("{}", message);
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
