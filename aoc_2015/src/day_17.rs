
fn solve_internal<const IS_PT2: bool>(data: &str, target: u32) {

    let container_sizes: Vec<u32> = data.lines()
    .map(|l| l.parse().unwrap())
    .collect();

    let combinations = solve_sub_problem(target, &container_sizes, 0, container_sizes.len());
    println!("{:?}", combinations);

    if IS_PT2 {
        let combinations:u32 = combinations.iter().copied()
        .enumerate()
        .filter(|(_, x)| *x > 0)
        .min_by(|(i0, _), (i1, _)| i0.cmp(i1)).unwrap()
        .1;
        println!("{combinations}");
    } else {
        let combinations:u32 = combinations.iter().sum();
        println!("{combinations}");    
    }
}
pub fn solve(data: &str, target: u32) {
    solve_internal::<false>(data, target);
}

pub fn solve_pt2(data: &str) {
    solve_internal::<true>(data, 150);
}

type EggnogAmount = u32;

fn solve_sub_problem(target: EggnogAmount,  container_sizes: &[EggnogAmount], used_containers: usize, n: usize) -> Vec<u32>{

    let mut result:Vec<u32> = vec![0;n]; 

    if target == 0 {
        result[used_containers -1] = 1;
        return result;
    }
    
    if container_sizes.is_empty() {
        return result;
    }

    let container = container_sizes[0];
    
    if target >= container {
        // Include container 
        solve_sub_problem(target - container, &container_sizes[1..], used_containers + 1, n).iter().enumerate()
        .for_each(|(i,x)| {
            result[i] += x;
        }); 
    }

    // Don't include container
    solve_sub_problem(target, &container_sizes[1..], used_containers, n).iter().enumerate()
    .for_each(|(i,x)| {
        result[i] += x;
    });

    result
}



#[cfg(test)]
mod tests {
    use super::{solve, solve_pt2};

    
    #[test]
    fn debug_solve() {
        solve("20\n15\n10\n5\n5", 25);
    }

    #[test]
    fn test_solve() {
        let data = include_str!("day_17_input");
        solve(data, 150);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_17_input");
        solve_pt2(data);
    }

}
