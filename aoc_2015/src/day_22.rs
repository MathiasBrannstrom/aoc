// https://adventofcode.com/2015/day/22

#[derive(Debug, Clone, Copy)]
struct Boss {
    hp: u32,
    damage: u32,
    poison_timer: u32,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    hp: u32,
    mana: u32,
    shield_timer: u32,
    recharge_timer: u32,
    total_mana_spent: u32,
}

#[derive(Debug, Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> u32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn iter() -> impl Iterator<Item = Spell> {
        vec![Spell::MagicMissile, Spell::Drain, Spell::Shield, Spell::Poison, Spell::Recharge].into_iter()
    }
}

#[derive(Debug, Clone, Copy)]
struct Evaluation {
    player: Player,
    boss: Boss,
    next_spell: Spell,
}

pub fn solve_pt1(data: &str) {
    solve_internal::<false>(data);
}

pub fn solve_pt2(data: &str) {
    solve_internal::<true>(data);
}

fn solve_internal<const IS_PT2: bool>(data: &str) {
    let mut lines = data.lines();
    let boss_hp = lines.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
    let boss_damage = lines.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();

    let boss = Boss {
        hp: boss_hp,
        damage: boss_damage,
        poison_timer: 0,
    };

    let player = Player {
        hp: 50,
        mana: 500,
        total_mana_spent: 0,
        shield_timer: 0,
        recharge_timer: 0,
    };

    let mut evaluations = vec![];
    for spell in Spell::iter() {
        evaluations.push(Evaluation{player, boss, next_spell: spell});
    }

    let mut min_mana_spent = u32::MAX;

    while let Some(evaluation) = evaluations.pop() {
        let mut new_player = evaluation.player;
        let mut new_boss = evaluation.boss;
    
        // Player turn
        if IS_PT2 {
            if new_player.hp == 1 {
                continue;
            }
            new_player.hp -= 1;
        }
        
        if new_player.shield_timer > 0 {
            new_player.shield_timer -= 1;
        }
        if new_player.recharge_timer > 0 {
            new_player.recharge_timer -= 1;
            new_player.mana += 101;
        }
        if new_boss.poison_timer > 0 {
            new_boss.poison_timer -= 1;
            if new_boss.hp <= 3 {
                if new_player.total_mana_spent < min_mana_spent {
                    min_mana_spent = new_player.total_mana_spent;
                }
                continue;
            }
            new_boss.hp -= 3;
        }

        
        
        let mana_cost = evaluation.next_spell.cost();

        if mana_cost > new_player.mana {
            continue;
        }

        new_player.mana -= mana_cost;
        new_player.total_mana_spent += mana_cost;

        if new_player.total_mana_spent >= min_mana_spent {
            continue;
        }

        let damage = match evaluation.next_spell {
            Spell::MagicMissile => 4,
            Spell::Drain => 2,
            _ => 0,
        };
        match evaluation.next_spell {
            Spell::MagicMissile => (),
            Spell::Drain => {
                new_player.hp += 2;
            },
            Spell::Shield => new_player.shield_timer = 6,
            Spell::Poison => new_boss.poison_timer = 6,
            Spell::Recharge => new_player.recharge_timer = 5,
        }

        if damage >= new_boss.hp {
            if new_player.total_mana_spent < min_mana_spent {
                min_mana_spent = new_player.total_mana_spent;
            }
            continue;
        }

        new_boss.hp -= damage;
        
        
        // Boss turn
        if new_player.shield_timer > 0 {
            new_player.shield_timer -= 1;
        }
        if new_player.recharge_timer > 0 {
            new_player.recharge_timer -= 1;
            new_player.mana += 101;
        }
        if new_boss.poison_timer > 0 {
            new_boss.poison_timer -= 1;
            if new_boss.hp <= 3 {
                if new_player.total_mana_spent < min_mana_spent {
                    min_mana_spent = new_player.total_mana_spent;
                }
                continue;
            }
            new_boss.hp -= 3;
        }

        let boss_damage = if new_player.shield_timer > 0 { new_boss.damage - 7 } else { new_boss.damage };

        if boss_damage >= new_player.hp { continue; }

        new_player.hp -= boss_damage;

        for spell in Spell::iter() {
            if match spell {
                Spell::Shield => new_player.shield_timer > 1,
                Spell::Poison => new_boss.poison_timer > 1,
                Spell::Recharge => new_player.recharge_timer > 1,
                _ => false,
            } {
                continue;
            }

            evaluations.push(Evaluation{player: new_player, boss: new_boss, next_spell: spell});
        }
        evaluations.sort_unstable_by(|a, b| a.boss.hp.cmp(&b.boss.hp).reverse());
    }

    if min_mana_spent == u32::MAX {
        println!("No solution found");
    } else {
        println!("Minimum mana spent: {}", min_mana_spent);
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_22_input");
        solve_pt1(data);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_22_input");
        solve_pt2(data);
    }

}
