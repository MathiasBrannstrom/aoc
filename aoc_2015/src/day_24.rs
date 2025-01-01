// https://adventofcode.com/2015/day/24

use std::cmp;


pub fn solve_pt1(data: &str) {
    let mut weights = data.lines().map(|l| l.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let total_weight: u32 = weights.iter().sum();
    let target_weight = total_weight / 3;   

    println!("{:?}", target_weight);

    weights.sort_unstable_by(|a, b| a.cmp(b).reverse());
    let weights = weights;

    let mut masks = masks_that_give_target_weight(target_weight, &weights);

    masks.sort_unstable_by(|a, b| mask_cmp(&weights, *a, *b));    

    let masks = masks;

    println!("Masks: {}", masks.len());

    for (a, mask_a) in masks.iter().enumerate() {
        for (b, mask_b) in masks[a+1..].iter().enumerate() {
            if mask_a & mask_b != 0 {
                continue;
            }

            for mask_c in masks[a+b+1..].iter() {
                if mask_a & mask_c != 0 || mask_b & mask_c != 0 {
                    continue;
                }

                println!("Quantum Entanglement: {}", quantum_entanglement(&weights, *mask_a));
                return;
            }
        }
    }

}


pub fn solve_pt2(data: &str) {
    let mut weights = data.lines().map(|l| l.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let total_weight: u32 = weights.iter().sum();
    let target_weight = total_weight / 4;   

    println!("{:?}", target_weight);

    weights.sort_unstable_by(|a, b| a.cmp(b).reverse());
    let weights = weights;

    let mut masks = masks_that_give_target_weight(target_weight, &weights);

    masks.sort_unstable_by(|a, b| mask_cmp(&weights, *a, *b));    

    let masks = masks;

    //print amount of masks
    println!("Masks: {}", masks.len());

    for (a, mask_a) in masks.iter().enumerate() {
        for (b, mask_b) in masks[a+1..].iter().enumerate() {
            if mask_a & mask_b != 0 {
                continue;
            }

            for (c,mask_c) in masks[a+b+1..].iter().enumerate() {
                if mask_a & mask_c != 0 || mask_b & mask_c != 0 {
                    continue;
                }
                for mask_d in masks[a+b+c+1..].iter() {
                    if mask_a & mask_d != 0 || mask_b & mask_d != 0 || mask_c & mask_d != 0 {
                        continue;
                    }

                    println!("Quantum Entanglement: {}", quantum_entanglement(&weights, *mask_a));
                    return;
                }
            }
        }
    }

    
}


fn mask_cmp(weights: &[u32], mask_a: u32, mask_b: u32) -> cmp::Ordering {
    mask_a.count_ones().cmp(&mask_b.count_ones())
        .then_with(|| quantum_entanglement(weights, mask_a).cmp(&quantum_entanglement(weights, mask_b)))
}

fn quantum_entanglement(weights: &[u32], mask: u32) -> u64 {
    (1..=weights.len())
    .filter(|i| is_bit_set(mask, *i))
    .map(|i| weights[weights.len() - i] as u64)
    .fold(1u64, |acc, x| acc.checked_mul(x).unwrap_or(u64::MAX))
    
}

fn masks_that_give_target_weight(target: u32, weights: &[u32]) -> Vec<u32> {

    if weights.is_empty() {
        return Vec::new();
    }

    let mut masks = Vec::new();

    let weight = weights[0];

    let sub_masks_not_including_weight = masks_that_give_target_weight(target, &weights[1..]);
    masks.extend(sub_masks_not_including_weight.iter());

    if weight == target {
        masks.push(1 << weights.len());
    }

    if target > weight {
        let sub_masks_including_weight = masks_that_give_target_weight(target-weight, &weights[1..]);

        masks.extend(sub_masks_including_weight.iter().map(|mask| mask | 1 << weights.len()));
    }
    
    masks
}


#[inline]
fn is_bit_set(mask: u32, bit: usize) -> bool {
    mask & (1 << bit) != 0
}

#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_24_input");
        solve_pt1(data);
    }

    #[test]
    fn debug_solve_pt1() {
        let data = "1\n2\n3\n4\n5\n6";
        solve_pt1(data);
    }


    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_24_input");
        solve_pt2(data);
    }
}
