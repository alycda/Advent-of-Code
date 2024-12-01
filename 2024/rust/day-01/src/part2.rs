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
            let nums = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            left.push(nums[0]);
            right.push(nums[1]);
        })
        // .flat_map(|line| {
        // })
        .collect::<Vec<_>>();

    // left.sort();
    // right.sort();

    // dbg!(&left, &right);

    for n in 0..left.len() {
    dbg!(left[n]);
        //     // dbg!(right[n]);
        let z = dbg!(right.iter().filter(|&x| x == &left[n]).count());

        //     // dbg!(i32::abs(left[n] - right[n]));

        //     sum += i32::abs(left[n] - right[n]);

        //     // dbg!((left[n] - right[n]).abs());

        sum += dbg!(left[n] * z as i32);
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
