use crate::custom_error::EcError;

use super::hit_count;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, EcError> {
    Ok(input.chars().map(hit_count).sum::<u32>().to_string())
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
