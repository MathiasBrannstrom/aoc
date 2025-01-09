// https://adventofcode.com/2024/day/14

#[derive(Debug, Clone)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

pub fn solve_pt1(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

    let mut robots = Vec::new();
    for line in data.lines() {
        let cap = re.captures(line).unwrap();
        let x = cap[1].parse()?;
        let y = cap[2].parse()?;
        let vx = cap[3].parse()?;
        let vy = cap[4].parse()?;
        robots.push(Robot { x, y, vx, vy });
    }

    let size_x = 101;
    let size_y = 103;
    let time = 100;

    let new_position_robots:Vec<Robot> = 
        robots
        .iter()
        .map(|robot| Robot {
            x: ((robot.x + robot.vx * time) + size_x*time) % size_x,
            y: ((robot.y + robot.vy * time) + size_y*time) % size_y,
            vx: robot.vx,
            vy: robot.vy,
        })
        .collect();
    
    // println!("{:?}", new_position_robots);

    let mut quadrant_0_count = 0;
    let mut quadrant_1_count = 0;
    let mut quadrant_2_count = 0;
    let mut quadrant_3_count = 0;

    for robot in new_position_robots.iter() {
        if robot.x < size_x / 2 && robot.y < size_y / 2 {
            quadrant_0_count += 1;
        } else if robot.x > size_x / 2 && robot.y < size_y / 2 {
            quadrant_1_count += 1;
        } else if robot.x < size_x / 2 && robot.y > size_y / 2 {
            quadrant_2_count += 1;
        } else if robot.x > size_x / 2 && robot.y > size_y / 2 {
            quadrant_3_count += 1;
        }
    }

    let safety_factor = quadrant_0_count * quadrant_1_count * quadrant_2_count * quadrant_3_count;

    println!("Quadrant 0: {}", quadrant_0_count);
    println!("Quadrant 1: {}", quadrant_1_count);
    println!("Quadrant 2: {}", quadrant_2_count);
    println!("Quadrant 3: {}", quadrant_3_count);

    println!("Safety factor: {}", safety_factor);
    Ok(())
}

fn check_for_tree(robots: &Vec<Robot>, grid: &mut Vec<Vec<char>>, time: i32) {
    
    for row in grid.iter_mut() {
        row.iter_mut().for_each(|cell| *cell = '.');
    }

    for robot in robots.iter() {
        grid[robot.y as usize][robot.x as usize] = '#';
    }

    
    let mut robots_with_3_or_more_neighbors = 0;
    for robot in robots.iter() {
        let mut neighbors = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x = robot.x + i;
                let y = robot.y + j;
                if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
                    continue;
                }
                if grid[y as usize][x as usize] == '#' {
                    neighbors += 1;
                }
            }
        }
        if neighbors >= 3 {
            robots_with_3_or_more_neighbors += 1;
        }
    }

    if robots_with_3_or_more_neighbors > robots.len() / 3 {
        println!("Time: {}", time);
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }
}

pub fn solve_pt2(data: &str) -> Result<(), Box<dyn std::error::Error>> {

    let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;

    let mut robots = Vec::new();
    for line in data.lines() {
        let cap = re.captures(line).unwrap();
        let x = cap[1].parse()?;
        let y = cap[2].parse()?;
        let vx = cap[3].parse()?;
        let vy = cap[4].parse()?;
        robots.push(Robot { x, y, vx, vy });
    }

    let size_x = 101;
    let size_y = 103;
    let mut grid = vec![vec!['.'; size_x as usize]; size_y as usize];
    let mut time = 0;
    loop {
        time += 1;
        for robot in robots.iter_mut() {
            robot.x = (robot.x + robot.vx + size_x) % size_x;
            robot.y = (robot.y + robot.vy + size_y) % size_y;
        }
        
        check_for_tree(&robots, &mut grid, time);

        if time == 10403 {
            break;
        }
    }


    Ok(())
}


#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_14_input");
        let result = solve_pt1(data);
        println!("{:?}", result);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_14_input");
        let result = solve_pt2(data);
        println!("{:?}", result);
    }

}
