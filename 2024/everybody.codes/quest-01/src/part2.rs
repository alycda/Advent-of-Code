use crate::custom_error::EcError;

use super::{hit_count, hit_count_bonus};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, EcError> {
    let output = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|pair| {
            let bonus = pair[0] != 'x' && pair[1] != 'x';

            // dbg!(pair, hit_count(pair[0]), hit_count(pair[1]), bonus);

            dbg!(hit_count_bonus(pair[0], bonus) + hit_count_bonus(pair[1], bonus))
        })
        .sum::<u32>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "AxBCDDCAxD";
        assert_eq!("28", process(input)?);
        Ok(())
    }
}
