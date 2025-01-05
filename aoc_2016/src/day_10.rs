// https://adventofcode.com/2016/day/10

use std::collections::HashMap;

use anyhow::anyhow;

type ChipValue = usize;

#[derive(Debug, Clone, Copy)]
enum Target {
    Bot(usize),
    Output(usize),
}

struct Bot {
    low: Target,
    high: Target,
    chips: Vec<ChipValue>,
}

type ParseResult = Result<(HashMap<usize, Bot>, Vec<usize>), Box<dyn std::error::Error>>;

fn parse_input(data: &str) -> ParseResult {
    let bot_setup_lines = data.lines().filter(|l| l.starts_with("bot"));
    let mut bots:HashMap<usize, Bot> = HashMap::new();

    for line in bot_setup_lines {

        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let bot_num = parts[1].parse::<usize>()?;
        let low_target = match parts[5] {
            "bot" => Target::Bot(parts[6].parse::<usize>()?),
            "output" => Target::Output(parts[6].parse::<usize>()?),
            _ => return Err("Invalid target".into()),
        };
        let high_target = match parts[10] {
            "bot" => Target::Bot(parts[11].parse::<usize>()?),
            "output" => Target::Output(parts[11].parse::<usize>()?),
            _ => return Err("Invalid target".into()),
        };

        let bot = Bot {
            low: low_target,
            high: high_target,
            chips: Vec::new(),
        };
        bots.insert(bot_num, bot);
    }
    
    let value_lines = data.lines().filter(|l| l.starts_with("value"));
    let mut bots_with_two_chips = vec![];

    for line in value_lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let chip_value = parts[1].parse::<ChipValue>()?;
        let bot_num = parts[5].parse::<usize>()?;
        let bot = bots.get_mut(&bot_num).ok_or(anyhow!("Bot not found"))?;
        bot.chips.push(chip_value);
        
        if bot.chips.len() == 2 {
            bots_with_two_chips.push(bot_num);
        }
    }


    Ok((bots, bots_with_two_chips))
}

pub fn solve_internal<const IS_PT2: bool>(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (mut bots, bots_with_two_chips) = parse_input(data)?;
    let mut bots_to_process = bots_with_two_chips.clone();
    let mut output:HashMap<usize, Vec<ChipValue>> = HashMap::new();

    while let Some(bot_id) = bots_to_process.pop() {
        let (low_chip, high_chip, low_target, high_target) = {
            let bot = bots.get_mut(&bot_id).ok_or(anyhow!("Bot not found"))?;
            if !IS_PT2 && bot.chips.contains(&61) && bot.chips.contains(&17) {
                println!("Bot {} is responsible for comparing chips 61 and 17", bot_id);
                break;
            }

            let low_chip = *bot.chips.iter().min().ok_or(anyhow!("No chips"))?;
            let high_chip = *bot.chips.iter().max().ok_or(anyhow!("No chips"))?;
            
            bot.chips.clear();
            (low_chip, high_chip, bot.low, bot.high)
            
        };

        match low_target {
            Target::Bot(low_bot_id) => {
                let low_bot = bots.get_mut(&low_bot_id).ok_or(anyhow!("Bot not found"))?;
                low_bot.chips.push(low_chip);
                if low_bot.chips.len() == 2 {
                    bots_to_process.push(low_bot_id);
                }
            },
            Target::Output(output_id) => {
                let output = output.entry(output_id).or_default();
                output.push(low_chip);
            }
        }

        match high_target {
            Target::Bot(high_bot_id) => {
                let high_bot = bots.get_mut(&high_bot_id).ok_or(anyhow!("Bot not found"))?;
                high_bot.chips.push(high_chip);
                if high_bot.chips.len() == 2 {
                    bots_to_process.push(high_bot_id);
                }
            },
            Target::Output(output_id) => {
                let output = output.entry(output_id).or_default();
                output.push(low_chip);
            }
        }

    }

    if IS_PT2 {
        let output_0 = output.get(&0).ok_or(anyhow!("Output 0 not found"))?;
        let output_1 = output.get(&1).ok_or(anyhow!("Output 1 not found"))?;
        let output_2 = output.get(&2).ok_or(anyhow!("Output 2 not found"))?;
        let result = output_0[0] * output_1[0] * output_2[0];
        println!("Result: {}", result);
    }

    Ok(())
}

pub fn solve_pt1(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve_internal::<false>(data)
}



pub fn solve_pt2(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve_internal::<true>(data)
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_10_input");
        let result = solve_pt1(data);
        println!("{:?}", result);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_10_input");
        let result = solve_pt2(data);
        println!("{:?}", result);
    }

}
