use std::collections::{HashMap, HashSet};

pub fn solve_pt1(data: &str) {
    let grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let source_row = grid
        .iter()
        .enumerate()
        .find(|r| r.1.contains(&'S'))
        .unwrap();

    let source_pos = (
        source_row.0,
        source_row.1.iter().position(|c| *c == 'S').unwrap(),
    );

    println!("{:?}", source_pos);

    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let mut split_count = 0;

    let mut tachyons: HashSet<(usize, usize)> = HashSet::new();

    tachyons.insert(source_pos);
    while !tachyons.is_empty() {
        let mut next_tachyons: HashSet<(usize, usize)> = HashSet::new();
        for t in tachyons.iter() {
            if t.0 + 1 == grid_height {
                break;
            }
            let char_below = grid[t.0 + 1][t.1];

            if char_below == '.' {
                next_tachyons.insert((t.0 + 1, t.1));
            } else {
                if t.1 > 0 {
                    next_tachyons.insert((t.0 + 1, t.1 - 1));
                }
                if t.1 + 1 < grid_width {
                    next_tachyons.insert((t.0 + 1, t.1 + 1));
                }
                split_count += 1;
            }
        }

        tachyons = next_tachyons;
    }

    println!("{}", split_count);
}

pub fn solve_pt2(data: &str) {
    let grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let source_row = grid
        .iter()
        .enumerate()
        .find(|r| r.1.contains(&'S'))
        .unwrap();

    let source_pos = (
        source_row.0,
        source_row.1.iter().position(|c| *c == 'S').unwrap(),
    );

    println!("{:?}", source_pos);

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let mut cache = HashMap::new();

    let result = how_many_timelines(source_pos, &grid, &mut cache);

    println!("{}", result);
}

pub fn how_many_timelines(
    source: (usize, usize),
    grid: &Vec<Vec<char>>,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if cache.contains_key(&source) {
        return *cache.get(&source).unwrap();
    }
    let mut current_pos = source;
    while true {
        current_pos = (current_pos.0 + 1, current_pos.1);
        if current_pos.0 >= grid.len() {
            cache.insert(source, 1);
            return 1;
        }
        if grid[current_pos.0][current_pos.1] == '^' {
            break;
        }
    }

    let mut sum = 0;

    if current_pos.1 > 0 {
        sum += how_many_timelines((current_pos.0, current_pos.1 - 1), grid, cache);
    }
    if current_pos.1 + 1 < grid[0].len() {
        sum += how_many_timelines((current_pos.0, current_pos.1 + 1), grid, cache);
    }

    cache.insert(source, sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    pub fn test_pt1() {
        let data = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        solve_pt1(data);
    }

    #[test]
    pub fn test_pt2() {
        let data = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        solve_pt2(data);
    }
}
