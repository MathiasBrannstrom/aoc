pub fn solve_pt1(data: &str) {
    let grid: Vec<Vec<bool>> = data.lines().map(|r| {
        r.chars().map(|c|
        {
            match c {
                '@' => true,
                _ => false
            }
        }).collect()
    }
    ).collect();

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let mut accessible_rolls = 0;

    for x in 0..grid_width{
        for y in 0..grid_height {
            if grid[y][x] && get_neighbors(&grid, x as isize, y as isize) < 4 {
                accessible_rolls += 1;
            }        
        }
    }

    println!("{}", accessible_rolls);
}

fn get_neighbors(grid:&Vec<Vec<bool>>, x:isize, y:isize) -> u64 {
    let mut count = 0;
    
    for b in y-1..(y+2) {
        if b < 0 || b >= grid.len() as isize {
            continue;
        }
        for a in x-1..(x+2) {
            if a < 0 || a >= grid[0].len() as isize {
                continue;
            }

  
            if a == x && b == y {
                continue;
            }
            if grid[b as usize][a as usize] {
                count += 1;
            }
        }
    }
    // println!("{}", count);
    count

}

pub fn solve_pt2(data: &str) {
        let mut grid: Vec<Vec<bool>> = data.lines().map(|r| {
        r.chars().map(|c|
        {
            match c {
                '@' => true,
                _ => false
            }
        }).collect()
    }
    ).collect();

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let mut accessible_rolls = vec![];
    let mut total_rolls_removed = 0;
    while true {
        for x in 0..grid_width{
            for y in 0..grid_height {
                if grid[y][x] && get_neighbors(&grid, x as isize, y as isize) < 4 {
                    accessible_rolls.push((x,y));
                }        
            }
        }

        if accessible_rolls.is_empty() {
            break;
        }

        for pos in accessible_rolls.iter() {
            grid[pos.1][pos.0] = false;
        }
        total_rolls_removed += accessible_rolls.len();
        accessible_rolls.clear();
    }
    

    println!("{}", total_rolls_removed);
}


#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() {
        let data = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        solve_pt1(data);
    }

      #[test]
    fn test_pt2() {
        let data = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        solve_pt2(data);
    }
}