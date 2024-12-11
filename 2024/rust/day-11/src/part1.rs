use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let numbers = input.split_whitespace()
        // .map(|n| n.parse::<usize>().unwrap())
        .inspect(|stone| {
            dbg!(stone, stone.chars().count());

            if stone == &"0" {
                [dbg!(1)];
            } else if stone.chars().count() / 2 == 0 {
                // split in half
                let (left, right) = stone.split_at(stone.chars().count() / 2);
                let left = left.parse::<usize>().unwrap();
                let right = right.parse::<usize>().unwrap();

                [left, right];
            } else {
                [dbg!(stone.parse::<usize>().unwrap() * 2024)];
            }
        }).count();
        // .collect::<Vec<usize>>();

    Ok("".to_string())
}

fn _blink(_input: &str, _times: usize) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(("125 17", 1), "253000 1 7")]
    // #[case(("125 17", 2), "253 0 2024 14168")]
    // #[case(("125 17", 3), "512072 1 20 24 28676032")]
    // #[case(("125 17", 4), "512 72 2024 2 0 2 4 2867 6032")]
    // #[case(("125 17", 5), "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32")]
    // #[case(("125 17", 6), "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2")]
    fn test_cases(#[case] input: (&str, usize), #[case] expected: &str) {
        // assert_eq!(blink(input.0, input.1), expected);
        assert_eq!(expected, process(input.0).unwrap());
    }

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     let input = "125 17";
    //     assert_eq!("55312", process(input)?);
    //     Ok(())
    // }
}
