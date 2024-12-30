
pub fn solve(data: &str) {

    let amount_of_paper:u32 =
    data.lines()
        .map(|line| 
            line.split('x')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>())
        .map(|dims| (dims[0], dims[1], dims[2]))
        .map(|dims| {
            let side_0 = dims.0*dims.1;
            let side_1 = dims.0*dims.2;
            let side_2 = dims.1*dims.2;

            2*side_0+2*side_1+2*side_2 + side_0.min(side_1).min(side_2)
        })
        .sum();
        
        println!("{}", amount_of_paper);
}

pub fn solve_pt2(data: &str) {

    let amount_of_ribbon:u32 =
    data.lines()
        .map(|line| 
            line.split('x')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>())
        .map(|dims| (dims[0], dims[1], dims[2]))
        .map(|dims| {
            let volume = dims.0*dims.1*dims.2;
            let shortest_perimiter = 2*(dims.0+dims.1).min(dims.0+dims.2).min(dims.1+dims.2);

            shortest_perimiter+volume
        })
        .sum();
        
        println!("{}", amount_of_ribbon);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = include_str!("day_2_input");
        solve(data);
    }
    
    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_2_input");
        solve_pt2(data);
    }
    
}