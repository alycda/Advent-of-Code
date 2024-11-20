use crate::custom_error::EcError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, EcError> {
    Ok(input
        .chars()
        .map(|c| match c {
            // 'A' => 1,
            'B' => 1,
            'C' => 3,
            _ => 0,
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "ABBAC";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
