use crate::{custom_error::AocError, is_safe, line_to_nums};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .filter(|line: &&str| is_safe(line_to_nums(line)))
        .count();

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
