// https://adventofcode.com/2015/day/19

use std::collections::{HashMap, HashSet};

fn parse_replacements(data: &str) -> Vec<(&str, &str)> {
    data.lines().take_while(|line| !line.is_empty()).map(|line| {
        let mut parts = line.split(" => ");
        (parts.next().unwrap(), parts.next().unwrap())
    }).collect()
}

fn parse_data(data: &str) -> (Vec<(&str, &str)>, &str) {
    let (replacements_raw, start_molecule) = data.split_once("\n\n").unwrap();
    let replacements = parse_replacements(replacements_raw);
    
    (replacements, start_molecule)
}

pub fn solve_pt1(data: &str) {
    let (replacements, start_molecule) = parse_data(data);

    let mut molecules:HashSet<String> = std::collections::HashSet::new();

    for (from, to) in replacements {
        for (i, _) in start_molecule.match_indices(from) {
            let new_molecule = start_molecule[..i].to_string() + to + &start_molecule[i+from.len()..];
            molecules.insert(new_molecule);
        }
    }

    println!("Molecules: {}", molecules.len());
}

#[derive(Debug, Clone)]
struct Evaluation {
    min_steps_to_e: u32,
    molecule: String,
    steps_taken_so_far: u32,
}

impl PartialEq for Evaluation {
    fn eq(&self, other: &Self) -> bool {
        self.steps_taken_so_far + self.min_steps_to_e == other.steps_taken_so_far + other.min_steps_to_e
    }
}

impl Eq for Evaluation {}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Evaluation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.steps_taken_so_far.cmp(&other.steps_taken_so_far) {
            std::cmp::Ordering::Equal => other.min_steps_to_e.cmp(&(self.min_steps_to_e)),
            res => res,
        }
    }
}

pub fn solve_pt2(data: &str) {
    let (replacements, target_molecule) = parse_data(data);

    let mut evaluations = vec![Evaluation{steps_taken_so_far:0, min_steps_to_e: min_steps_from_molecule_to_e(target_molecule), molecule: target_molecule.to_string()}];

    let mut lowest_visit_score:HashMap<String, u32> = HashMap::new();

    while let Some(evaluation) = evaluations.pop() {
        // println!("Steps taken: {}, Min steps left: {}, Best score: {},  Len: {}", evaluation.steps_taken_so_far, evaluation.min_steps_to_e, evaluation.min_steps_to_e + evaluation.steps_taken_so_far, evaluation.molecule.len());
        if lowest_visit_score.contains_key(&evaluation.molecule) && lowest_visit_score[&evaluation.molecule] <= evaluation.steps_taken_so_far {
            continue;
        }

        // For any given input this might not be the shortest path, but it's correct for the given input.
        // A general solution would need to continue evaluating possible paths, but could prune paths that would never be shorter than the current best path.
        if evaluation.molecule == "e" {
            println!("Steps: {}", evaluation.steps_taken_so_far);
            break;
        }

        lowest_visit_score.insert(evaluation.molecule.clone(), evaluation.steps_taken_so_far);

        for (from, to) in &replacements {
            for (i, _) in evaluation.molecule.match_indices(to) {
                let new_molecule = evaluation.molecule[..i].to_string() + from + &evaluation.molecule[i+to.len()..];
                let new_evaluation = Evaluation{steps_taken_so_far: evaluation.steps_taken_so_far + 1, min_steps_to_e: min_steps_from_molecule_to_e(&new_molecule), molecule: new_molecule};
                evaluations.push(new_evaluation);

            }
        }
        
        evaluations.sort_unstable();
    }

}

pub fn min_steps_from_molecule_to_e(molecule: &str) -> u32 {
    // This is hard-coded for the given input.
    const BIGGEST_REPLACEMENT: u32 = 9;

    (molecule.len() as u32-1).div_ceil(BIGGEST_REPLACEMENT).max(1)
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_19_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_19_input");
        solve_pt2(data);
    }
    #[test]
    fn test_min_steps_from_molecule_to_e() {
        (1..35).for_each(|i|{
            println!("{}: {}", i, super::min_steps_from_molecule_to_e(&"A".repeat(i)));
        });
    }

    #[test]
    fn debug_solve_pt2() {
        let data = r#"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO"#;
        solve_pt2(data);
    }

}
