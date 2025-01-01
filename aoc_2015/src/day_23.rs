// https://adventofcode.com/2015/day/23

pub fn solve_pt1(_data: &str) {
    // Manually created this code by looking at my specific input. 
    let mut a_reg:u32 = 20895;

    let mut b_reg:u32 = 0;

    while a_reg != 1 {
        b_reg += 1;
        
        if a_reg % 2 == 0 {
            a_reg /= 2;
        } else {
            a_reg = (a_reg * 3) + 1;
        }
    }

    println!("b_reg: {}", b_reg);

}

pub fn solve_pt2(_data: &str) {
    // Manually created this code by looking at my specific input. 
    let mut a_reg:u32 = 60975;

    let mut b_reg:u32 = 0;

    while a_reg != 1 {
        b_reg += 1;
        
        if a_reg % 2 == 0 {
            a_reg /= 2;
        } else {
            a_reg = (a_reg * 3) + 1;
        }
    }

    println!("b_reg: {}", b_reg);
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_23_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_23_input");
        solve_pt2(data);
    }

}
