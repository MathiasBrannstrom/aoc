// https://adventofcode.com/2016/day/1

use std::collections::HashSet;

pub fn solve_pt1(data: &str) {
    let instructions = data.split(", ");
    let mut direction = (0, 1);
    let mut position = (0, 0);

    for instruction in instructions {
        let (turn, distance) = instruction.split_at(1);
        let distance = distance.trim().parse::<i32>().expect(distance);

        direction = match turn {
            "L" => (-direction.1, direction.0),
            "R" => (direction.1, -direction.0),
            _ => panic!("Invalid turn"),
        };
        position.0 += direction.0 * distance;
        position.1 += direction.1 * distance;
    }

    println!("Distance from origo: {:?}", position.0.abs() + position.1.abs());
}

pub fn solve_pt2(data: &str) {
    let instructions = data.split(", ");
    let mut direction = (0, 1);
    let mut position = (0, 0);
    let mut visited:HashSet<(i32, i32)> = HashSet::new();

    for instruction in instructions {
        let (turn, distance) = instruction.split_at(1);
        let distance = distance.trim().parse::<i32>().expect(distance);

        direction = match turn {
            "L" => (-direction.1, direction.0),
            "R" => (direction.1, -direction.0),
            _ => panic!("Invalid turn"),
        };
        let mut found_pos = false;
        for _ in 0..distance {
            position.0 += direction.0;
            position.1 += direction.1;
            
            if visited.contains(&position) {
                found_pos = true;
                break;
            }
            visited.insert(position);
        }
        if found_pos {
            break;
        }
    }
    println!("{:?}", position);
    println!("Distance from origo: {:?}", position.0.abs() + position.1.abs());
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_01_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_01_input");
        solve_pt2(data);
    }

}
