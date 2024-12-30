use std::ops::{Add, Mul};


#[cfg(test)]
mod tests {
    use super::{solve, solve_pt2};


    #[test]
    fn test_solve() {
        let data = include_str!("day_15_input");
        solve(data);
    }

    
    #[test]
    fn debug_solve() {
        let data = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#.to_string();
        solve(data.as_str());
    }
    

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_15_input");
        solve_pt2(data);
    }

}

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32   
}

impl Add for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Self) -> Self::Output {
        Ingredient {
            capacity:  (self.capacity + rhs.capacity),
            durability: (self.durability + rhs.durability),
            flavor: (self.flavor + rhs.flavor),
            texture: (self.texture + rhs.texture),
            calories: (self.calories + rhs.calories),
        }
    }
}

impl Mul<i32> for Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i32) -> Self::Output {
        Ingredient {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}


pub fn solve(data: &str) {
    solve_internal::<false>(data);
}

pub fn solve_pt2(data: &str) {
    solve_internal::<true>(data);
}

fn solve_internal<const IS_PT2: bool>(data: &str) {

    let ingredients:Vec<Ingredient> = data
    .replace(',', "")
    .lines()
    .map(|line| line.split(' ').collect::<Vec<&str>>())
    .map(|split| Ingredient {
        capacity: split[2].parse().unwrap(),
        durability: split[4].parse().unwrap(),
        flavor: split[6].parse().unwrap(),
        texture: split[8].parse().unwrap(),
        calories: split[10].parse().unwrap()})
    .collect();

    let mut best_score = 0;
    for i in 0..=100 {
        for j in i..=100 {
            for k in j..=100 {
                let ingredient_amounts = [i, j-i, k-j, 100-k];
                
                let mut total_ingredient = Ingredient{capacity:0, durability: 0, flavor: 0, texture: 0, calories: 0};
                for m in 0..4 {
                    total_ingredient = total_ingredient + ingredients[m]*ingredient_amounts[m]
                }

                if IS_PT2 && total_ingredient.calories != 500 {
                    continue;
                }

                let score = 
                    total_ingredient.capacity.max(0)
                    * total_ingredient.durability.max(0)
                    * total_ingredient.flavor.max(0)
                    * total_ingredient.texture.max(0);

                if score > best_score {
                    best_score = score;
                }
            }
        }
    }
    println!("{}", best_score);
}