use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(data: &str) {
    
    let distances:HashMap<(String, String), u32> = data.lines()
    .flat_map(|line| {
        let split: Vec<&str> = line.split(' ').collect();
        [((split[0].to_string(), split[2].to_string()), split[4].parse().unwrap()),
        ((split[2].to_string(), split[0].to_string()), split[4].parse().unwrap())]
    })
    .collect();

    let all_locations: HashSet<String> = distances.keys()
    .flat_map(|x| [x.0.clone(), x.1.clone()])
    .collect();

    let shortest_distance = all_locations.iter().cloned()
    .permutations(all_locations.len())
    .map(|permutation| permutation.iter().cloned()
        .tuple_windows::<(_,_)>()
        .map(|location_pair| distances.get(&(location_pair.0, location_pair.1)).unwrap())
        .sum::<u32>())
    .min().unwrap();
    
    println!("{}", shortest_distance);
}

pub fn solve_pt2(data: &str) {
    
    let distances:HashMap<(String, String), u32> = data.lines()
    .flat_map(|line| {
        let split: Vec<&str> = line.split(' ').collect();
        [((split[0].to_string(), split[2].to_string()), split[4].parse().unwrap()),
        ((split[2].to_string(), split[0].to_string()), split[4].parse().unwrap())]
    })
    .collect();

    let all_locations: HashSet<String> = distances.keys()
    .flat_map(|x| [x.0.clone(), x.1.clone()])
    .collect();

    let longest_distance = all_locations.iter().cloned()
    .permutations(all_locations.len())
    .map(|permutation| permutation.iter().cloned()
        .tuple_windows::<(_,_)>()
        .map(|location_pair| distances.get(&(location_pair.0, location_pair.1)).unwrap())
        .sum::<u32>())
    .max().unwrap();
    
    println!("{}", longest_distance);
}


#[cfg(test)]
mod tests{
    use super::{solve, solve_pt2};

    #[test]
    fn test_solve(){
        let data = include_str!("day_9_input");
        solve(data);
    }

    #[test]
    fn test_solve_pt2(){
        let data = include_str!("day_9_input");
        solve_pt2(data);
    }
}

