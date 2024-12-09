use crate::custom_error::AocError;

use std::collections::HashMap;

fn expand(input: &str) -> HashMap<usize, usize> {
    let mut d: HashMap<usize, usize> = HashMap::new();
    let mut current_position = 0;
    let mut is_file = true;
    let mut file_id = 0;

    input.chars()
        .filter_map(|c| c.to_digit(10))
        .for_each(|length| {
            let length = length as usize;
            
            if is_file {
                // Populate with file locations
                for offset in 0..length {
                    d.insert(current_position + offset, file_id);
                }
                file_id += 1;
            } else {
                is_file = !is_file;
            }
            // Update position and toggle state
            current_position += length;
        });

    d
}

fn rearrange (mut exp: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let max_pos = exp.keys().max().unwrap_or(&0) + 1;
    
    for current_pos in 0..max_pos {
        // If current position has a file
        if let Some(&current_file) = exp.get(&current_pos) {
            // Look ahead for a gap (position without an entry in the map)
            if let Some(next_gap) = (current_pos + 1..max_pos)
                .find(|pos| !exp.contains_key(pos)) 
            {
                // Move file to gap
                exp.remove(&current_pos);
                exp.insert(next_gap, current_file);
            }
        }
    }
    
    exp
}

fn checksum(exp: &HashMap<usize, usize>) -> u128 {
    exp.iter()
        .map(|(loc, file_id)| (*loc as u128) * (*file_id as u128))
        .sum()
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut expanded = expand(input);
    expanded = rearrange(expanded);
    Ok(checksum(&expanded).to_string())
}

// fn expand(input: &str) -> HashMap<usize, usize> {
//     let mut disk = HashMap::new();
//     let mut next_pos = 0;
//     // a
//     let mut is_file = true;  
//     // b
//     let mut file_id = 0;     

//     for c in input.chars() {
//         if let Some(length) = c.to_digit(10) {
//             let length = length as usize;
            
//             if is_file {
//                 // Instead of repeating a string, store actual positions
//                 for i in 0..length {
//                     disk.insert(next_pos + i, file_id);
//                 }
//                 file_id += 1;
//             }
//             // if not file (is_file = false), we just skip those positions
            
//             is_file = !is_file;
//             next_pos += length;
//         }
//     }
//     disk
// }

// // fn rearrange(disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
// //     let max_pos = *disk.keys().max().unwrap_or(&0);
    
// //     // Convert to vec for easier manipulation
// //     let mut disk_vec: Vec<char> = (0..=max_pos)
// //         .map(|i| {
// //             if let Some(&id) = disk.get(&i) {
// //                 char::from_digit(id as u32, 10).unwrap()
// //             } else {
// //                 '.'
// //             }
// //         })
// //         .collect();

// //     println!("Initial state: {}", disk_vec.iter().collect::<String>());
    
// //     // Keep moving until no dots remain
// //     while disk_vec.contains(&'.') {
// //         // Find rightmost non-dot position
// //         let last_digit_pos = disk_vec.iter()
// //             .position(|&c| c != '.')
// //             .unwrap_or(0);
        
// //         // Find leftmost dot
// //         if let Some(first_dot_pos) = disk_vec.iter().position(|&c| c == '.') {
// //             // Get the digit to move
// //             let digit = disk_vec[last_digit_pos];
// //             // Remove the digit (replace with dot)
// //             disk_vec[last_digit_pos] = '.';
// //             // Replace dot with digit
// //             disk_vec[first_dot_pos] = digit;
// //             println!("After move: {}", disk_vec.iter().collect::<String>());
// //         }
// //     }

// //     // Convert back to HashMap
// //     disk_vec.iter()
// //         .enumerate()
// //         .filter(|(_, &c)| c != '.')
// //         .map(|(pos, c)| (pos, c.to_digit(10).unwrap() as usize))
// //         .collect()
// // }

// fn rearrange(disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
//     let max_pos = *disk.keys().max().unwrap_or(&0);
    
