use regex::*;
use json::*;

pub fn solve_pt2(data: &str) {
    let parsed = json::parse(data).unwrap();

    let sum = add_recursive(&parsed);

    println!("{}", sum.unwrap_or(0));
}

fn add_recursive(json: &JsonValue) -> Option<i32> {

    if json.as_str()== Some("red") {
        return None;
    }

    if let Some(n) = json.as_i32() {
        return Some(n);
    }

    if json.is_array() {
        return Some(json.members()
        .filter_map(add_recursive)
        .sum());
    }

    let results:Vec<Option<i32>> = json.entries().map(|(_,v)| add_recursive(v)).collect();

    if results.iter().any(|r| r.is_none()){
        return Some(0);
    }

    Some(results.into_iter().flatten().sum())
}

pub fn solve(data: &str) {

    let reg = Regex::new(r"-?\d+").unwrap();

    let sum:i32 = reg.find_iter(data)
    .map(|x| x.as_str().parse::<i32>().unwrap())
    .sum();

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::{solve, solve_pt2};


    #[test]
    fn test_solve() {
        let data = include_str!("day_12_input");
        solve(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_12_input");
        solve_pt2(data);
    }

    #[test]
    fn test_1_solve_pt2() {
        solve_pt2(r#"[1,2,3]"#);
        solve_pt2(r#"[1,{"c":"red","b":2},3]"#);
        solve_pt2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#);
        solve_pt2(r#"[1,"red",5]"#);
        
    }

    #[test]
    fn test_2_solve_pt2() {
        solve_pt2(r#"[1,{"c":2, "d":{"a":3, "f":"red"}},3]"#);
    }

}
