pub fn solve_pt1(data: &str) {
    let input: Vec<&str> = data.lines().collect();

    let operations: Vec<&str> = input.last().unwrap().split_whitespace().collect();

    let numbers: Vec<Vec<u32>> = input[0..input.len() - 1]
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|x| {
                    println!("{}", x);
                    x.parse().unwrap()
                })
                .collect()
        })
        .collect();

    let mut total = 0;

    for c in 0..operations.len() {
        let mut x = numbers[0][c];
        let is_mult = operations[c] == "*";
        for r in 1..numbers.len() {
            if is_mult {
                x *= numbers[r][c];
            } else {
                x += numbers[r][c];
            }
        }

        total += x;
    }

    println!("{}", total)
}

pub fn solve_pt2(data: &str) {
    let input: Vec<&str> = data.lines().collect();

    let operations: Vec<&str> = input.last().unwrap().split_whitespace().collect();

    let num_grid: Vec<Vec<Option<u32>>> = input[0..input.len() - 1]
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    ' ' => None,
                    x => Some(x.to_digit(10).unwrap() as u32),
                })
                .collect()
        })
        .collect();

    let mut numbers: Vec<Option<u32>> = vec![];

    for r in 0..num_grid[0].len() {
        let mut sequence = vec![];
        for c in 0..num_grid.len() {
            sequence.push(num_grid[r][c]);
        }
        numbers.push(convert_sequence_to_number(&sequence));
    }

    numbers.push(None);

    let mut total = 0;
    let mut n_iter = numbers.iter();

    for op in operations {
        let mut x = n_iter.next().unwrap().unwrap();

        let mut next_num = *n_iter.next().unwrap();
        while next_num != None {
            x = apply_op(x, next_num.unwrap(), op);
            next_num = *n_iter.next().unwrap();
        }

        total += x;
    }

    println!("{}", total);
    // for n in numbers {
    //     println!("{:?}", n);
    // }
    // for op in operations {}
}

fn apply_op(num0: u32, num1: u32, op: &str) -> u32 {
    if op == "*" { num0 * num1 } else { num0 + num1 }
}

fn convert_sequence_to_number(sequence: &[Option<u32>]) -> Option<u32> {
    let mut num = 0;
    let mut any_val = false;

    for n in sequence.iter().flatten() {
        any_val = true;
        num = num * 10 + n;
    }

    if any_val { Some(num) } else { None }
}

#[cfg(test)]
mod tests {

    use super::{solve_pt1, solve_pt2};

    #[test]
    pub fn test_pt1() {
        let data = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

        solve_pt1(data);
    }

    #[test]
    pub fn test_pt2() {
        let data = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

        solve_pt2(data);
    }
}
