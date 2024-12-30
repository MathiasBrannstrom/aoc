use std::collections::HashSet;


pub fn solve(data: &str) {

    let mut visited_positions:HashSet<(i32, i32)> = HashSet::new();
    let mut position = (0,0);
    visited_positions.insert(position);

    data.chars()
    .map(|c| match c {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => panic!()
    })
    .for_each(|dir| {
        position = (position.0 + dir.0, position.1 + dir.1);
        visited_positions.insert(position);    
    });

    println!("{}", visited_positions.len());
}

pub fn solve_pt2(data: &str) {

    let mut visited_positions:HashSet<(i32, i32)> = HashSet::new();
    let mut position = (0,0);
    let mut robo_position = (0,0);
    let mut robo_turn = false;
    visited_positions.insert(position);

    data.chars()
    .map(|c| match c {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => panic!()
    })
    .for_each(|dir| {
        if robo_turn {
            robo_position = (robo_position.0 + dir.0, robo_position.1 + dir.1);
            visited_positions.insert(robo_position);    
        }
        else {
            position = (position.0 + dir.0, position.1 + dir.1);
            visited_positions.insert(position);    
        }

        robo_turn = !robo_turn;
    });

    println!("{}", visited_positions.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = include_str!("day_3_input");
        solve(data);
    }
    
    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_3_input");
        solve_pt2(data);
    }
    
}