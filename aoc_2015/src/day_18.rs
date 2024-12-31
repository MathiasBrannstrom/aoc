// https://adventofcode.com/2015/day/18

pub fn solve_pt1(data: &str) {
    let mut grid = parse_grid(data);
    let mut second_grid = vec![vec![false; grid[0].len()]; grid.len()];
    for _ in 0..100 {
        perform_iteration(&grid, &mut second_grid);
        std::mem::swap(&mut grid, &mut second_grid);
    }

    let lights_on = grid.iter().flatten().copied().filter(|x| *x).count();

    println!("Lights on: {}", lights_on);
}

pub fn solve_pt2(data: &str) {
    let mut grid = parse_grid(data);
    let mut second_grid = vec![vec![false; grid[0].len()]; grid.len()];

    let (rows, cols) = (grid.len(), grid[0].len());
    for _ in 0..100 {
        perform_iteration(&grid, &mut second_grid);
        second_grid[0][0] = true;
        second_grid[0][cols-1] = true;
        second_grid[rows-1][0] = true;
        second_grid[rows-1][cols-1] = true;
        std::mem::swap(&mut grid, &mut second_grid);
    }

    let lights_on = grid.iter().flatten().copied().filter(|x| *x).count();

    println!("Lights on: {}", lights_on);
}

fn perform_iteration(grid: &[Vec<bool>], second_grid: &mut [Vec<bool>]) {

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let neighbors_on_amount = neighbors(grid, (i,j));
            
            second_grid[i][j] = matches!((neighbors_on_amount, grid[i][j]), (2, true) | (3, _));
        }
    }
}

fn neighbors(grid: &[Vec<bool>], pos: (usize,usize)) -> u32 {
    let mut neighbors = 0;
    let (i, j) = (pos.0 as isize, pos.1 as isize);
    for x in i-1..=i+1 {
        for y in j-1..=j+1 {
            if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[x as usize].len() as isize || (x == i && y == j) {
                continue;
            }
            if grid[x as usize][y as usize] {
                neighbors += 1;
            }
        }
    }
    neighbors
}

fn parse_grid(data: &str) -> Vec<Vec<bool>> {
    let mut grid = Vec::new();
    for line in data.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c == '#');
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_18_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_18_input");
        solve_pt2(data);
    }

}
