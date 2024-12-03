use nom::{
    bytes::complete::take_until,
    error::{Error, ErrorKind},
    sequence::preceded,
};

use crate::{custom_error::AocError, parse_mul};

// #[derive(Debug)]
// enum ParsedResult {
//     Do(String),
//     Dont(String),
// }

// fn parse_do(input: &str) -> IResult<&str, ParsedResult> {
//     let (input, _) = take_until("do()")(input)?;

//     preceded(first, second)

// }

// fn parse_dont(input: &str) -> IResult<&str, ParsedResult> {
//     let (input, _) = take_until("don't()")(input)?;
// }

// fn parse_either(input: &str) -> IResult<&str, ParsedResult> {
//     alt((
//         parse_do,
//         parse_dont,
//     ))(input)
// }

#[tracing::instrument]
pub fn process(mut input: &str) -> miette::Result<String, AocError> {
    let mut sum = Vec::new();

    // first pass
    if let Ok((remainder, (a, b))) = parse_mul(input) {
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();

        sum.push(a * b);

        // input = remainder;
    } else {
        panic!("bad input");
        // return Err(AocError::IoError("bad input".to_string()));
    }

    while !input.is_empty() {
        match take_until::<&str, &str, Error<&str>>("do()")(input) {
            Ok((remainder, _)) => match parse_mul(remainder) {
                Ok((remainder, (a, b))) => {
                    let a = a.parse::<usize>().unwrap();
                    let b = b.parse::<usize>().unwrap();

                    sum.push(a * b);

                    // input = dbg!(remainder);
                    input = remainder;
                }
                _ => {
                    // todo!()
                    input = &input[1..];
                }
            },
            Err(e) => {
                // if let nom::Err::Error(err) = &e {
                //     Error {
                //         input: _,
                //         code: ErrorKind::TakeUntil,
                //     } => {
                //         input = "";
                //     }
                //     _ => panic!("{err:?}"),
                // } else {
                //     panic!("{e:?}");
                // }

                if let nom::Err::Error(err) = &e {
                    match err {
                        // invalid terminator but string is not empty
                        // Error {
                        //     input: _,
                        //     code: ErrorKind::Char,
                        // } => {
                        //     if input.len() > 1 {
                        //         input = &input[1..];
                        //     } else {
                        //         panic!("{e:?}");
                        //     }
                        // }
                        Error {
                            input: _,
                            code: ErrorKind::TakeUntil,
                        } => {
                            input = "";
                        }
                        _ => panic!("{e:?}"),
                    }
                } else {
                    panic!("{e:?}");
                }
            }
        }
    }

    dbg!(&sum.len());
    dbg!(&sum);

    Ok(sum.iter().sum::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "select()} <*mul(843,597)!~mul(717,524)&?}'mul(928,721)>mul(194,52)'why()]-*select()what(898,458):#*mul(31,582)mul(209,470)'-mul(834,167)>}mul(188,914)where(344,689)select(90,321)where()-when()[{]mul(133,940)#-mul(732,657)why()$when()-how()?!>who(208,16)mul(332,604)?",
        "2792009"
    )]

    #[case(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?",
        "8"
    )]
    #[case(
        // "&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", // invalid, input will always begin with a valid mul
        "mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        "65"
    )]
    fn test_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
