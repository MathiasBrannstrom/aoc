mod day_8_after_ai_cleanup;
mod day_9;

fn main() {
    let data = include_str!("inputs/day_9_input");

    let pt1_result = day_9::solve_pt1(data);
    println!("Part 1: {pt1_result}");

    let pt2_result = day_9::solve_pt2(data);
    println!("Part 2: {pt2_result}");
}