//     // Convert to vec for easier manipulation
//     let mut disk_vec: Vec<String> = (0..=max_pos)
//         .map(|i| {
//             if let Some(&id) = disk.get(&i) {
//                 id.to_string()
//             } else {
//                 ".".to_string()
//             }
//         })
//         .collect();

//     // println!("Initial state: {}", disk_vec.join(""));
    
//     // Keep moving until no dots remain
//     while disk_vec.contains(&".".to_string()) {
//         // Find rightmost non-dot position
//         let last_digit_pos = disk_vec.iter()
//             .rposition(|s| s != ".")
//             .unwrap_or(0);
        
//         // Find leftmost dot
//         if let Some(first_dot_pos) = disk_vec.iter().position(|s| s == ".") {
//             // Get the digit to move
//             let digit = disk_vec[last_digit_pos].clone();
//             // Remove the digit (replace with dot)
//             disk_vec[last_digit_pos] = ".".to_string();
//             // Replace dot with digit
//             disk_vec[first_dot_pos] = digit;
//             // println!("After move: {}", disk_vec.join(""));
//         }
//     }

//     // Convert back to HashMap
//     disk_vec.iter()
//         .enumerate()
//         .filter(|(_, s)| *s != ".")
//         .map(|(pos, s)| (pos, s.parse::<usize>().unwrap()))
//         .collect()
// }

// // /// 2024
// // fn rearrange(disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
// //     let max_pos = *disk.keys().max().unwrap_or(&0);
    
// //     // Convert to vec for easier manipulation
// //     let mut disk_vec: Vec<char> = (0..=max_pos)
// //         .map(|i| {
// //             if let Some(&id) = disk.get(&i) {
// //                 char::from_digit(id as u32, 10).unwrap()
// //             } else {
// //                 '.'
// //             }
// //         })
// //         .collect();

// //     println!("Initial state: {}", disk_vec.iter().collect::<String>());
    
// //     // Keep moving until no dots remain
// //     while disk_vec.contains(&'.') {
// //         // Find rightmost non-dot position
// //         let last_digit_pos = disk_vec.iter()
// //             .position(|c| *c != '.')
// //             .unwrap_or(0);
        
// //         // Find leftmost dot
// //         if let Some(first_dot_pos) = disk_vec.iter().position(|c| *c == '.') {
// //             // Get the digit to move
// //             let digit = disk_vec[last_digit_pos];
// //             // Remove the digit (replace with dot)
// //             disk_vec[last_digit_pos] = '.';
// //             // Replace dot with digit
// //             disk_vec[first_dot_pos] = digit;
// //             println!("After move: {}", disk_vec.iter().collect::<String>());
// //         }
// //     }

// //     // Convert back to HashMap
// //     disk_vec.iter()
// //         .enumerate()
// //         .filter(|(_, c)| **c != '.')
// //         .map(|(pos, c)| (pos, c.to_digit(10).unwrap() as usize))
// //         .collect()
// // }

// // fn rearrange(disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
// //     let max_pos = *disk.keys().max().unwrap_or(&0);
    
// //     // Convert to string representation 
// //     let mut disk_str: String = (0..=max_pos)
// //         .map(|i| {
// //             if let Some(&id) = disk.get(&i) {
// //                 id.to_string()
// //             } else {
// //                 ".".to_string()
// //             }
// //         })
// //         .collect();

// //     println!("Initial state: {}", disk_str);
    
// //     // Keep moving until no dots remain
// //     while disk_str.contains('.') {
// //         // Find rightmost non-dot position
// //         if let Some(last_digit_pos) = disk_str.chars()
// //             .enumerate()
// //             .rev()
// //             .find(|(_, c)| *c != '.')
// //             .map(|(i, _)| i) {
                
// //             // Find leftmost dot
// //             if let Some(first_dot_pos) = disk_str.find('.') {
// //                 // Get the digit to move
// //                 let digit = disk_str.remove(last_digit_pos);
// //                 // Remove the dot
// //                 disk_str.remove(first_dot_pos);
// //                 // Insert digit where dot was
// //                 disk_str.insert(first_dot_pos, digit);
// //                 println!("After move: {}", disk_str);
// //             }
// //         }
// //     }

