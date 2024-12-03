use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    sequence::{preceded, terminated, tuple},
};

use crate::custom_error::AocError;

// fn _mul(a: usize, b: usize) -> usize {
//     a * b
// }

// fn recursive_find(input: &str) -> Option<&str> {
//     match input.find("mul(") {
//         Some(idx) => recursive_find(&input[idx + 3..]),
//         _ => None,
//     }
// }

fn parse_mul(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = take_until("mul(")(input)?;
    preceded(
        tag("mul("),
        terminated(
            tuple((
                digit1,
                // consume the comma
                preceded(char(','), digit1),
            )),
            char(')'),
        ),
    )(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    if let Ok((remainder, (a, b))) = dbg!(parse_mul(input)) {
        let a = dbg!(a.parse::<usize>().unwrap());
        let b = dbg!(b.parse::<usize>().unwrap());

        return Ok((a * b).to_string());
    }

    // let indices: Vec<_> = input
    //     .match_indices("mul(")
    //     .inspect(|(idx, s)| {
    //         dbg!(idx, s);

    //         // dbg!(&input[idx + 3..]);
    //         dbg!()
    //     })
    //     // .filter_map(|(idx, s)| {
    //     // })
    //     .collect();

    Ok("".to_string())

    // let mut peekable = input.chars().peekable();

    // let mut vals: Vec<(usize, usize)> = vec![];
    // let mut tmp: String::new();

    // let output: usize = input
    //     .chars()
    //     .flat_map(|c| {

    //     })
    //     // .fold(vals, |c| {
    //     //     if c == 'm' {

    //     //     }
    //     // })
    //     .map(mul).sum();
    //     // .enumerate()
    //     // .flat_map(|(idx, c)| {
    //     // })
    //     // .inspect(|(idx, c)| {
    //     //     if *c == 'm' {
    //     //         dbg!(idx);
    //     //         let _ = peekable.advance_by(*idx + 1);

    //     //         if peekable.peek().unwrap() == &'u' {
    //     //             dbg!(peekable.peek());
    //     //             peekable.next();

    //     //             if peekable.peek().unwrap() == &'l' {
    //     //                 dbg!(peekable.peek());
    //     //                 peekable.next();
    //     //             }
    //     //         }

    //     //         // dbg!(peekable.peek());
    //     //         // peekable.next();
    //     //         // dbg!(peekable.peek());
    //     //         // peekable.next();
    //     //         // dbg!(peekable.peek());
    //     //     }
    //     // })
    //     // .count();

    // Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("mul(2,4)", "8")]
    #[case("mul(5,5)", "25")]
    #[case("mul(32,64]", "")]
    #[case("mul(11,8)", "88")]
    #[case("mul(8,5)", "40")]
    #[case("xmul(2,4)%", "8")]
    #[case("xmul(2,4)%&mul[3,7]", "8")]
    fn test_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
