use std::cmp::Reverse;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn squared_distance_to(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::parse::<i64>);

        let x = parts.next().ok_or("missing x")?.map_err(|_| "invalid x")?;
        let y = parts.next().ok_or("missing y")?.map_err(|_| "invalid y")?;
        let z = parts.next().ok_or("missing z")?.map_err(|_| "invalid z")?;

        Ok(Self { x, y, z })
    }
}

/// Represents a distance between two positions, ordered by distance.
#[derive(Debug, Clone, Copy)]
struct Edge {
    distance: i64,
    p0: Position,
    p1: Position,
}

impl Edge {
    fn new(p0: Position, p1: Position) -> Self {
        Self {
            distance: p0.squared_distance_to(&p1),
            p0,
            p1,
        }
    }
}

/// Union-Find data structure for efficient cluster merging.
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]]; // Path compression
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
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

    fn cluster_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        // Normalize all parents first to avoid borrow issues
        for i in 0..n {
            self.find(i);
        }
        (0..n)
            .filter(|&i| self.parent[i] == i)
            .map(|i| self.size[i])
            .collect()
    }
}

fn parse_positions(data: &str) -> Vec<Position> {
    data.lines()
        .map(|line| line.parse().expect("valid position"))
        .collect()
}

fn compute_all_edges(positions: &[Position]) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(positions.len() * (positions.len() - 1) / 2);

    for (i, &p0) in positions.iter().enumerate() {
        for &p1 in &positions[i + 1..] {
            edges.push(Edge::new(p0, p1));
        }
    }

    edges.sort_unstable_by_key(|e| e.distance);
    edges
}

fn build_position_index(positions: &[Position]) -> HashMap<Position, usize> {
    positions
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect()
}

#[must_use]
pub fn solve_pt1(data: &str, connection_count: usize) -> usize {
    let positions = parse_positions(data);
    let edges = compute_all_edges(&positions);
    let pos_to_idx = build_position_index(&positions);

    let mut uf = UnionFind::new(positions.len());

    for edge in edges.iter().take(connection_count) {
        let idx0 = pos_to_idx[&edge.p0];
        let idx1 = pos_to_idx[&edge.p1];
        uf.union(idx0, idx1);
    }

    let mut sizes = uf.cluster_sizes();
    sizes.sort_unstable_by_key(|&s| Reverse(s));

    sizes.into_iter().take(3).product()
}

#[must_use]
pub fn solve_pt2(data: &str) -> i64 {
    let positions = parse_positions(data);
    let edges = compute_all_edges(&positions);
    let pos_to_idx = build_position_index(&positions);
    let total_positions = positions.len();

    let mut uf = UnionFind::new(total_positions);

    for edge in &edges {
        let idx0 = pos_to_idx[&edge.p0];
        let idx1 = pos_to_idx[&edge.p1];

        if uf.union(idx0, idx1) {
            let root = uf.find(idx0);
            if uf.size[root] == total_positions {
                return edge.p0.x * edge.p1.x;
            }
        }
    }

    panic!("no solution found")
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    const TEST_DATA: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_pt1() {
        let result = solve_pt1(TEST_DATA, 10);
        // With 10 connections on 20 points, we expect specific cluster sizes
        assert!(result == 40, "result should be 40");
    }

    #[test]
    fn test_pt2() {
        let result = solve_pt2(TEST_DATA);
        assert!(result == 25272, "result should be 25272");
    }
}
