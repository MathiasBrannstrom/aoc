pub fn solve_pt1(data: &str) {
    let ranges = data.split(',').map(|x| {
        let mut range = x.split('-');
        (range.next().unwrap().parse::<u64>().unwrap(), range.next().unwrap().parse::<u64>().unwrap())
    });


    let mut invalid_count = 0;
    let mut invalid_sum = 0;
    for range in ranges {
        for n in range.0..(range.1+1) {
            let number_length = length_of_number(n);
            if  number_length % 2 != 0 {
                continue;
            }
            let top_part = n/10u64.pow(number_length/2);
            let bottom_part = n - top_part*10u64.pow(number_length/2);
            if top_part == bottom_part {
                invalid_count += 1;
                invalid_sum += n;
            }            
        }
    }

    println!("{}", invalid_count);
    println!("{}", invalid_sum);
}

fn length_of_number(n: u64) -> u32 {
    n.ilog10() + 1
}

pub fn solve_pt2(data: &str) {
    let ranges = data.split(',').map(|x| {
        let mut range = x.split('-');
        (range.next().unwrap().parse::<u64>().unwrap(), range.next().unwrap().parse::<u64>().unwrap())
    });


    let mut invalid_count = 0;
    let mut invalid_sum = 0;
    for range in ranges {
        for n in range.0..(range.1+1) {
            let number_length = length_of_number(n);

            for r in 2..(number_length+1) {
                if does_number_repeat_r_times(n, r) {
                    // println!("{}", n);
                    invalid_count += 1;
                    invalid_sum += n;
                    break;
                }
            }
        }
    }

    println!("{}", invalid_count);
    println!("{}", invalid_sum);
}

fn does_number_repeat_r_times(n: u64, r: u32) -> bool {
    let number_length = length_of_number(n);

    if  number_length % r != 0 {
        return false
    }

    let mut digits = Vec::new();

    let mut curr_n = n;

    for i in 0..number_length {
        let next_n = curr_n / 10;
        digits.push(curr_n - next_n*10);
        curr_n = next_n;
    }

    let pattern_length = number_length / r;

    for i in pattern_length..number_length {
        let d0 = digits[i as usize];
        let d1 = digits[(i-pattern_length) as usize];
        if d0 != d1 {
            return false;
        }
    }

    true
}


#[cfg(test)]
mod tests {
    use crate::day_2::does_number_repeat_r_times;

    use super::{solve_pt1, solve_pt2};


    #[test]
    fn pt1_test() {
        let data = 
r"11-22";
        solve_pt1(data);
    }

        #[test]
    fn pt2_test() {
        let data = 
r"11-22";
        solve_pt2(data);
    }


    #[test]
    fn repeat_test() {
        assert!(does_number_repeat_r_times(222222, 6));
        assert!(does_number_repeat_r_times(222222, 6));
        assert!(does_number_repeat_r_times(2121, 2));
        assert!(does_number_repeat_r_times(212121, 3));
        assert!(!does_number_repeat_r_times(212121, 2));
    }

    #[test]
    fn pt1_example_1() {
        let data = 
r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        solve_pt1(data);
    }


    #[test]
    fn pt2_example_1() {
        let data = 
r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        solve_pt2(data);
    }

}