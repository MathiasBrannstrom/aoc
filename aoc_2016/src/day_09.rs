// https://adventofcode.com/2016/day/9

use anyhow::anyhow;
use regex::Regex;

pub fn solve_pt1(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut decompressed_len = 0;
    let mut position = 0;

    let re = Regex::new(r"\((\d+)x(\d+)\)")?;

    loop {
        
        let captures = re.captures_at(data, position);

        let captures = match captures {
            Some(captures) => captures,
            None => {
                decompressed_len += data.len() - position;
                break;
            }
        };

        let (len, repeat) = (captures[1].parse::<usize>()?, captures[2].parse::<usize>()?);
        let start = captures.get(0).ok_or(anyhow!("Failed to find first capture"))?.start();
        decompressed_len +=  start-position;
        decompressed_len += len * repeat;

        position = start + captures[0].len() + len;
    }

    println!("Decompressed length: {}", decompressed_len);
    Ok(())
}


pub fn solve_pt2(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let decompressed_len = get_decompressed_length(data)?;

    println!("Decompressed length: {}", decompressed_len);
    Ok(())
}

fn get_decompressed_length(text: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut decompressed_len = 0;
    let mut position = 0;

    let re = Regex::new(r"\((\d+)x(\d+)\)")?;

    loop {
        
        let captures = re.captures_at(text, position);

        let captures = match captures {
            Some(captures) => captures,
            None => {
                decompressed_len += text.len() - position;
                break;
            }
        };

        let (len, repeat) = (captures[1].parse::<usize>()?, captures[2].parse::<usize>()?);
        let start = captures.get(0).ok_or(anyhow!("Failed to find first capture"))?.start();

        let repeat_text_start = start + captures[0].len();

        let repeat_text = &text[repeat_text_start..repeat_text_start + len];
        let decompressed_repeat_text_len = get_decompressed_length(repeat_text)?;

        decompressed_len += start-position;
        decompressed_len += decompressed_repeat_text_len * repeat;

        position = start + captures[0].len() + len;
    }

    Ok(decompressed_len)
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_09_input");
        let result = solve_pt1(data);
        println!("{:?}", result);
    }

    #[test]
    fn debug_solve_pt1() {
        let data = "A(2x2)BCD(2x2)EFG";
        let result = solve_pt1(data);
        println!("{:?}", result);
    }


    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_09_input");
        let result = solve_pt2(data);
        println!("{:?}", result);
    }

    #[test]
    fn debug_solve_pt2() {
        let data = "X(8x2)(3x3)ABCY";
        let result = solve_pt2(data);
        println!("{:?}", result);
    }


}
