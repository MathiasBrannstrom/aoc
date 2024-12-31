pub mod day_19;

pub fn main() {

    let test_data = r#"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO"#;
    let data = include_str!("day_19_input");
    day_19::solve_pt2(data);
}