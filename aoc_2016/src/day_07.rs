// https://adventofcode.com/2016/day/7

use std::collections::HashSet;

pub fn solve_pt1(data: &str) {
    let tls_count = data.lines()
    .filter(|line| supports_tls(line))
    .count();

    println!("{tls_count}");
}

fn supports_tls(ip: &str) -> bool {
    let mut in_brackets = false;
    let mut has_abba = false;
    
    let ip_chars:Vec<char> = ip.chars().collect();

    for char in ip_chars[0..3].iter() {
        match char {
            '[' => in_brackets = true,
            ']' => in_brackets = false,
            _ => ()
        };
    }

    for (i,char) in ip_chars.iter().enumerate().skip(3) {
        
        match char {
            '[' => {
                in_brackets = true;
                continue;
            },
            ']' => {
                in_brackets = false;
                continue;
            },
            _ => ()
        };

        if ip_chars[i-3] == ip_chars[i] 
        && ip_chars[i-2] == ip_chars[i-1] 
        && ip_chars[i-1] != ip_chars[i] {
            if in_brackets {
                return false;
            }

            has_abba = true;
        }
    }

    has_abba
}

pub fn solve_pt2(data: &str) {
    let ssl_count = data.lines()
    .filter(|line| supports_ssl(line))
    .count();

    println!("{ssl_count}");
}

fn supports_ssl(ip: &str) -> bool{

    let mut in_brackets = false;
    let mut abas_found:HashSet<String> = HashSet::new();
    let mut babs_found:HashSet<String> = HashSet::new();
    
    let ip_chars:Vec<char> = ip.chars().collect();

    for char in ip_chars[0..2].iter() {
        match char {
            '[' => in_brackets = true,
            ']' => in_brackets = false,
            _ => ()
        };
    }

    for (i,char) in ip_chars.iter().enumerate().skip(2) {
        
        match char {
            '[' => {
                in_brackets = true;
                continue;
            },
            ']' => {
                in_brackets = false;
                continue;
            },
            _ => ()
        };

        if ip_chars[i-2] == ip_chars[i] 
        && ip_chars[i-1] != ip_chars[i] {
            let sequence:String = ip_chars[i-2..i].iter().collect();

            if in_brackets {
                babs_found.insert(sequence);
                continue;
            }

            abas_found.insert(sequence);
        }
    }

    for aba in abas_found.iter() {
        let expected_bab = format!("{}{}", aba.chars().nth(1).unwrap(), aba.chars().next().unwrap());
        if babs_found.contains(&expected_bab){
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_07_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_07_input");
        solve_pt2(data);
    }

}
