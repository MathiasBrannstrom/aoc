// https://adventofcode.com/2016/day/8

#[derive(Debug)]
enum Command {
    Rect(usize, usize),
    ShiftRow(usize, usize),
    ShiftCol(usize, usize)
}

pub fn solve_pt1(data: &str) {
    let mut screen = [[false; 50];6];


    let commands:Vec<Command> = data.replace("rotate ", "rotate").lines()
    .map(|line| {
        let mut split_iter = line.split_whitespace();

        match split_iter.next() {
            Some("rect") => {
                let (raw_a,raw_b) = split_iter.next().unwrap().split_once('x').unwrap();
                Command::Rect(raw_a.parse().unwrap(), raw_b.parse().unwrap())
            },
            Some("rotaterow") => {
                let (_,raw_a) = split_iter.next().unwrap().split_once('=').unwrap();
                split_iter.next();
                let raw_b = split_iter.next().unwrap();
                Command::ShiftRow(raw_a.parse().unwrap(), raw_b.parse().unwrap())
            },
            Some("rotatecolumn") => {
                let (_,raw_a) = split_iter.next().unwrap().split_once('=').unwrap();
                split_iter.next();
                let raw_b = split_iter.next().unwrap();
                Command::ShiftCol(raw_a.parse().unwrap(), raw_b.parse().unwrap())
            },
            Some(a) => {
                println!("{:?}", a);
                panic!()
            },
            None => {
                panic!("None")
            }
        }
    })
    .collect();

    for command in commands {
        match command {
            Command::Rect(x, y) => {
                for row in screen[0..y].iter_mut() {
                    for pixel in row[0..x].iter_mut() {
                        *pixel = true;
                    }
                }
            },
            Command::ShiftRow(y, amount) => {
                screen[y].rotate_right(amount);
            },
            Command::ShiftCol(x, amount) => {
                let old_screen = screen;
                for i in 0..6{
                    screen[i][x] = old_screen[(((i as i32) - amount as i32 + 6)%6) as usize][x]
                }
            },
        }
    }
    print_screen(&screen);

    let pixels_on = screen.as_flattened().iter().copied().filter(|x| *x).count();
    
    println!("{pixels_on}");
}

fn print_screen(screen: &[[bool; 50];6]){
    for row in screen.iter() {
        for pixel in row.iter() {
            print!("{}", if *pixel {'#'} else {'.'});
        }
        println!();
    }
}

pub fn solve_pt2(data: &str) {
    solve_pt1(data);
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_08_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_08_input");
        solve_pt2(data);
    }

}
