// https://adventofcode.com/2016/day/4

use std::collections::HashMap;

use itertools::Itertools;

pub fn solve_pt1(data: &str) {
    let re = regex::Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();

    let sector_id_sum_of_real_rooms = data.lines()
    .map(|line| {
        let (name, sector_id, checksum) = re.captures(line).unwrap().iter()
        .skip(1)
        .map(|m| m.map(|m| m.as_str()))
        .collect_tuple()
        .map(|(name, sector_id, checksum)| (name.unwrap(), sector_id.unwrap().parse::<u32>().unwrap(), checksum.unwrap()))
        .unwrap();

        let mut char_count:HashMap<char, u32> = HashMap::new();
        
        for c in name.chars() {
            if c != '-' {
                *char_count.entry(c).or_insert(0) += 1;
            }
        }

        let mut char_count:Vec<(char, u32)> = char_count.into_iter().collect();
        char_count.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        for (i, (c, _)) in char_count.iter().enumerate().take(5) {
            if checksum.chars().nth(i).unwrap() != *c {
                return (false, sector_id);
            }
        }
        (true, sector_id)
    })
    .filter(|(is_real, _)| *is_real)
    .map(|(_, sector_id)| sector_id)
    .sum::<u32>();

    println!("Sum of sector IDs of real rooms: {}", sector_id_sum_of_real_rooms);

}

pub fn solve_pt2(data: &str) {
    let re = regex::Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();

    let names_real_rooms:Vec<(String, u32)> = data.lines()
    .map(|line| {
        let (name, sector_id, checksum) = re.captures(line).unwrap().iter()
        .skip(1)
        .map(|m| m.map(|m| m.as_str()))
        .collect_tuple()
        .map(|(name, sector_id, checksum)| (name.unwrap(), sector_id.unwrap().parse::<u32>().unwrap(), checksum.unwrap()))
        .unwrap();

        let mut char_count:HashMap<char, u32> = HashMap::new();
        
        for c in name.chars() {
            if c != '-' {
                *char_count.entry(c).or_insert(0) += 1;
            }
        }

        let mut char_count:Vec<(char, u32)> = char_count.into_iter().collect();
        char_count.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        
        for (i, (c, _)) in char_count.iter().enumerate().take(5) {
            if checksum.chars().nth(i).unwrap() != *c {
                return (false, name.to_string(), sector_id);
            }
        }

        let decrypted_name = name.chars()
        .map(|c| {
            if c == '-' {
                ' '
            } else {
                let c = c as u32 - 'a' as u32;
                let c = (c + sector_id) % 26;
                (c + 'a' as u32) as u8 as char
            }
        });
        (true, decrypted_name.collect::<String>(), sector_id)
    })
    .filter(|(is_real, _, _)| *is_real)
    .map(|(_, name, sector_id)| (name, sector_id))
    .collect();

    for room in names_real_rooms {
        if room.0.contains("north") {
            println!("{}  {}", room.0, room.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_04_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_04_input");
        solve_pt2(data);
    }

}
