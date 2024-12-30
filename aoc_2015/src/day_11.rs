// use itertools::Itertools;

// Ended up solving this on paper instead, turned out that it was quite easy to construct the next one by avoiding modifying as many
// left digits as possible. 

// #[cfg(test)]
// mod tests{

//     use super::solve;

//     #[test]
//     fn test_solve(){
//         let data = include_str!("day_11_input");
//         solve(data);
//     }
// }

// pub fn solve(data: &str) {

//     let mut password = data.to_string();
//     loop {
//         get_next_password(&mut password);
        
//         if is_password_allowed(&password) {
//             break;
//         }
//     }

//     println!("{}", password);
// }

// fn is_password_allowed(password: &str) -> bool {
//     println!("Testing: {}", password);
//     let mut straight_count = 0;
//     let mut last_char = '1';
//     let mut found_straight = false;
//     let mut found_two_pairs = false;
//     let mut pos_of_pair:Option<usize> = None;
//     let mut i = 0;
//     for char in password.chars() {

//         if char as i16 - last_char as i16 == 1 {
//             straight_count += 1;
//             if straight_count == 2 {
//                 found_straight = true;
//             }
//         } else {
//             straight_count = 0;
//         }

//         if char == last_char {
//             if let Some(x) = pos_of_pair {
//                 if x > i+1 {
//                     found_two_pairs = true;
//                 }
//             }
//             else {
//                 pos_of_pair = Some(i);
//             }

//         }

//         last_char = char;
//         i += 1;
//     }

//     found_straight && found_two_pairs
// }

// fn get_next_password(password: &mut str) {
//     // loop {

//     //     let any_pair_in_first_six = password[..6].chars()
//     //         .tuple_windows()
//     //         .any(|(l,r)| l == r);

//     //     if any_pair_in_first_six {
//     //         return;
//     //     }
        
//     //     increase_letter_at_pos(password, 5);
//     // }
    

//     increase_letter_at_pos(password, password.len()-1);
// }

// fn increase_letter_at_pos(password: &mut str, pos: usize) {
//     let bytes = unsafe { password.as_bytes_mut() };
//     let mut increase_next_pos = false;

//     if pos < bytes.len() {
//         bytes[pos] = match bytes[pos] {
//             b'a'..b'h' => bytes[pos] + 1,
//             b'h' => b'j',
//             b'j' => b'k',
//             b'k' => b'm',
//             b'm'=> b'n',
//             b'n' => b'p',
//             b'p'..b'z' => bytes[pos] + 1,
//             b'z' => {
//                 increase_next_pos = true;
//                 b'a'
//             },
//             _ => panic!(),
//         };
//     }

//     if increase_next_pos {
//         increase_letter_at_pos(password, pos-1);
//     }
// }