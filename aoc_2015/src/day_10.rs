
pub fn solve(data: &str) {
    let input: Vec<u32> = data.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut sequence = input.clone();
    
    for _ in 0..40 {
        sequence = apply_look_and_say(&sequence);
    }

    println!("{:?}", sequence.len());
}


pub fn solve_pt2(data: &str) {
    let input: Vec<u32> = data.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut sequence = input.clone();
    
    for _ in 0..50 {
        sequence = apply_look_and_say(&sequence);
    }

    println!("{:?}", sequence.len());
}

fn apply_look_and_say(sequence: &[u32]) -> Vec<u32> {
    let mut new_sequence = vec![];

    let mut curr_num = sequence[0];
    let mut amount_of_num = 1;
    for i in sequence.iter().copied().skip(1) {
        if i == curr_num {
            amount_of_num += 1;
            continue;
        }

        new_sequence.push(amount_of_num);
        new_sequence.push(curr_num);
        curr_num = i;
        amount_of_num = 1;
    }

    new_sequence.push(amount_of_num);
    new_sequence.push(curr_num);

    new_sequence
}



#[cfg(test)]
mod tests{
    use super::{solve, solve_pt2};

    #[test]
    fn test_solve(){
        let data = include_str!("day_10_input");
        solve(data);
    }

    #[test]
    fn test_solve_pt2(){
        let data = include_str!("day_10_input");
        solve_pt2(data);
    }
}
