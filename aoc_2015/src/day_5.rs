use std::collections::{HashMap, HashSet};

pub fn solve(data: &str) {

    let amount_of_nice_strings = data.lines()
    .filter(|line| {
        is_nice(line)
    })
    .count();

    println!("{}", amount_of_nice_strings);
}

pub fn solve_pt2(data: &str) {

    let amount_of_nice_strings = data.lines()
    .filter(|line| {
        is_nice_pt2(line)
    })
    .count();

    println!("{}", amount_of_nice_strings);
}

fn is_nice_pt2(line: &str) -> bool {
    
    let mut position_of_pairs:HashMap<[char;2], Vec<usize>> = HashMap::new();

    let mut letter_two_ago = '1';
    let mut last_letter = line.chars().next().unwrap();
    let mut found_repeat = false;
    let mut pos = 1;
    for letter in line.chars().skip(1) {

        if letter == letter_two_ago {
            found_repeat = true;
        }

        position_of_pairs.entry([letter, last_letter]).or_default().push(pos);

        pos += 1;
        letter_two_ago = last_letter;
        last_letter = letter;
    }

    let any_non_overlapping_pair = {
        let mut found = false;
        for (_, positions) in position_of_pairs {
            if positions.len() > 2 {
                found = true;
                break;
            }   

            if positions.len() == 2 
            && positions[0].abs_diff(positions[1]) > 1 {
                found = true;
                break;
            }
        }
        found
    };

    found_repeat && any_non_overlapping_pair
}

fn is_nice(line: &str) -> bool {
    let mut vowel_count = 0;
    let mut found_letter_in_a_row = false;
    let mut last_letter = '1';

    let vowels:HashSet<char> = HashSet::from_iter(vec!['a', 'e', 'i', 'o', 'u']);

    for letter in line.chars() {
        if vowels.contains(&letter) {
            vowel_count += 1;
        }
        if last_letter == letter {
            found_letter_in_a_row = true;
        }

        let invalid_segment_found = match last_letter {
            'a' => letter == 'b',
            'c' => letter == 'd',
            'p' => letter == 'q',
            'x' => letter == 'y',
            _ => false
        };

        if invalid_segment_found {
            return false;
        }

        last_letter = letter;
    }

    found_letter_in_a_row && vowel_count >= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = include_str!("day_5_input");
        solve(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_5_input");
        solve_pt2(data);
    }
}