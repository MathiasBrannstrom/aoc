// https://adventofcode.com/2015/day/20

pub fn solve_pt1(data: &str) {
    let present_amount = data.trim().parse::<usize>().unwrap();

    let pow = ((present_amount/10) as f32).log2().ceil() as u32;

    let upper_bound = 2u32.pow(pow) as usize;
    let mut houses = vec![0; upper_bound];

    for i in 1..upper_bound {
        for j in (i..upper_bound).step_by(i) {
            houses[j] += i * 10;
        }
    }

    for (i, house) in houses.iter().enumerate() {
        if *house >= present_amount {
            println!("House: {}", i);
            break;
        }
    }
}

pub fn solve_pt2(data: &str) {
    
    let present_amount = data.trim().parse::<usize>().unwrap();
    let upper_bound = present_amount/10;
    
    let mut houses = vec![0; upper_bound];

    for i in 1..upper_bound {
        for j in 1..=50 {
            if i*j >= upper_bound {
                break;
            }
            houses[i*j] += i * 11;
        }
    }

    for (i, house) in houses.iter().enumerate() {
        if *house >= present_amount {
            println!("House: {}", i);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_20_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_20_input");
        solve_pt2(data);
    }

}
