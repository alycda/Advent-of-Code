use crate::{custom_error::AocError, is_safe, line_to_nums};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (safe, notsafe): (Vec<_>, Vec<_>) = input
        .lines()
        .enumerate()
        .partition(|(_idx, line)| is_safe(line_to_nums(line)));

    // dbg!(&safe, &notsafe);

    let dampened = notsafe
        .iter()
        // .filter(|(_idx, line)| {
        //     // dbg!(idx, line);
        //     let nums = line_to_nums(line);
        //     for index in 0..nums.len() {
        //         let mut new_nums = nums.clone();
        //         new_nums.remove(index);
        //         if is_safe(new_nums) {
        //             return true;
        //         } else {
        //             continue;
        //         }
        //     }
        //     false
        // })
        .filter(|(_idx, line)| {
            let nums = line_to_nums(line);
            // Check if removing any single number makes the sequence safe
            nums.iter().enumerate().any(|(skip_idx, _)| {
                is_safe(
                    nums.iter()
                        .enumerate()
                        .filter(|(i, _)| *i != skip_idx)
                        .map(|(_, &n)| n)
                        .collect(),
                )
            })
        })
        .count();

    Ok((safe.len() + dampened).to_string())
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
    #[case("3 2 3 2 1", "0")]
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
