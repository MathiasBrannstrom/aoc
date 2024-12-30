use std::collections::HashMap;

type Output = String;
type OneInput = String;

#[derive(Debug, Clone)]
struct TwoInputs{
    input_a: String,
    input_b: String
}

#[derive(Debug, Clone)]
enum Gate {
    And(TwoInputs),
    Or(TwoInputs),
    LShift(TwoInputs),
    RShift(TwoInputs),
    Not(OneInput),
    Transfer(OneInput)

}

pub fn solve(data: &str) {

    let gates = parse_gates(data);
    let mut cache:HashMap<String, u16> = HashMap::new();
    let a_value = dfs("a".to_string(), &gates, &mut cache);

    println!("{}", a_value);
}

pub fn solve_pt2(data: &str) {

    let mut gates = parse_gates(data);
    gates.insert("b".to_string(), Gate::Transfer("16076".to_string()));
    
    let mut cache:HashMap<String, u16> = HashMap::new();
    let a_value = dfs("a".to_string(), &gates, &mut cache);

    println!("{}", a_value);
}

fn dfs(gate_output: Output, gates: &HashMap<Output, Gate>, cache: &mut HashMap<String, u16>) -> u16 {
    
    if cache.contains_key(&gate_output) {
        return *cache.get(&gate_output).unwrap();
    }

    let possible_number: Option<u16> = gate_output.parse().ok();

    if let Some(number) = possible_number {
        return number;
    }
    
    let gate = gates.get(&gate_output).unwrap();

    let value = match gate.clone() {
        Gate::And(ins) => dfs(ins.input_a, gates, cache) & dfs(ins.input_b, gates, cache),
        Gate::Or(ins) => dfs(ins.input_a, gates, cache) | dfs(ins.input_b, gates, cache),
        Gate::LShift(ins) => dfs(ins.input_a, gates, cache) << dfs(ins.input_b, gates, cache),
        Gate::RShift(ins) => dfs(ins.input_a, gates, cache) >> dfs(ins.input_b, gates, cache),
        Gate::Not(input) => dfs(input, gates, cache) ^ u16::MAX,
        Gate::Transfer(input) => dfs(input, gates, cache),
    };

    cache.insert(gate_output, value);
    value
}



fn parse_gates(data: &str) -> HashMap<Output, Gate> {
    let mut gates: HashMap<Output, Gate> = HashMap::new();

    for line in data.lines() {
        let split_line:Vec<&str> = line.split(' ').collect();

        if split_line.len() == 3 {
            gates.insert(split_line[2].to_string(), Gate::Transfer(split_line[0].to_string()));
            continue;
        }

        if split_line[0] == "NOT" {
            gates.insert(split_line[3].to_string(), Gate::Not(split_line[1].to_string()));
            continue;
        }

        let two_inputs = TwoInputs {
            input_a: split_line[0].to_string(),
            input_b: split_line[2].to_string()
        };

        let gate = match split_line[1] {
            "AND" => Gate::And(two_inputs),
            "OR" => Gate::Or(two_inputs),
            "LSHIFT" => Gate::LShift(two_inputs),
            "RSHIFT" => Gate::RShift(two_inputs),
            _ => {
                println!("{}", line);
                panic!();
            }
        };

        gates.insert(split_line[4].to_string(), gate);
    }

    gates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = include_str!("day_7_input");
        solve(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_7_input");
        solve_pt2(data);
    }
}