// //     // Convert back to HashMap
// //     disk_str.char_indices()
// //         .map(|(pos, c)| (pos, c.to_digit(10).unwrap() as usize))
// //         .collect()
// // }

// // Calculate checksum directly from positions and file IDs
// fn calculate_checksum(disk: &HashMap<usize, usize>) -> u64 {
//     disk.iter()
//         .map(|(pos, file_id)| (*pos as u64) * (*file_id as u64))
//         .sum()
// }

// #[tracing::instrument]
// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let disk = expand(input);
//     let rearranged = rearrange(disk);
//     Ok(calculate_checksum(&rearranged).to_string())
// }
















// // // fn rearrange(mut s: String, target: char) -> String {
// // //     if let Some(c) = s.pop() {
// // //         if let Some(pos) = s.find(target) {
// // //             s.remove(pos);
// // //             s.insert(pos, c);
// // //         }
// // //     }
// // //     s
// // // }

// // fn rearrange(mut disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
// //     let mut result = HashMap::new();
// //     let mut next_free_pos = 0;
    
// //     // Find the rightmost position
// //     if let Some(max_pos) = disk.keys().max().cloned() {
// //         // Work from right to left
// //         for pos in (0..=max_pos).rev() {
// //             if let Some(file_id) = disk.remove(&pos) {
// //                 // Found a file, put it in the next available position
// //                 result.insert(next_free_pos, file_id);
// //                 next_free_pos += 1;
// //             }
// //         }
// //     }
    
// //     result
// // }

// // fn rearrange(mut disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
// //     let mut result = HashMap::new();
// //     let max_file_id = *disk.values().max().unwrap_or(&0);
// //     let mut next_pos = 0;

// //     // Place files in order from highest ID to lowest
// //     for file_id in (0..=max_file_id).rev() {
// //         // Count how many positions this file ID occupies
// //         let file_positions = disk.iter()
// //             .filter(|(_, &id)| id == file_id)
// //             .count();
        
// //         // Place all positions for this file ID contiguously
// //         for i in 0..file_positions {
// //             result.insert(next_pos + i, file_id);
// //         }
// //         next_pos += file_positions;
// //     }
    
// //     result
// // }

// // /// 2024 Edition
// // fn rearrange(mut disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
// //     let mut result = HashMap::new();
// //     let max_file_id = *disk.values().max().unwrap_or(&0);
// //     let mut next_pos = 0;

// //     // Place files in order from highest ID to lowest
// //     for file_id in (0..=max_file_id).rev() {
// //         // Count how many positions this file ID occupies
// //         let file_positions = disk.iter()
// //             .filter(|(_, id)| **id == file_id)  // Fixed pattern matching
// //             .count();
        
// //         // Place all positions for this file ID contiguously
// //         for i in 0..file_positions {
// //             result.insert(next_pos + i, file_id);
// //         }
// //         next_pos += file_positions;
// //     }
    
// //     result
// // }

// // fn rearrange(mut disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
// //     let mut result = HashMap::new();
// //     let mut left = 0;
// //     let max_pos = *disk.keys().max().unwrap_or(&0);
    
// //     for right in (0..=max_pos).rev() {
// //         if let Some(file_id) = disk.get(&right) {
// //             result.insert(left, *file_id);
// //             left += 1;
// //         }
// //     }
    
// //     result
// // }

// // fn rearrange(mut disk: HashMap<usize, usize>) -> HashMap<usize, usize> {
// //     // Convert to string representation for easier manipulation
// //     let max_pos = *disk.keys().max().unwrap_or(&0);
// //     let mut disk_str: String = (0..=max_pos)
// //         .map(|i| match disk.get(&i) {
// //             Some(&file_id) => file_id.to_string(),
// //             None => ".".to_string()
// //         })
// //         .collect();

