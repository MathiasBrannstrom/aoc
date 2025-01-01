// https://adventofcode.com/2015/day/21

const WEAPONS: [(u32, u32); 5] = [
    (8, 4),
    (10, 5),
    (25, 6),
    (40, 7),
    (74, 8),
];

const ARMOR: [(u32, u32); 6] = [
    (0, 0),
    (13, 1),
    (31, 2),
    (53, 3),
    (75, 4),
    (102, 5),
];

const RINGS: [(u32, u32, u32); 8] = [
    (0, 0, 0),
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

struct Character {
    hp: u32,
    damage: u32,
    armor: u32,
}

pub fn solve_pt2(data: &str) {
    let mut lines = data.lines();
    let boss_hp = lines.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
    let boss_damage = lines.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
    let boss_armor = lines.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();

    let boss = Character {
        hp: boss_hp,
        damage: boss_damage,
        armor: boss_armor,
    };

    let mut max_cost = 0;

    for weapon in WEAPONS.iter() {
        for armor in ARMOR.iter() {
            for ring1 in RINGS.iter() {
                for ring2 in RINGS.iter() {
                    if ring1 == ring2 {
                        continue;
                    }

                    

                    let player = Character {
                        hp: 100,
                        damage: weapon.1 + ring1.1 + ring2.1,
                        armor: armor.1 + ring1.2 + ring2.2,
                    };

                    if !battle_result(&player, &boss) {
                        let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                        if cost > max_cost {
                            max_cost = cost;
                        }
                    }
                }
            }
        }
    }

    println!("Max cost: {}", max_cost);
}

fn battle_result(player: &Character, boss: &Character) -> bool {
    let damage_to_boss = (player.damage as i32 - boss.armor as i32).max(1) as u32;
    let damage_to_player = (boss.damage as i32 - player.armor as i32).max(1) as u32;
    let player_turns_needed = boss.hp.div_ceil(damage_to_boss);
    let boss_turns_needed = player.hp.div_ceil(damage_to_player);

    player_turns_needed <= boss_turns_needed
}

pub fn solve_pt1(data: &str) {
    let mut lines = data.lines();
    let boss_hp = lines.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
    let boss_damage = lines.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
    let boss_armor = lines.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();

    let boss = Character {
        hp: boss_hp,
        damage: boss_damage,
        armor: boss_armor,
    };

    let mut min_cost = u32::MAX;

    for weapon in WEAPONS.iter() {
        for armor in ARMOR.iter() {
            for ring1 in RINGS.iter() {
                for ring2 in RINGS.iter() {
                    if ring1 == ring2 {
                        continue;
                    }

                    

                    let player = Character {
                        hp: 100,
                        damage: weapon.1 + ring1.1 + ring2.1,
                        armor: armor.1 + ring1.2 + ring2.2,
                    };

                    if battle_result(&player, &boss) {
                        let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                        if cost < min_cost {
                            min_cost = cost;
                        }
                    }
                }
            }
        }
    }

    println!("Min cost: {}", min_cost);
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_21_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_21_input");
        solve_pt2(data);
    }

}
