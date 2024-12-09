use crate::custom_error::AocError;

fn expand(input: &str) -> String {
    // true = file, false = free space
    let mut a = false;
    let mut b = 0;

    input.chars()
        .filter_map(|c| match c.to_digit(10) {
            Some(d) => Some(d),
            None => None  // Skips newlines, spaces, etc
        })
        .map(|c| {

            // let c = dbg!(c.to_digit(10).unwrap());
            a = !a; 

            match a {
                true => {
                    let d = b.to_string().repeat(c as usize);
                    b += 1; 
                    // dbg!(d)
                    d
                }
                false => {
                    let d = '.'.to_string().repeat(c as usize);
                    // dbg!(d)
                    d
                }
            }
        })
        
        .collect::<String>()
}

fn rearrange(mut s: String, target: char) -> String {
    if let Some(c) = s.pop() {
        if let Some(pos) = s.find(target) {
            s.remove(pos);
            s.insert(pos, c);
        }
    }
    s
}

// fn checksum(input: &str) -> u64 {
//     let mut x = 0;
//     input.chars().skip(1)
//         // .inspect(|c| {
//         //     dbg!(c);
//         // })
//         .map(|c|{
//             let c = c.to_digit(10).unwrap() as u64;
//             x += 1;
            
//             c * x
//         }).sum()
// }

// fn checksum(input: &str) -> u64 {
//     input.chars().enumerate().fold(0, |acc, (pos, c)| {
//         match c.to_digit(10) {
//             Some(digit) => acc + (pos as u64 * digit as u64),
//             None => acc  // Skip '.' characters
//         }
//     })
// }

fn checksum(input: &str) -> u128 {
    input.char_indices()  // Get both index and char
        .filter(|(_, c)| c.is_digit(10))  // Only process digits
        .map(|(pos, c)| {
            let digit = c.to_digit(10).unwrap() as u128;
            pos as u128 * digit
        })
        .sum()
}

// fn checksum(input: &str) -> u128 {
//     let mut current_num = String::new();
//     let mut position: u128 = 0;
//     let mut sum: u128 = 0;

//     for c in input.chars() {
//         if c.is_digit(10) {
//             current_num.push(c);
//         } else if !current_num.is_empty() {
//             // Found a delimiter after a number
//             if let Ok(num) = current_num.parse::<u128>() {
//                 // println!("Position: {}, Number: {}, Product: {}", position, num, position * num);
//                 sum = sum.saturating_add(position.saturating_mul(num));
//             }
//             current_num.clear();
//             position += 1;
//         } else {
//             // Found a delimiter with no number built up
//             position += 1;
//         }
//     }

//     // Handle last number if string ends with digit
//     if !current_num.is_empty() {
//         if let Ok(num) = current_num.parse::<u128>() {
//             // println!("Final Position: {}, Number: {}, Product: {}", position, num, position * num);
//             sum = sum.saturating_add(position.saturating_mul(num));
//         }
//     }

//     println!("Input: {}", input);
//     println!("Final Sum: {}", sum);
//     sum
// }

// fn checksum(input: &str) -> u128 {
//     // println!("Input to checksum: {}", input);
//     let result = input.chars()
//         .enumerate()
//         .filter_map(|(pos, c)| {
//             if c.is_digit(10) {
//                 let digit = c.to_digit(10).unwrap() as u128;
//                 let product = pos as u128 * digit;
//                 println!("pos: {}, digit: {}, product: {}", pos, digit, product);
//                 Some(product)
//             } else {
//                 None
//             }
//         })
//         .sum();
//     // println!("Final sum: {}", result);
//     result
// }

// fn checksum(input: &str) -> u128 {  // Changed to u128
//     let (final_num, final_pos, sum) = input.chars()
//         .fold((String::new(), 0_u128, 0_u128), |(mut num, pos, sum), c| {
//             if c.is_digit(10) {
//                 num.push(c);
//                 (num, pos, sum)
//             } else {
//                 let contribution = if !num.is_empty() {
//                     match num.parse::<u128>() {
//                         Ok(digit) => pos.saturating_mul(digit), // Use saturating_mul to handle overflow
//                         Err(_) => 0 // Handle parse errors gracefully
//                     }
//                 } else {
//                     0
//                 };
//                 (String::new(), pos.saturating_add(1), sum.saturating_add(contribution))
//             }
//         });

