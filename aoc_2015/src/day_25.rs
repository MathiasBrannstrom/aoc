// https://adventofcode.com/2015/day/25

pub fn solve_pt1(_: &str) {
    let row = 3010;
    let col = 3019;

    let mut code = 20151125u64;

    let iterations = code_number(row, col);

    for _ in 1..iterations {
        code = (code * 252533) % 33554393;
    }

    println!("Code: {}", code);
}

fn code_number(row: u64, col: u64) -> u64 {
    let mut code_number = 0u64;

    for i in 1..=col {
        code_number += i;
        println!("{}", code_number);
    }
    for i in 0..(row-1) {
        code_number += col + i;
        println!("{}", code_number);
    }

    code_number
}


pub fn solve_pt2(_data: &str) {
    println!("Merry Christmas!");
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_25_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_25_input");
        solve_pt2(data);
    }

}
