use std::collections::HashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64
}

impl Position {
    pub fn dist_to(&self, other:Position) -> i64 {
        (self.x - other.x).pow(2) + 
        (self.y - other.y).pow(2) + 
        (self.z - other.z).pow(2)
    }
}

pub fn solve_pt1(data: &str, is_test: bool) {
    let positions:Vec<Position> = data.lines().map(|l| {
        let mut sp = l.split(',').map(|c| c.parse().unwrap());
        Position{
            x:sp.next().unwrap(), 
            y:sp.next().unwrap(), 
            z:sp.next().unwrap()}
    }).collect();

    let mut all_distances:Vec<(i64, Position, Position)> = vec![];

    for (i,p0) in positions[0..positions.len()-1].iter().copied().enumerate() {
        for p1 in positions[(i+1)..].iter().copied() {
            let dist = p0.dist_to(p1);
            all_distances.push((dist, p0, p1));
        }
    }

    all_distances.sort_unstable_by(|(d0, _, _), (d1, _, _)| { d0.cmp(d1) });

    let all_distances = all_distances;

    let mut circuits: Vec<Vec<Position>> = vec![vec![]; positions.len()];
    let mut position_to_circuit_map: HashMap<Position, usize> = HashMap::new();

    for (c_i,p) in positions.iter().copied().enumerate() {
        circuits[c_i].push(p);
        position_to_circuit_map.insert(p, c_i);
    }

    println!("{}",positions.len());
    println!("{}",all_distances.len());

    let connection_count = if is_test {10} else {1000};

    for (_, p0, p1) in all_distances[0..connection_count].iter().copied() {
        let c0 = position_to_circuit_map[&p0];
        let c1 = position_to_circuit_map[&p1];

        if c0 == c1 {
            continue;
        }

        // Move all positions from c1 to c0
        let mut positions_to_move = circuits[c1].clone();
        for p in positions_to_move.iter().copied() {
            position_to_circuit_map.insert(p, c0);
        }
        circuits[c1].clear();
        circuits[c0].append(&mut positions_to_move);
    }

    circuits.sort_by(|a,b| a.len().cmp(&b.len()).reverse());

    let result = circuits[0].len() * circuits[1].len() * circuits[2].len();

    println!("{}", result);
}

pub fn solve_pt2(data: &str) {
        let positions:Vec<Position> = data.lines().map(|l| {
        let mut sp = l.split(',').map(|c| c.parse().unwrap());
        Position{
            x:sp.next().unwrap(), 
            y:sp.next().unwrap(), 
            z:sp.next().unwrap()}
    }).collect();

    let mut all_distances:Vec<(i64, Position, Position)> = vec![];

    for (i,p0) in positions[0..positions.len()-1].iter().copied().enumerate() {
        for p1 in positions[(i+1)..].iter().copied() {
            let dist = p0.dist_to(p1);
            all_distances.push((dist, p0, p1));
        }
    }

    all_distances.sort_unstable_by(|(d0, _, _), (d1, _, _)| { d0.cmp(d1) });

    let all_distances = all_distances;

    let mut circuits: Vec<Vec<Position>> = vec![vec![]; positions.len()];
    let mut position_to_circuit_map: HashMap<Position, usize> = HashMap::new();

    for (c_i,p) in positions.iter().copied().enumerate() {
        circuits[c_i].push(p);
        position_to_circuit_map.insert(p, c_i);
    }

    println!("{}",positions.len());
    println!("{}",all_distances.len());

    for (_, p0, p1) in all_distances.iter().copied() {
        let c0 = position_to_circuit_map[&p0];
        let c1 = position_to_circuit_map[&p1];

        if c0 == c1 {
            continue;
        }
        
        // Move all positions from c1 to c0
        let mut positions_to_move = circuits[c1].clone();
        for p in positions_to_move.iter().copied() {
            position_to_circuit_map.insert(p, c0);
        }
        circuits[c1].clear();
        circuits[c0].append(&mut positions_to_move);

        if circuits[c0].len() == positions.len() {
            println!("Last connection  {}", p0.x * p1.x);
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    pub fn test_pt1(){
        let data = r"162,817,812
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
        solve_pt1(data, true);
    }

    #[test]
    pub fn test_pt2(){
        let data = r"162,817,812
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
        solve_pt2(data);
    }
}