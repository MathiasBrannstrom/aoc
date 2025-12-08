pub fn solve_pt1(data: &str) {
    let split:Vec<&str> = data.split("\r\n\r\n").collect();
    let ranges:Vec<(u64, u64)> = split[0].lines().map(|l|{
        let mut i = l.split('-');
        println!("{}", l);
        (i.next().unwrap().parse::<u64>().unwrap(), i.next().unwrap().parse::<u64>().unwrap())
    }).collect();

    let ids = split[1].lines().map(|l| {
        l.parse::<u64>().unwrap()
    });

    let mut fresh_count = 0;
    
    for id in ids {
        for range in ranges.iter() {
            if id >= range.0 && id <= range.1 {
                fresh_count += 1;
                break;
            }
        }
    }

    println!("{}", fresh_count);
}

pub fn solve_pt2(data: &str) {
    let split:Vec<&str> = data.split("\r\n\r\n").collect();
    let ranges:Vec<(u64, u64)> = split[0].lines().map(|l|{
        let mut i = l.split('-');
        println!("{}", l);
        (i.next().unwrap().parse::<u64>().unwrap(), i.next().unwrap().parse::<u64>().unwrap())
    }).collect();

    let mut disjunct_ranges:Vec<(u64, u64)> = vec![];

    let mut ranges_to_process:Vec<(u64, u64)> = ranges;

    while !ranges_to_process.is_empty() {
        let new_range = ranges_to_process.pop().unwrap();
        let mut range_index_to_merge = None;
        for (i, range) in disjunct_ranges.iter().enumerate() {
            //Check if overlap
            if does_ranges_overlap(&new_range, &range) {
                range_index_to_merge = Some(i);
                break;
            }
        }

        if let Some(idx) = range_index_to_merge {
            let range_to_merge = disjunct_ranges.remove(idx);
            let range_to_add = merge_ranges(range_to_merge, new_range);
            ranges_to_process.push(range_to_add);
        } else {
            disjunct_ranges.push(new_range);
        }
    } 

    //Count the amount of fresh IDs by max-min for each disjunct range.
    let mut fresh_count = 0;
    for range in disjunct_ranges {
        fresh_count += range.1 - range.0 + 1;
    }

    println!("{}", fresh_count)
}

fn merge_ranges(r0: (u64, u64), r1: (u64, u64)) -> (u64, u64) {
    (r0.0.min(r1.0), r0.1.max(r1.1))
}

fn does_ranges_overlap(r0: &(u64, u64), r1: &(u64, u64)) -> bool {
    !(r1.1 < r0.0 || r1.0 > r0.1)
}

#[cfg(test)]
mod tests {

    use super::{solve_pt1, solve_pt2};

    #[test]
    pub fn test_pt1() {
        let data = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        solve_pt1(data);
    }

    
    #[test]
    pub fn test_pt2() {
        let data = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        solve_pt2(data);
    }
}