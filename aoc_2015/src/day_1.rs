
pub fn solve(data: &str) {
    let floor = data.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });
    println!("Floor: {}", floor);
}

pub fn solve_pt2(data: &str) {
    let mut floor = 0;
    for (i, c) in data.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 {
            println!("Basement at: {}", i + 1);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = include_str!("day_1_input");
        solve(data);
    }
    
    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_1_input");
        solve_pt2(data);
    }
    
}