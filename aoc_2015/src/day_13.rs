use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve_pt2(data: &str) {
    let (mut names, mut happiness) = parse_data(data);

    for name in names.iter() {
        happiness.insert(("Mathias".to_string(), name.clone()), 0);
        happiness.insert((name.clone(), "Mathias".to_string()), 0);
    }
    names.insert("Mathias".to_string());
    
    let max = names.iter().cloned()
    .permutations(names.len())
    .map(|permutation| permutation.iter().cloned()
        .circular_tuple_windows()
        .map(|(l,m,r)| happiness.get(&(m.clone(),l.clone())).unwrap_or_else(|| panic!("No entry for {} {}", m, l)) + happiness.get(&(m.clone(),r.clone())).unwrap_or_else(|| panic!("No entry for {} {}", m, r)))
        .sum::<i32>())
    .max().expect("Max was empty?");
    
    println!("{}", max);

}


pub fn solve(data: &str) {
    let (names, happiness) = parse_data(data);

    println!("{:?}", names);
    println!("{:?}", happiness);

    let max = names.iter().cloned()
    .permutations(names.len())
    .map(|permutation| permutation.iter().cloned()
        .circular_tuple_windows()
        .map(|(l,m,r)| happiness.get(&(m.clone(),l.clone())).unwrap_or_else(|| panic!("No entry for {} {}", m, l)) + happiness.get(&(m.clone(),r.clone())).unwrap_or_else(|| panic!("No entry for {} {}", m, r)))
        .sum::<i32>())
    .max().expect("Max was empty?");
    
    println!("{}", max);
}

type Name = String;

fn parse_data(data: &str) -> (HashSet<Name>, HashMap<(Name, Name), i32>) {
    let mut names = HashSet::new();
    let mut happiness = HashMap::new();

    let parsed:Vec<((Name, Name), i32)> = data.lines()
    .map(|line| line.split(' ').collect())
    .map(|split:Vec<&str>| ((split[0].to_string(), (split[10][..(split[10].len()-1)]).to_string()), (if split[2]=="gain" {1} else {-1}) * split[3].parse::<i32>().unwrap()))
    .collect();

    for x in parsed {
        names.insert(x.0.0.clone());
        happiness.insert((x.0.0.clone(), x.0.1.clone()), x.1);
    }

    (names, happiness)
}

#[cfg(test)]
mod tests {
    use super::{solve, solve_pt2};


    #[test]
    fn test_solve() {
        let data = include_str!("day_13_input");
        solve(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_13_input");
        solve_pt2(data);
    }

}
