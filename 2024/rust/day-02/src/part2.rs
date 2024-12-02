//! tolerate a single bad level
//!

use std::collections::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: usize = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            // dbg!(line);
            let mut peekable = line.split_whitespace().peekable();
            let nums = line
                .split_whitespace()
                .take(line.split_whitespace().count() - 1)
                .map(|c| {
                    let c = c.parse::<i32>().unwrap();
                    peekable.next();
                    let next = peekable.peek().unwrap_or(&"0");
                    let next = next.parse::<i32>().unwrap();
                    // dbg!(c, next);
                    // // c.parse::<u16>().unwrap()
                    // c
                    match (c, next) {
                        (a, b) if a > b => ('-', (a - b).abs()),
                        (a, b) if a < b => ('+', (b - a).abs()),
                        _ => {
                            ('=', 0) // '?'
                        }
                    }
                })
                // .fold - if all signs are the same AND the difference is less than 4, return the sign, else panic to determine edge cases
                .collect::<Vec<_>>();
            // let filtered_count = nums.iter().filter(|(sign, diff)| *diff < 4).count();
            // dbg!(nums);
            // dbg!(&nums[..nums.len() - 1]);
            // dbg!(filtered_count);
            // 1000
            (idx, nums)
        })
        .filter(|(_idx, nums)| {
            // dbg!(&nums[..nums.len() - 1]);
            nums.iter().filter(|(sign, _diff)| *sign == '=').count() < 2
        })
        .filter(|(idx, nums)| {
            // dbg!(&nums[..nums.len() - 1]);
            // nums.iter().all(|sign, _diff| *sign == '+') ||
            // nums.iter().all(|sign, _diff| *sign == '-')
            // let map: HashMap<char, i32> = *nums.clone().into_iter().collect::<HashMap<char, i32>>();
            let h = nums.iter().fold(
                // .take(nums.iter().count() - 1)
                HashMap::new(),
                |mut acc, (sign, _diff)| {
                    let count = acc.entry(*sign).or_insert(0);
                    *count += 1;
                    acc
                },
            );

            if h.keys().len() == 2 {
                let mut l = input.lines().collect::<Vec<_>>()[*idx]
                    .chars()
                    .collect::<Vec<_>>();

                l.sort();

                dbg!(idx, input.lines().collect::<Vec<_>>()[*idx], l, &h);
            }

            if h.keys().len() > 2 {
                return false;
            }

            // if h.keys().len() == 1 {
            //     return true;
            // }

            // all keys are the same
            h.keys().len() == 1 ||
            // or only 2 keys with no more than value of 1 for the lesser key
            /*h.keys().len() < 3 &&*/ {
                let v = h.values().collect::<Vec<_>>();
                // let [a, b] = h.values().collect();
                v[0].min(v[1]) < &2 && nums
                .iter()
                .all(|(_sign, diff)| *diff < 4) //&& nums.iter().all(|(_sign, diff)| *diff < 4)
                // true
            }
        })
        // .filter(|nums| nums.iter().all(|(sign, diff)| *diff < 4))
        .filter(|(_idx, nums)| {
            // dbg!(&nums[..nums.len() - 1]);
            nums[1..nums.len() - 1]
                .iter()
                .all(|(_sign, diff)| *diff < 4)
            // true
        })
        // .inspect(|(idx, nums)| {
        //     println!("{}: {:?}", idx, input.lines().collect::<Vec<_>>()[*idx]);
        //     // dbg!(nums);
        // })
        .count();

    // let output: i32 = input
    //     .lines()
    //     // TODO: nom parser to filter our double duplicates to get the upper bound of "good" lines
    //     // .inspect(|line| {
    //     //     let _ = dbg!(line.chars().collect::<Vec<_>>());
    //     //     let mut last = 0;
    //     //     for c in line.split_whitespace() {
    //     //         let _ = dbg!(c);
    //     //         if(c.parse::<i32>().unwrap() > last) {
    //     //             safe = true;
    //     //         }
    //     //     }
    //     // })
    //     .enumerate()
    //     .map(|(idx, line)| {
    //         let mut safe = false;
    //         let mut escape_hatch = false;
    //         let mut bad: i32 = i32::MIN;

    //         let nums = line
    //             .split_whitespace()
    //             .map(|c| c.parse::<i32>().unwrap())
    //             .collect::<Vec<_>>();

    //         // let nums2 = dbg!(nums.clone());
    //         let nums2 = nums.clone();

    //         // panic!("{}", &nums2.len());

    //         let mut last = 0;
    //         let mut direction = ' ';

    //         for n in nums {
    //             //     dbg!(direction);

    //             // last = n;
    //             match direction {
    //                 '+' => {
    //                     if n > last {
    //                         // dbg!((last - n).abs());
    //                         match (last - n).abs() {
    //                             1..=3 => {
    //                                 safe = true;
    //                             }
    //                             _ => {
    //                                 // dbg!(escape_hatch);
    //                                 if escape_hatch {
    //                                     safe = false;
    //                                     // panic!(" + why ");
    //                                     break;
    //                                 } else {
    //                                     bad = n;
    //                                     escape_hatch = true;
    //                                     continue;
    //                                     // safe = true;
    //                                 }
    //                             }
    //                         }

    //                         // dbg!(safe);
    //                     } else {
    //                         if escape_hatch {
    //                             safe = false;
    //                             break;
    //                         } else {
    //                             bad = n;
    //                             escape_hatch = true;
    //                             continue;
    //                             // safe = true;
    //                         }
    //                     }
    //                 }
    //                 '-' => {
    //                     if n < last {
    //                         // dbg!((n - last).abs());
    //                         match (n - last).abs() {
    //                             1..=3 => {
    //                                 safe = true;
    //                             }
    //                             _ => {
    //                                 // dbg!(escape_hatch);
    //                                 if escape_hatch {
    //                                     safe = false;
    //                                     // panic!(" - why ");
    //                                     break;
    //                                 } else {
    //                                     bad = n;
    //                                     escape_hatch = true;
    //                                     if idx < 2 {
    //                                         direction = ' ';
    //                                     }
    //                                     continue;
    //                                     // safe = true;
    //                                 }
    //                             }
    //                         }

    //                         // safe = true;
    //                     } else {
    //                         if escape_hatch {
    //                             safe = false;
    //                             break;
    //                         } else {
    //                             bad = n;
    //                             escape_hatch = true;
    //                             if idx < 2 {
    //                                 direction = ' ';
    //                             }
    //                             continue;
    //                             // safe = true;
    //                         }
    //                     }
    //                 }
    //                 // first pass
    //                 _ => {
    //                     //             if n > nums2[1] {
    //                     //                 direction = '-';
    //                     //             } else if n < nums2[1] {
    //                     //                 direction = '+';
    //                     //             }
    //                     //             // dbg!(n, last, direction);

    //                     // dbg!(last, nums2[1]);

    //                     if n > nums2[idx as usize + 1] {
    //                         // && nums2[1] > nums2[2]
    //                         direction = '-';
    //                     } else if n < nums2[idx as usize + 1] {
    //                         //&& nums2[1] < nums2[2]
    //                         direction = '+';
    //                     } else {
    //                         // panic!("eql");
    //                         if nums2[idx as usize + 1] > nums2[idx as usize + 2] {
    //                             direction = '-';
    //                         } else if nums2[idx as usize + 1] < nums2[idx as usize + 2] {
    //                             direction = '+';
    //                         } else {
    //                             safe = false;
    //                             break;
    //                         }

    //                         // safe = false;
    //                         // break;
    //                     }
    //                 }
    //             }
    //             last = n;
    //         }

    //         dbg!(&line, direction, safe, escape_hatch, bad);

    //         if safe {
    //             1
    //         } else {
    //             if direction == '-' {
    //                 // println!("{}: {}", idx, &line);
    //                 // dbg!(idx, &line);
    //                 // dbg!(&nums2);
    //                 // dbg!(direction, escape_hatch, bad);
    //             }
    //             0
    //         }
    //     })
    //     .sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1 2 6 3", "1")] // Removing 6 makes a valid 1,2,3 but 2->6 is >3
    #[case("6 2 3 4", "1")] // Removing 6 makes valid 2,3,4 but 6->2 is >3
    #[case("1 5 2 3", "1")] // Removing 5 makes valid 1,2,3 but 1->5 is >3
    #[case("1 2 1 2 3", "0")]
    #[case("3 2 3 2 1", "1")]
    #[case("1 2 3 3 4", "1")]
    #[case("1 2 2 3 4", "1")]
    #[case("1 2 5 3 4", "1")]
    #[case("1 4 2 3 4", "1")]
    #[case("2 2 2 1 0", "0")]
    #[case("1 4 7 10", "1")]
    #[case("10 7 4 1", "1")]
    #[case("7 6 4 2 1", "1")]
    #[case("1 2 7 8 9", "0")]
    #[case("9 7 6 2 1", "0")]
    #[case("1 3 2 4 5", "1")]
    #[case("8 6 4 4 1", "1")]
    #[case("1 3 6 7 9", "1")]
    // #[case("1 1", "0")]
    #[case("1 1 1", "0")]
    #[case("1 1 1 1", "0")]
    #[case("75 76 77 80 82 85 84", "1")]
    #[case("49 52 53 55 58 59 61 61", "1")]
    #[case("54 57 60 62 66", "1")]
    #[case("7 7 7 10 13 15 17 21", "0")]
    #[case("52 52 49 50 49", "0")] // 520 + 1
    #[case("28 28 27 26 23", "1")]
    #[case("45 44 46 47 50 52 54", "1")]
    #[case("83 76 73 72 71 71 72", "0")]
    #[case("35 40 41 38 40 42 49", "0")] // 518 + 1
    #[case("32 35 33 34 35 38", "1")] // 517 + 1
    #[case("10 10 11 12 13 17", "0")] // 514 + 1
    #[case("61 60 65 66 69 73", "0")] // 513 + 1
    #[case("31 33 36 33 34 38", "0")] // 511 + 1
    #[case("51 48 49 51 58 59 57", "0")] // 510 + 1
    #[case("46 50 52 54 53 54 54", "0")] // 507 + 1
    #[case("59 56 53 50 47 47 44 41", "1")] // 506 + 1
    #[case("92 89 86 82 81 84", "0")] // 504 + 1
    #[case("35 32 33 35 37 41 46", "0")] // 503 + 1
    #[case("55 49 48 51 50 53", "0")] // 501 + 1
    #[case("76 80 84 87 89 90 92 92", "0")] // 499 + 1
    #[case("43 38 38 35 32 30 24", "0")] // 498 + 1
    #[case("62 61 64 61 63 64 68", "0")] // 497 + 1
    #[case("62 55 52 51 49 48 43 43", "0")] // 493 + 1
    #[case("45 40 39 38 36 35 29 31", "0")] // 492 + 1
    #[case("83 78 74 71 71", "0")] // 488 + 1
    fn test_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
