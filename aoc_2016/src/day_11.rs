// https://adventofcode.com/2016/day/11

use std::collections::{HashMap, HashSet};

type Floor = u32;
type Steps = u32;
#[derive(Debug, Clone, Copy)]
enum Item {
    Generator(u32),
    Microchip(u32),
}

type State = HashMap<Floor, Vec<Item>>;

pub fn solve_pt1(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let initial_state = parse_input(data)?;
    let mut visited_states = HashMap::new();
    let steps = steps_until_final_state_from_state(&initial_state, 0, &mut visited_states);

    println!("Steps: {}", steps);
    Ok(())
}

fn steps_until_final_state_from_state(state: &State, steps: Steps, visited_states: &mut HashMap<State, Steps>) -> u32 {
    // Recursively call all possible states from current state until final state is reached. 
    // if visited_states
    
    0
}


fn parse_input(data: &str) -> Result<State, Box<dyn std::error::Error>> {
    let mut floors = HashMap::new();
    
    let mut type_map:HashMap<String, u32> = HashMap::new();

    for (i, line) in data.lines().enumerate() {
        let floor = i as Floor;
        let mut items = Vec::new();
        let mut prev_word:Option<&str> = None;
        for word in line.split_whitespace() {
            let word = word.trim_end_matches('.').trim_end_matches(',').trim_end_matches("-compatible");   
            if word == "generator" || word == "microchip" {
                let item_type = prev_word.ok_or("Invalid input")?;
                let current_amount_of_items = type_map.len();
                let item_type = type_map.entry(item_type.to_string()).or_insert(current_amount_of_items as u32);
                
                match word {
                    "generator" => items.push(Item::Generator(*item_type)),
                    "microchip" => items.push(Item::Microchip(*item_type)),
                    _ => return Err("Invalid input".into()),
                }
                
            }
            prev_word = Some(word);
        }

        floors.insert(floor, items);
    }

    Ok(floors)
}

pub fn solve_pt2(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_11_input");
        let result = solve_pt1(data);
        println!("{:?}", result);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_11_input");
        let result = solve_pt2(data);
        println!("{:?}", result);
    }

}
