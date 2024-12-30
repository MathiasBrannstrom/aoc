#[derive(Debug, Clone, Copy)]
enum Action{
    Toggle,
    On,
    Off
}


pub fn solve_pt2(data: &str) {

    let mut grid = vec![vec![0i32; 1000]; 1000];

    for line in data.lines() {
        let mod_line = line.replace("toggle", "tog gle");
        let split_line:Vec<&str> = mod_line.split(' ').collect();

        let action = match split_line[1] {
            "on" => Action::On,
            "off" => Action::Off,
            "gle" => Action::Toggle,
            _ => panic!()
        };

        let (start_x, start_y) = {
            let coords: Vec<usize> = 
                split_line[2]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();

            (coords[0], coords[1])
        };

        let (end_x, end_y) = {
            let coords: Vec<usize> = 
                split_line[4]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            
            (coords[0], coords[1])
        };

        for row in grid[start_y..=end_y].iter_mut() {
            for brightness in row[start_x..=end_x].iter_mut() {
                *brightness = match action {
                    Action::Toggle => *brightness + 2,
                    Action::On => *brightness + 1,
                    Action::Off => (*brightness - 1).max(0),
                };
            }
        }
    }

    let mut total_brightness = 0;
    for row in grid {
        for brightness in row {
            total_brightness += brightness;
        }
    }

    println!("{}", total_brightness);
}

pub fn solve(data: &str) {

    let mut grid = vec![vec![false; 1000]; 1000];

    for line in data.lines() {
        let mod_line = line.replace("toggle", "tog gle");
        let split_line:Vec<&str> = mod_line.split(' ').collect();

        let action = match split_line[1] {
            "on" => Action::On,
            "off" => Action::Off,
            "gle" => Action::Toggle,
            _ => panic!()
        };

        let (start_x, start_y) = {
            let coords: Vec<usize> = 
                split_line[2]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();

            (coords[0], coords[1])
        };

        let (end_x, end_y) = {
            let coords: Vec<usize> = 
                split_line[4]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            
            (coords[0], coords[1])
        };

        for row in grid[start_y..=end_y].iter_mut() {
            for lamp_state in row[start_x..=end_x].iter_mut() {
                *lamp_state = match action {
                    Action::Toggle => !*lamp_state,
                    Action::On => true,
                    Action::Off => false,
                };
            }
        }

    }

    let mut light_count = 0;
    for row in grid {
        for lamp_on in row {
            if lamp_on {
                light_count += 1;
            }
        }
    }
    println!("{}", light_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = include_str!("day_6_input");
        solve(data);
    }
    
    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_6_input");
        solve_pt2(data);
    }
    
}