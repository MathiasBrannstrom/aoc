// https://adventofcode.com/2016/day/2

pub fn solve_pt1(data: &str) {
    let mut pos = (1i32, 1i32);
    let keypad = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9']];

    let mut code = String::new();
    
    for line in data.lines() {
        for c in line.chars() {
            match c {
                'U' => pos.0 = (pos.0 - 1).max(0),
                'D' => pos.0 = (pos.0 + 1).min(2),
                'L' => pos.1 = (pos.1 - 1).max(0),
                'R' => pos.1 = (pos.1 + 1).min(2),
                _ => panic!("Invalid direction"),
            }
        }
        code.push(keypad[pos.0 as usize][pos.1 as usize]);
    }
    
    println!("Code: {}", code);
}



pub fn solve_pt2(data: &str) {
    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' ']];

    let (mut pos_x, mut pos_y) = (0i32, 2i32);
    let mut code = String::new();
    
    for line in data.lines() {
        for c in line.chars() {
            
            let new_pos_x = match c {  
                'L' => (pos_x - 1).max(0),
                'R' => (pos_x + 1).min(4),
                _ => pos_x,
            };
            let new_pos_y = match c {
                'U' => (pos_y - 1).max(0),
                'D' => (pos_y + 1).min(4),
                _ => pos_y,
            };

            if keypad[new_pos_y as usize][new_pos_x as usize] != ' ' {
                (pos_x, pos_y) = (new_pos_x, new_pos_y);
            }
        }
        code.push(keypad[pos_y as usize][pos_x as usize]);
    }
    
    println!("Code: {}", code);
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_02_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_02_input");
        solve_pt2(data);
    }

}
