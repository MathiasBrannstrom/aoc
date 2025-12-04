pub fn solve_pt1(data: &str) {
    let rots = data.lines().map(|x| {
        if x.starts_with("L") {
            -x.split_at(1).1.parse::<i32>().unwrap()
        }
        else {
            x.split_at(1).1.parse::<i32>().unwrap()
        }
    });

    let mut dial = 50;
    let mut zero_count = 0;

    for rot in rots {
        dial = ((dial + rot) + 100) % 100;

        if dial == 0 {
            zero_count += 1;
        }
    }

    println!("{}", zero_count);
}

pub fn solve_pt2(data: &str) {
        let rots = data.lines().map(|x| {
        if x.starts_with("L") {
            -x.split_at(1).1.parse::<i32>().unwrap()
        }
        else {
            x.split_at(1).1.parse::<i32>().unwrap()
        }
    });

    let mut dial = 50;
    let mut zero_count = 0;

    for rot in rots {
        let rotations_across_zero = if rot < 0 {
            if dial == 0 {
                rot.abs() / 100
            } else {
                (rot.abs() + (100-dial)) / 100 
            }
        } else {
            (rot + dial) / 100
        };
        dial = ((dial + rot) + 100000) % 100;

        zero_count += rotations_across_zero;
    }

    println!("{}", zero_count);
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};


    #[test]
    fn pt1_example_1() {
        let data = 
r"L50
L50";
        solve_pt2(data);
    }
}