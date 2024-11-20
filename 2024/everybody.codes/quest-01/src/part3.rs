use crate::custom_error::EcError;

use super::{hit_count, hit_count_bonus, hit_count_double_bonus};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, EcError> {
    let output = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|enemies| {
            let x_count = dbg!(dbg!(enemies).iter().filter(|&&c| c == 'x').count());

            match x_count {
                0 => {
                    let bonus = enemies[0] != 'x' && enemies[1] != 'x' && enemies[2] != 'x';

                    hit_count_double_bonus(enemies[0], bonus)
                        + hit_count_double_bonus(enemies[1], bonus)
                        + hit_count_double_bonus(enemies[2], bonus)
                }
                1 => {
                    hit_count_bonus(enemies[0], enemies[0] != 'x')
                        + hit_count_bonus(enemies[1], enemies[1] != 'x')
                        + hit_count_bonus(enemies[2], enemies[2] != 'x')
                }
                2 => hit_count(enemies[0]) + hit_count(enemies[1]) + hit_count(enemies[2]),
                _ => 0,
                // _ => hit_count(enemies[0]) + hit_count(enemies[1]) + hit_count(enemies[2]),
            }
        })
        .sum::<u32>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xBxAAABCDxCC";
        assert_eq!("30", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case("xxx", 0)]
    #[case("xbx", 1)]
    #[case("xBx", 1)]
    #[case("AAA", 6)]
    #[case("BCD", 15)]
    #[case("xCC", 8)]
    fn multi_test(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process(input).unwrap().parse::<u32>().unwrap());
    }
}
