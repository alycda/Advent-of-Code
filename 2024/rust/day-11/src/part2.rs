use crate::custom_error::AocError;

use std::collections::HashMap;

fn blink(input: &str, target_iterations: usize) -> String {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut current = input.to_string();
    let mut iteration = 0;
    
    while iteration < target_iterations {
        // If we've seen this state before, we found a cycle
        if let Some(prev_iteration) = seen.get(&current) {
            let cycle_length = iteration - prev_iteration;
            let remaining_iterations = (target_iterations - iteration) % cycle_length;
            
            // Fast forward through remaining iterations
            for _ in 0..remaining_iterations {
                current = single_blink(&current);
            }
            return current;
        }
        
        seen.insert(current.clone(), iteration);
        current = single_blink(&current);
        iteration += 1;
    }
    
    current
}

// Move your transformation logic to a separate function
fn single_blink(input: &str) -> String {
    input.split_whitespace()
        .flat_map(|stone| {
            if stone == "0" {
                vec![1]
            } else if stone.len() % 2 == 0 {
                let (left, right) = stone.split_at(stone.len() / 2);
                vec![
                    left.parse().unwrap(),
                    right.parse().unwrap()
                ]
            } else {
                vec![stone.parse::<usize>().unwrap() * 2024]
            }
        })
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(blink(input, 75))
}

// #[tracing::instrument]
// pub fn process(input: &str, times: usize) -> miette::Result<String, AocError> {
//     // let numbers: Vec<usize> = input
//     //     .split_whitespace()
//     //     .flat_map(|stone| {
//     //         if stone == "0" {
//     //             vec![1]
//     //         } else if stone.len() % 2 == 0 {
//     //             // For even length strings, split and parse both halves
//     //             let (left, right) = stone.split_at(stone.len() / 2);
//     //             vec![
//     //                 left.parse().unwrap_or(0),
//     //                 right.parse().unwrap_or(0)
//     //             ]
//     //         } else {
//     //             // For odd length, multiply by 2024
//     //             vec![stone.parse::<usize>().unwrap_or(0) * 2024]
//     //         }
//     //     })
//     //     .collect();

//     // // implement the recursion
//     // let result = blink(&numbers.iter()
//     //     .map(|n| n.to_string())
//     //     .collect::<Vec<_>>()
//     //     .join(" "), 1);

//     // Ok(result)

//     Ok(blink(input, times).split_whitespace().count().to_string())
// }

// use std::collections::HashMap;

// fn find_cycle(input: &str) -> (usize, usize) {
//     let mut seen: HashMap<String, usize> = HashMap::new();
//     let mut current = input.to_string();
//     let mut iteration = 0;
    
//     while !seen.contains_key(&current) {
//         seen.insert(current.clone(), iteration);
//         current = single_blink(&current); // your transform logic
//         iteration += 1;
//     }
    
//     let cycle_start = seen[&current];
//     let cycle_length = iteration - cycle_start;
    
//     (cycle_start, cycle_length)
// }

// fn blink(input: &str, times: usize) -> String {
//     // Base case - if times is 0, return the input
//     if times == 0 {
//         return input.to_string();
//     }

//     // Process current iteration similar to flat_map
//     let numbers: Vec<usize> = input
//         .split_whitespace()
//         .flat_map(|stone| {
//             if stone == "0" {
//                 vec![1]
//             } else if stone.len() % 2 == 0 {
//                 let (left, right) = stone.split_at(stone.len() / 2);
//                 vec![
//                     left.parse().unwrap(),
//                     right.parse().unwrap()
//                 ]
//             } else {
//                 vec![stone.parse::<usize>().unwrap() * 2024]
//             }
//         })
//         .collect();

//     // Convert back to string and recurse
//     let new_input = numbers.iter()
//         .map(|n| n.to_string())
//         .collect::<Vec<_>>()
//         .join(" ");

//     // Recursive call with times-1
//     blink(&new_input, times - 1)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // use rstest::rstest;

//     // #[rstest]
//     // #[case(("125 17", 1), "253000 1 7")]
//     // #[case(("125 17", 2), "253 0 2024 14168")]
//     // #[case(("125 17", 3), "512072 1 20 24 28676032")]
//     // #[case(("125 17", 4), "512 72 2024 2 0 2 4 2867 6032")]
//     // #[case(("125 17", 5), "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32")]
//     // #[case(("125 17", 6), "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2")]
//     // fn test_cases(#[case] input: (&str, usize), #[case] expected: &str) {
//     //     // assert_eq!(blink(input.0, input.1), expected);
//     //     assert_eq!(expected, process(input.0, input.1).unwrap());
//     // }

//     #[test]
//     fn test_process() -> miette::Result<()> {
//         let input = "125 17";
//         assert_eq!("55312", process(input, 25)?);
//         Ok(())
//     }
// }
