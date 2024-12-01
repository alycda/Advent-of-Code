use crate::{custom_error::AocError, unzip};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    let output: i32 = left
        .iter()
        .map(|n| n * right.iter().filter(|&x| x == n).count() as i32)
        .sum();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