//     // Handle last number if exists
//     if input.chars().last().map_or(false, |c| c.is_digit(10)) {
//         let last_str: String = input.chars()
//             .rev()
//             .take_while(|c| c.is_digit(10))
//             .collect::<Vec<_>>()
//             .into_iter()
//             .rev()
//             .collect();
            
//         match last_str.parse::<u128>() {
//             Ok(last_num) => sum.saturating_add(final_pos.saturating_mul(last_num)),
//             Err(_) => sum
//         }
//     } else {
//         sum
//     }
// }

// fn checksum(input: &str) -> u64 {
//     let sum = input.chars()
//         .fold((String::new(), 0, 0), |(mut num, pos, sum), c| {
//             if c.is_digit(10) {
//                 num.push(c);
//                 (num, pos, sum)
//             } else {
//                 let contribution = if !num.is_empty() {
//                     let digit = num.parse::<u64>().unwrap();
//                     pos * digit
//                 } else {
//                     0
//                 };
//                 (String::new(), pos + 1, sum + contribution)
//             }
//         }).2;
    
//     // Handle last number if string ends with digit
//     if input.chars().last().unwrap().is_digit(10) {
//         let last_num = input.chars()
//             .rev()
//             .take_while(|c| c.is_digit(10))
//             .collect::<String>()
//             .chars()
//             .rev()
//             .collect::<String>()
//             .parse::<u64>()
//             .unwrap();
//         sum + (input.len() - 1) as u64 * last_num
//     } else {
//         sum
//     }
// }

use std::collections::HashMap;


fn process_disk(input: &str) -> u128 {
    let mut disk: HashMap<usize, usize> = HashMap::new();  // Changed to usize
    let mut next_disk_location = 0;
    let mut next_is_fill = true;
    let mut next_file_id = 0;

    // First pass: populate the disk
    for c in input.chars() {
        if let Some(length) = c.to_digit(10) {
            let length = length as usize;
            if next_is_fill {
                // Add file
                for loc in 0..length {
                    disk.insert(next_disk_location + loc, next_file_id);
                }
                next_file_id += 1;
                next_is_fill = false;
            } else {
                next_is_fill = true;
            }
            next_disk_location += length;
        }
    }

    // Compact the disk
    let mut disk_copy = disk.clone();
    let mut left = 0;
    let mut right = *disk_copy.keys().max().unwrap_or(&0);
    
    while left < right {
        if let Some(file_id) = disk_copy.remove(&right) {
            while disk_copy.contains_key(&left) {
                left += 1;
            }
            disk_copy.insert(left, file_id);
        }
        right -= 1;
    }

    // Calculate checksum
    disk_copy.iter()
        .map(|(loc, file_id)| (*loc as u128) * (*file_id as u128))  // Cast to u128 for multiplication
        .sum()
}

fn rearrange_loop(mut expanded: String) -> String {
    while let Some(_) = &expanded[..].find('.') {
        // expanded = dbg!(rearrange(expanded, '.'));
        expanded = rearrange(expanded, '.');
    }

    expanded
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(process_disk(input).to_string())

    // // let mut expanded = dbg!(expand(input));
    // let mut expanded = expand(input);

    // // while let Some(_) = &expanded[..].find('.') {
    // //     // expanded = dbg!(rearrange(expanded, '.'));
    // //     expanded = rearrange(expanded, '.');
    // // }

    // expanded = rearrange_loop(expanded);
    
    // // dbg!(checksum(&expanded));
    // // panic!("halt");

    // // Ok(expanded)

    // Ok(checksum(&expanded).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("12345", "0..111....22222")]
    #[case("90909", "000000000111111111222222222")]
    #[case("2333133121414131402", "00...111...2...333.44.5555.6666.777.888899")]
    fn test_expand(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expand(input), expected);
    }

    #[rstest]
    #[case("00...111...2...333.44.5555.6666.777.888899", "0099811188827773336446555566")]
    fn test_rearrange(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(rearrange_loop(input.to_string()), expected);
    }

    // #[rstest]
    // #[case("00...111...2...333.44.5555.6666.777.888899", 1928)]
    // fn test_checksum(#[case] input: &str, #[case] expected: u64) {
    //     assert_eq!(checksum(rearrange(input)), expected);
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
