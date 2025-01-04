// https://adventofcode.com/2016/day/3

use itertools::Itertools;

pub fn solve_pt1(data: &str) {
    let possible_triangle_count = data.lines()
    .map(|line| line
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>())
    .map(|mut v| {
        v.sort();
        v
    })
    .filter(|v| v[0] + v[1] > v[2])
    .count();
    
    println!("Possible triangles: {}", possible_triangle_count);
}

pub fn solve_pt2(data: &str) {
    let possible_triangle_count = data.lines()
    .chunks(3).into_iter()
    .map(|c| {
        let (l1, l2, l3) = c.collect_tuple().unwrap();
        let v = [l1, l2, l3]
        .iter()
        .map(|line| line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
        
        let mut possible_triangles = 0;
        for i in 0..3 {
            let mut sides = [v[0][i], v[1][i], v[2][i]];
            sides.sort();
            if sides[0] + sides[1] > sides[2] {
                possible_triangles += 1;
            }
        }

        possible_triangles
    })
    .sum::<u32>();

    println!("Possible triangles: {}", possible_triangle_count);
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_03_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_03_input");
        solve_pt2(data);
    }

}
