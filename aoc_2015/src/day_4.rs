
pub fn solve(data: &str) {

    let mut i = 0;
    loop {
        let input = format!("{}{}", data, i);
        let digest = md5::compute(input);
        let hex_digest = format!("{:x}", digest);

        if hex_digest.starts_with("000000"){
            break;
        }

        i += 1;
    }   

    println!("{}", i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = "iwrupvqb";
        solve(data);
    }
    
}