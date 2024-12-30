
pub fn solve_pt2(data: &str) {

    let mut sum = 0;

    for line in data.lines() {
        let raw_length = line.len();

        let encoded_length:usize = line.chars()
        .map(|c| match c {
            '\\' => 2,
            '\"' => 2,
            _ => 1
        })
        .sum();
        
        sum += 2 + encoded_length - raw_length;
    }
    
    println!("{}", sum);

}

pub fn solve(data: &str) {
    
    let mut sum = 0;
    for line in data.lines() {
        let raw_length = line.len();

        let mut parsed_length = 0;
        let mut chars_iter = line.chars().skip(1);
         
        while let Some(char) = chars_iter.next() {

            parsed_length += 1;

            if char != '\\' {    
                continue;
            }

            let next_char = chars_iter.next().unwrap();

            let chars_to_skip = match next_char {
                '\\' => 0,
                '\"' => 0,
                'x' => 2,
                _ => {
                    println!("{}", next_char);
                    panic!();
                }
            };

            for _ in 0..chars_to_skip {
                chars_iter.next();
            }
        }

        // Compensate for the last " in the string.
        parsed_length -= 1; 

        sum += raw_length - parsed_length;
    }
    
    println!("{}", sum);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = include_str!("day_8_input");
        solve(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_8_input");
        solve_pt2(data);
    }
    
}