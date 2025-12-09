use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(',').map(str::parse::<i64>);
        let x = sp.next().ok_or("Missing x")?.map_err(|_| "invalid x")?;
        let y = sp.next().ok_or("Missing y")?.map_err(|_| "Invalid y")?;
        Ok(Self{ x, y })
    }
}

impl Position {
    pub fn RectangleArea(&self, other: &Position) -> i64 {
        ((self.x-other.x).abs() + 1) * ((self.y-other.y).abs() + 1)
    }
}

pub fn solve_pt1(data: &str) -> i64 {
    let positions: Vec<Position> = data.lines().map(|l| l.parse().expect("Parsing position failed")).collect();

    let mut combinations:Vec<(Position, Position)> = vec![];
    for (i, p0) in positions[0..positions.len()-1].iter().enumerate() {
        for p1 in positions[i..].iter() {
            combinations.push((*p0, *p1));
        }
    } 

    combinations.iter().map(|c| c.0.RectangleArea(&c.1)).max().unwrap()
}

pub fn solve_pt2(data: &str) -> i64 {
    let mut positions: Vec<Position> = data.lines().map(|l| l.parse().expect("Parsing position failed")).collect();
    let count = positions.len();
    positions.append(&mut positions.clone());


    let mut max_valid_area = 0;

    // for (i,p0) in positions[0..count].iter().enumerate() {
    //     let mut current_direction = 
    //     for p1 in positions[i+1..i+count].iter() {
            
    //     }
    // }

    // max_valid_area
    // let mut combinations:Vec<(Position, Position)> = vec![];
    // for (i, p0) in positions[0..positions.len()-1].iter().enumerate() {
    //     for p1 in positions[i..].iter() {
    //         let min = Position{x: p0.x.min(p1.x), y: p0.y.min(p1.y)};
    //         let max = Position{x: p0.x.max(p1.x), y: p0.y.max(p1.y)};

    //         combinations.push((min, max));
    //     }
    // }
    // println!("{}", combinations.len());
    // let filtered:Vec<(Position, Position)> = combinations.iter().copied()
    // .filter(|c| no_corner_inside(c, &positions)).collect();

    // println!("{}", filtered.len());

    // filtered.iter()
    // .map(|c| c.0.RectangleArea(&c.1))
    // .max()
    // .unwrap()
}

fn no_corner_inside(corners: &(Position, Position), positions: &[Position]) -> bool {
    if corners.0.RectangleArea(&corners.1) == 40 {
        println!("Hej");
    }
    
    for p in positions {
        if 
        p.x > corners.0.x && p.x < corners.1.x &&
        p.y > corners.0.y && p.y < corners.1.y {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    const DATA : &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    pub fn test_pt1() {
        let result = solve_pt1(DATA);
        assert!(result == 50);
    }

    #[test]
    pub fn test_pt2() {
        let result = solve_pt2(DATA);
        println!("{}", result);
        assert!(result == 24);
    }
}