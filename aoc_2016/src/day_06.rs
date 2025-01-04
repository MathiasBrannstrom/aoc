// https://adventofcode.com/2016/day/6

use std::collections::HashMap;

pub fn solve_pt1(data: &str) {
    let data = data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut char_count:Vec<HashMap<char, u32>> = (0..data[0].len()).map(|_| HashMap::new()).collect();

    for line in data {
        for (i, c) in line.iter().enumerate() {
            let count = char_count[i].entry(*c).or_insert(0);
            *count += 1;
        }
    }

    char_count.iter().map(|map| {
        let mut max = 0;
        let mut max_char = ' ';
        for (c, count) in map.iter() {
            if *count > max {
                max = *count;
                max_char = *c;
            }
        }
        max_char
    }).for_each(|c| print!("{}", c));
}

pub fn solve_pt2(data: &str) {
    let data = data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut char_count:Vec<HashMap<char, u32>> = (0..data[0].len()).map(|_| HashMap::new()).collect();

    for line in data {
        for (i, c) in line.iter().enumerate() {
            let count = char_count[i].entry(*c).or_insert(0);
            *count += 1;
        }
    }

    char_count.iter().map(|map| {
        let mut min = u32::MAX;
        let mut min_char = ' ';
        for (c, count) in map.iter() {
            if *count < min {
                min = *count;
                min_char = *c;
            }
        }
        min_char
    }).for_each(|c| print!("{}", c));
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_06_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_06_input");
        solve_pt2(data);
    }
}
