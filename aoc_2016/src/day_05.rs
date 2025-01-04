// https://adventofcode.com/2016/day/5

pub fn solve_pt1(data: &str) {
    let mut i = 0;
    let mut password = String::new();
    let mut context = md5::Context::new();
    context.consume(data);

    loop {
        let mut context_clone = context.clone();
        context_clone.consume(i.to_string());
        let digest = context_clone.compute();
        let hex_digest = format!("{:x}", digest);

        if hex_digest.starts_with("00000"){
            password.push(hex_digest.chars().nth(5).unwrap());
            if password.len() == 8 {
                break;
            }
        }

        i += 1;
    }   

    println!("Password: {}", password);
    
}

pub fn solve_pt2(data: &str) {
    let mut i = 0;
    let mut password:[Option<char>;8] = [None; 8];
    let mut found_chars = 0;
    let mut context = md5::Context::new();
    context.consume(data);

    loop {
        let mut context_clone = context.clone();
        context_clone.consume(i.to_string());
        let digest = context_clone.compute();
        let hex_digest = format!("{:x}", digest);

        if hex_digest.starts_with("00000"){
            let pos = hex_digest.chars().nth(5).unwrap().to_digit(10);
            let pos = match pos {
                Some(pos) => pos as usize,
                None => {
                    i += 1;
                    continue;
                }
            };
            if pos < 8 && password[pos].is_none() {
                password[pos] = Some(hex_digest.chars().nth(6).unwrap());
                found_chars += 1;
                let password_str = password.iter().map(|c| c.unwrap_or('_')).collect::<String>();
                println!("Password: {}", password_str);
            }

            if found_chars == 8 {
                break;
            }
        }

        i += 1;
    }   

    println!("Password: {}", password.map(|c| c.unwrap()).into_iter().collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_05_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_05_input");
        solve_pt2(data);
    }

}
