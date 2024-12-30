
#[cfg(test)]
mod tests {
    use super::{solve, solve_pt2};


    #[test]
    fn test_solve() {
        let data = include_str!("day_14_input");
        solve(data, 2503);
    }

    #[test]
    fn debug_solve() {
        let data = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#.to_string();
        solve(data.as_str(), 1000);
    }

//

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_14_input");
        solve_pt2(data, 2503);
    }

    #[test]
    fn debug_solve_pt2() {
        let data = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#.to_string();
        solve_pt2(data.as_str(), 1000);
    }
}

type Speed = u32;
type FlyTime = u32;
type RestTime = u32;
type Distance = u32;
type Points = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Flying(u32),
    Resting(u32)
}

pub fn solve_pt2(data: &str, inspect_time: u32) {
    let reindeers:Vec<(Speed, FlyTime, RestTime)> = data.lines()
    .map(|line| line.split(' ').collect::<Vec<&str>>())
    .map(|split| (split[3].parse().unwrap(), split[6].parse().unwrap(), split[13].parse().unwrap()))
    .collect();

    let mut reindeer_states: Vec<(State, Distance, Points)> = reindeers.iter()
    .map(|_| (State::Flying(0), 0, 0)).collect();

    for _ in 0..inspect_time {
        for i in 0..reindeers.len() {
            let (state, mut distance, points) = reindeer_states.get(i).unwrap();
            let (speed, fly_time, rest_time) = reindeers.get(i).unwrap();
            

            let next_state = match state {
                State::Flying(time_in_air) => {
                    if *time_in_air == *fly_time {State::Resting(1) } else { State::Flying(time_in_air + 1)}
                },
                State::Resting(time_resting) => {
                    if *time_resting == *rest_time {State::Flying(1)} else { State::Resting(time_resting + 1)}
                }    
            };

            if let State::Flying(_) = next_state{
                distance += speed;
            }


            reindeer_states[i] = (next_state, distance, *points);
        }

        let max_distance = reindeer_states.iter().map(|(_, distance, _)| *distance).max().unwrap();
        for (_, distance, points) in reindeer_states.iter_mut() {
            if *distance == max_distance {
            *points += 1;
            }
        }

        // println!("{:?}", reindeer_states);
    }
    
    let max_points = reindeer_states.iter().map(|(_, _, points)| *points).max().unwrap();

    println!("{}", max_points);
}

pub fn solve(data: &str, inspect_time: u32) {
    let reindeers:Vec<(Speed, FlyTime, RestTime)> = data.lines()
    .map(|line| line.split(' ').collect::<Vec<&str>>())
    .map(|split| (split[3].parse().unwrap(), split[6].parse().unwrap(), split[13].parse().unwrap()))
    .collect();

    let winning_distance:u32 = reindeers.iter().copied()
    .map(|(speed, fly_time, rest_time)| {

        let cycle_time = fly_time+rest_time;
        let number_of_full_cycles = inspect_time / cycle_time;
        
        let time_left = inspect_time - number_of_full_cycles * cycle_time;
        let distance_flewn_on_last_cycle = time_left.min(fly_time) * speed;
        
        distance_flewn_on_last_cycle + number_of_full_cycles * (fly_time*speed)
    })
    .max().unwrap();

    println!("{}", winning_distance);
}

