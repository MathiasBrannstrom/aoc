use std::{str::FromStr, vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    r: i64,
    c: i64
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(',').map(str::parse::<i64>);
        let x = sp.next().ok_or("Missing x")?.map_err(|_| "invalid x")?;
        let y = sp.next().ok_or("Missing y")?.map_err(|_| "Invalid y")?;
        Ok(Self{ r: y, c: x })
    }
}

impl Position {
    pub fn RectangleArea(&self, other: &Position) -> i64 {
        ((self.r-other.r).abs() + 1) * ((self.c-other.c).abs() + 1)
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


    let max_r = positions.iter().max_by_key(|p| p.r).unwrap().r + 2;
    let max_c = positions.iter().max_by_key(|p| p.c).unwrap().c + 2;

    let mut last_pos = positions.last().unwrap();
    let mut horizontal_lines:Vec<(Position, Position)> = vec![];
    let mut vertical_lines:Vec<(Position, Position)> = vec![];
    for p in positions.iter() {
        let dir = ((p.r - last_pos.r).signum(), (p.c - last_pos.c).signum());

        if dir.0.abs() == 1 {
            vertical_lines.push((*last_pos, *p));
        } else {
            horizontal_lines.push((*last_pos, *p));
        }

        last_pos = &p;
    }


    let mut combinations:Vec<(Position, Position)> = vec![];
    for (i, p0) in positions[0..positions.len()-1].iter().enumerate() {
        for p1 in positions[i..].iter() {
            let min = Position{r: p0.r.min(p1.r), c: p0.c.min(p1.c)};
            let max = Position{r: p0.r.max(p1.r), c: p0.c.max(p1.c)};

            combinations.push((min, max));
        }
    }
    let count = combinations.len();
    let mut max_valid_area = 0;
    for (i,combination) in combinations.iter().enumerate() {
        
        if i % 10000 == 0 {
            println!("{} / {}", i, count);
        }

        // Check four corners. If any outside, reject combination.

        // Check all rectangle sides against all lines of shape. If any have a crossing, reject combination.
        // Having the side touching the line is OK.

        let mut vertical_sides = vec![];
        let mut horizontal_sides = vec![];

        let c00 = combination.0;
        let c11 = combination.1;
        let c01 = Position{r:combination.0.r, c:combination.1.c};
        let c10 = Position{r:combination.1.r, c:combination.0.c};

        horizontal_sides.push((c00, c01));
        vertical_sides.push((c00, c10));
        vertical_sides.push((c11, c10));
        horizontal_sides.push((c11, c01));

        let mut valid_combination = true;
        'sidesCheck: for (s0, s1) in vertical_sides {
            let c = s0.c;

            let min_r = s0.r.min(s1.r);
            let max_r = s0.r.max(s1.r);
            for (l0, l1) in horizontal_lines.iter().copied() {
                let l_r = l0.r;
                let l_min_c = l0.c.min(l1.c);
                let l_max_c = l0.c.max(l1.c);

                if c >= l_min_c && c <= l_max_c && min_r < l_r && max_r > l_r {
                    valid_combination = false;
                    break 'sidesCheck;
                }
            }
        }
        'sidesCheck: for (s0, s1) in horizontal_sides {
            let r = s0.r;

            let min_c = s0.c.min(s1.c);
            let max_c = s0.c.max(s1.c);
            for (l0, l1) in vertical_lines.iter().copied() {
                let l_c = l0.c;
                let l_min_r = l0.r.min(l1.r);
                let l_max_r = l0.r.max(l1.r);

                if r >= l_min_r && r <= l_max_r && min_c < l_c && max_c > l_c {
                    valid_combination = false;
                    break 'sidesCheck;
                }
            }
        }


        if valid_combination {
            let area = combination.0.RectangleArea(&combination.1);
            if area > max_valid_area {
                max_valid_area = area;
            }
        }
    }

    max_valid_area
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
        p.r > corners.0.r && p.r < corners.1.r &&
        p.c > corners.0.c && p.c < corners.1.c {
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