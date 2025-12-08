use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Union by size
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().copied().collect()
    }
}

fn solve_part1(input: &str, connections: usize) -> usize {
    // Parse junction boxes
    let points: Vec<Point> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<i32> = line
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if parts.len() == 3 {
                Some(Point {
                    x: parts[0],
                    y: parts[1],
                    z: parts[2],
                })
            } else {
                None
            }
        })
        .collect();

    let n = points.len();

    // Calculate all pairwise distances
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance_squared(&points[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort by distance
    edges.sort_by_key(|e| e.0);

    // Create union-find structure
    let mut uf = UnionFind::new(n);

    // Process the first 'connections' closest pairs (whether they connect or not)
    for idx in 0..connections.min(edges.len()) {
        let (_dist, i, j) = edges[idx];
        uf.union(i, j);
    }

    // Get circuit sizes
    let mut sizes = uf.get_component_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending

    // Multiply the three largest
    if sizes.len() >= 3 {
        sizes[0] * sizes[1] * sizes[2]
    } else {
        0
    }
}

fn solve_part2(input: &str) -> i64 {
    // Parse junction boxes
    let points: Vec<Point> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<i32> = line
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if parts.len() == 3 {
                Some(Point {
                    x: parts[0],
                    y: parts[1],
                    z: parts[2],
                })
            } else {
                None
            }
        })
        .collect();

    let n = points.len();
    eprintln!("Number of junction boxes: {}", n);

    // Calculate all pairwise distances
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance_squared(&points[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort by distance
    edges.sort_by_key(|e| e.0);

    // Create union-find structure
    let mut uf = UnionFind::new(n);

    // Keep connecting until everything is in one circuit
    let mut last_i = 0;
    let mut last_j = 0;

    for (_dist, i, j) in edges {
        if uf.union(i, j) {
            last_i = i;
            last_j = j;

            // Check if everything is connected
            let sizes = uf.get_component_sizes();
            if sizes.len() == 1 {
                eprintln!("All boxes connected!");
                eprintln!(
                    "Last connection: box {} ({},{},{}) and box {} ({},{},{})",
                    i,
                    points[i].x,
                    points[i].y,
                    points[i].z,
                    j,
                    points[j].x,
                    points[j].y,
                    points[j].z
                );
                break;
            }
        }
    }

    // Multiply X coordinates
    let result = points[last_i].x as i64 * points[last_j].x as i64;
    eprintln!(
        "Result: {} * {} = {}",
        points[last_i].x, points[last_j].x, result
    );
    result
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
                let result = solve_part1(&input, 1000);
                println!("Part 1 Result: {}", result);
            }
            "part2" => {
                let result = solve_part2(&input);
                println!("Part 2 Result: {}", result);
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