// //     // Keep moving characters until no dots remain
// //     while let Some(dot_pos) = disk_str.find('.') {
// //         if let Some(last_char) = disk_str.chars().last() {
// //             if last_char != '.' {
// //                 // Remove last character
// //                 let last_char = disk_str.pop().unwrap();
// //                 // Remove the dot
// //                 disk_str.remove(dot_pos);
// //                 // Insert the character where the dot was
// //                 disk_str.insert(dot_pos, last_char);
// //             } else {
// //                 // If last character is a dot, remove it
// //                 disk_str.pop();
// //             }
// //         }
// //     }

// //     // Convert back to HashMap
// //     disk_str.char_indices()
// //         .filter(|(_, c)| c != &'.')
// //         .map(|(pos, c)| (pos, c.to_digit(10).unwrap() as usize))
// //         .collect()
// // }


// // // fn checksum(input: &str) -> u64 {
// // //     let mut x = 0;
// // //     input.chars().skip(1)
// // //         // .inspect(|c| {
// // //         //     dbg!(c);
// // //         // })
// // //         .map(|c|{
// // //             let c = c.to_digit(10).unwrap() as u64;
// // //             x += 1;
            
// // //             c * x
// // //         }).sum()
// // // }

// // // fn checksum(input: &str) -> u64 {
// // //     input.chars().enumerate().fold(0, |acc, (pos, c)| {
// // //         match c.to_digit(10) {
// // //             Some(digit) => acc + (pos as u64 * digit as u64),
// // //             None => acc  // Skip '.' characters
// // //         }
// // //     })
// // // }

// // // fn checksum(input: &str) -> u128 {
// // //     input.char_indices()  // Get both index and char
// // //         .filter(|(_, c)| c.is_digit(10))  // Only process digits
// // //         .map(|(pos, c)| {
// // //             let digit = c.to_digit(10).unwrap() as u128;
// // //             pos as u128 * digit
// // //         })
// // //         .sum()
// // // }

fn _process_disk(input: &str) -> u128 {
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

// // #[tracing::instrument]
// // pub fn process(input: &str) -> miette::Result<String, AocError> {
// //     // Ok(process_disk(input).to_string())

// //     let mut expanded = dbg!(expand(input));
// //     // let mut expanded = expand(input);


// //     // // while let Some(_) = &expanded[..].find('.') {
// //         expanded = dbg!(rearrange(expanded));
// //         // expanded = rearrange(expanded);
// //     // // }

// //     // expanded = rearrange_loop(expanded);

// //     // panic!("halt");
    
// //     // // dbg!(checksum(&expanded));
// //     // // panic!("halt");

// //     // // Ok(expanded)

// //     // Ok(checksum(&expanded).to_string())

// //     let checksum: u64 = expanded.iter()
// //         .map(|(pos, file_id)| (*pos as u64) * (*file_id as u64))
// //         .sum();

// //     Ok(checksum.to_string())
// // }

// pub fn _process(input: &str) -> miette::Result<String, AocError> {
//     // Print initial state in readable format
//     let expanded = expand(input);
//     println!("\nExpanded disk by position:");
//     let max_pos = *expanded.keys().max().unwrap_or(&0);
//     for i in 0..=max_pos {
//         if let Some(&file_id) = expanded.get(&i) {
//             println!("pos {}: file {}", i, file_id);
//         } else {
//             println!("pos {}: empty", i);
//         }
//     }

//     let rearranged = rearrange(expanded);
//     println!("\nRearranged disk by position:");
//     let max_pos = *rearranged.keys().max().unwrap_or(&0);
//     for i in 0..=max_pos {
//         if let Some(&file_id) = rearranged.get(&i) {
//             println!("pos {}: file {}", i, file_id);
//         }
//     }

//     let checksum: u64 = rearranged.iter()
//         .map(|(pos, file_id)| {
//             let product = (*pos as u64) * (*file_id as u64);
//             println!("pos {} * file {} = {}", pos, file_id, product);
//             product
//         })
//         .sum();

//     println!("\nChecksum: {}", checksum);
//     Ok(checksum.to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("00...111...2...333.44.5555.6666.777.888899", "0099811188827773336446555566")]
    // fn test_rearrange(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(rearrange_loop(input.to_string()), expected);
    // }

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
