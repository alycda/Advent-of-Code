use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // u16
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut sum = 0;

    let _ = input
        .lines()
        .inspect(|line| {
            // dbg!(line);
            let nums = dbg!(
                line.split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            );

            left.push(nums[0]);
            right.push(nums[1]);
        })
        // .flat_map(|line| {
        // })
        .collect::<Vec<_>>();

    left.sort();
    right.sort();

    // dbg!(&left, &right);

    for n in 0..left.len() {
        // dbg!(left[n]);
        // dbg!(right[n]);

        // dbg!(i32::abs(left[n] - right[n]));

        sum += i32::abs(left[n] - right[n]);

        // dbg!((left[n] - right[n]).abs());
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("4", "3  7")]
    // #[case("6", (9, 3))]
    // #[case("2", (1, 3))]
    // #[case("1", (2, 3))]
    // #[case("0", (3, 3))]
    // #[case("1", (3, 4))]
    // #[case("2", (3, 5))]
    // #[case("5", (4, 9))]
    // fn test_cases(#[case] expected: &str, #[case] input: (u32, u32)) {
    fn test_cases(#[case] expected: &str, #[case] input: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
