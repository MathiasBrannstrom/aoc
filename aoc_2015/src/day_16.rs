use std::collections::HashMap;

pub fn solve(data: &str) {
    solve_internal::<false>(data);
}

pub fn solve_pt2(data: &str) {
    solve_internal::<true>(data);
}

pub fn solve_internal<const IS_PT2:bool>(data: &str) {
    let sues: Vec<HashMap<String, u32>> = 
    data
    .replace(',', "")
    .lines()
    .map(|line| line.split(' ').collect::<Vec<&str>>())
    .map(|split| 
        [
            (split[2].to_string(), split[3].parse().expect(split[3])), 
            (split[4].to_string(), split[5].parse().expect(split[5])), 
            (split[6].to_string(), split[7].parse().expect(split[7]))
        ].into())
    .collect();

    let sensor:HashMap<String, u32> = [
        ("children:".to_string(), 3),
        ("cats:".to_string(), 7),
        ("samoyeds:".to_string(), 2),
        ("pomeranians:".to_string(), 3),
        ("akitas:".to_string(), 0),
        ("vizslas:".to_string(), 0),
        ("goldfish:".to_string(), 5),
        ("trees:".to_string(), 3),
        ("cars:".to_string(), 2),
        ("perfumes:".to_string(), 1),
    ].into();

    #[allow(clippy::type_complexity)]
    let compare: Box<dyn Fn(&str, &u32) -> bool> = 
        if IS_PT2 {
            Box::new(|item: &str, amount: &u32| -> bool {
                match item {
                    "cats:" | "trees:" => sensor[item] < *amount,
                    "pomeranians:" | "goldfish:" => sensor[item] > *amount,
                    _ => sensor[item] == *amount,
                }
            })
        } else {
            Box::new(|item: &str, amount: &u32| -> bool {
                sensor[item] == *amount
            })
        };
    

    for (i, sue) in sues.into_iter().enumerate() {
        let all_items_match = sue.iter().all(|(item, amount)| compare(item, amount));

        if all_items_match {
            println!("{}", i+1);
            break;
        }
    }

}



#[cfg(test)]
mod tests {
    use super::{solve, solve_pt2};


    #[test]
    fn test_solve() {
        let data = include_str!("day_16_input");
        solve(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_16_input");
        solve_pt2(data);
    }

}
