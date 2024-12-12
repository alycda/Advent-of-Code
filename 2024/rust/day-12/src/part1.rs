use std::collections::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let thing: HashMap<char, (usize, usize)> = HashMap::new();

    let output = input.lines()
        .flat_map(|line| {
            // dbg!(line);

            line.chars().collect::<Vec<char>>()
        })
        // .inspect(|c| {
        //     // dbg!(c);
        // })
        .fold(thing, |mut acc, c| {
            let (x, _y) = acc.entry(c).or_insert((0, 0));
            *x += 1;
            // *y += 1;
            acc             
        });

    dbg!(output);

    Ok("output".to_string())
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
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
