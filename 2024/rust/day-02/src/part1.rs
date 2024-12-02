//! The levels are either all increasing or all decreasing.
//! Any two adjacent levels differ by at least one and at most three.

use crate::custom_error::AocError;

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: i32 = input
        .lines()
        // .inspect(|line| {
        //     let _ = dbg!(line.chars().collect::<Vec<_>>());
        //     let mut last = 0;
        //     for c in line.split_whitespace() {
        //         let _ = dbg!(c);
        //         if(c.parse::<i32>().unwrap() > last) {
        //             safe = true;
        //         }
        //     }
        // })
        .map(|line| {
            let mut safe = false;

            let nums = line
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let nums2 = dbg!(nums.clone());
            let mut last = 0;
            let mut direction = ' ';

            for n in nums {
                //     dbg!(direction);

                // last = n;
                match direction {
                    '+' => {
                        if n > last {
                            dbg!((last - n).abs());
                            match (last - n).abs() {
                                1..=3 => {
                                    safe = true;
                                }
                                _ => {
                                    safe = false;
                                    // panic!(" + why ");
                                    break;
                                }
                            }

                            // dbg!(safe);
                        } else {
                            safe = false;
                            break;
                        }
                    }
                    '-' => {
                        if n < last {
                            // dbg!((n - last).abs());
                            match (n - last).abs() {
                                1..=3 => {
                                    safe = true;
                                }
                                _ => {
                                    safe = false;
                                    // panic!(" - why ");
                                    break;
                                }
                            }

                            // safe = true;
                        } else {
                            safe = false;
                            break;
                        }
                    }
                    // first pass
                    _ => {
                        //             if n > nums2[1] {
                        //                 direction = '-';
                        //             } else if n < nums2[1] {
                        //                 direction = '+';
                        //             }
                        //             // dbg!(n, last, direction);

                        // dbg!(last, nums2[1]);

                        if n > nums2[1] {
                            direction = '-';
                        } else if n < nums2[1] {
                            direction = '+';
                        } else {
                            // panic!("eql");
                            safe = false;
                            break;
                        }
                    }
                }
                last = n;
            }

            dbg!(direction, safe);

            if safe { 1 } else { 0 }
        })
        // .flat_map(|line| line.split_whitespace())
        // .inspect(|c| {
        //     let _ = dbg!(c);
        // })
        .sum();

    // dbg!(output);

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("7 6 4 2 1", "1")]
    #[case("1 2 7 8 9", "0")]
    #[case("9 7 6 2 1", "0")]
    #[case("1 3 2 4 5", "0")]
    #[case("8 6 4 4 1", "0")]
    #[case("1 3 6 7 9", "1")]
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
