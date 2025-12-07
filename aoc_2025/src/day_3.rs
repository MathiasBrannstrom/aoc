pub fn solve_pt1(data: &str) {
    let jolts: u32 = data
        .lines()
        .map(|bank_str| {
            let batteries: Vec<u32> = bank_str
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();

            let mut max_1 = 0;
            let mut max_2 = 0;
            let b_count = batteries.len();

            for (i, b) in batteries.into_iter().enumerate() {
                if b > max_1 && i < (b_count - 1) {
                    max_1 = b;
                    max_2 = 0;
                    continue;
                }

                if b > max_2 {
                    max_2 = b;
                }
            }

            // println!("{}", max_1);
            // println!("{}", max_2);
            max_1 * 10 + max_2
        })
        .sum();

    println!("{}", jolts);
}

pub fn solve_pt2(data: &str) {
    let jolts: u64 = data
        .lines()
        .map(|bank_str| {
            let batteries: Vec<u32> = bank_str
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();

            find_max_joltage(&batteries, 12) / 10
        })
        .sum();

    println!("{}", jolts);
}

fn find_max_joltage(bank: &[u32], n_turn_on: u32) -> u64 {
    if n_turn_on == 0 {
        return 0;
    }

    let mut max = 0;
    let mut best_index = 1;

    for i in 0..(bank.len() as u32 - n_turn_on + 1) as usize {
        if bank[i] > max {
            max = bank[i];
            best_index = i;
        }
    }

    bank[best_index] as u64 * 10u64.pow(n_turn_on)
        + find_max_joltage(&bank[(best_index + 1)..], n_turn_on - 1)
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        solve_pt1(
            r"987654321111111
811111111111119
234234234234278
818181911112111",
        );
    }

    #[test]
    fn test_solve_pt2() {
        solve_pt2(
            r"987654321111111
811111111111119
234234234234278
818181911112111",
        );
    }
}
