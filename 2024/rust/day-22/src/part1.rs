use crate::{calculate_next_secret, AocError};

fn repeat(input: usize, count: usize) -> usize {
    if count == 0 {
        return input;
    }

    repeat(calculate_next_secret(input), count - 1)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: usize = input.lines()
        .map(|line| {
            let number: usize = line.parse().unwrap();

            repeat(number, 2000)
        })
        .sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case((42, 15), 37)]
    #[case((15, 42), 37)]
    fn test_mix(#[case] input: (usize, usize), #[case] expected: usize) {
        assert_eq!(crate::mix(input.0, input.1), expected);
    }

    #[rstest]
    #[case(100000000, 16113920)]
    fn test_prune(#[case] input: usize, #[case] expected: usize) {
        assert_eq!(crate::prune(input), expected);
    }

    #[rstest]
    #[case((123, 1), 15887950)]
    #[case((123, 2), 16495136)]
    #[case((123, 3), 527345)]
    #[case((123, 4), 704524)]
    #[case((123, 5), 1553684)]
    #[case((123, 6), 12683156)]
    #[case((123, 7), 11100544)]
    #[case((123, 8), 12249484)]
    #[case((123, 9), 7753432)]
    #[case((123, 10), 5908254)]
    fn test_cases(#[case] input: (usize, usize), #[case] expected: usize) {
        assert_eq!(repeat(input.0, input.1), expected);
    }

    // #[rstest]
    // #[case(("1", 2000), "8685429")]
    // #[case(("10", 2000), "4700978")]
    // #[case(("100", 2000), "15273692")]
    // #[case(("2024", 2000), "8667524")]

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(input)?);
        Ok(())
    }
}
