use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    error::{Error, ErrorKind},
    sequence::{preceded, terminated, tuple},
};

use crate::custom_error::AocError;

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
pub fn process(mut input2: &str) -> miette::Result<String, AocError> {
    let mut sum = Vec::new();

    // if let Ok((remainder, (a, b))) = dbg!(parse_mul(input)) {
    //     let a = dbg!(a.parse::<usize>().unwrap());
    //     let b = dbg!(b.parse::<usize>().unwrap());

    //     return Ok((a * b).to_string());
    // }

    // while input.len() > 3 {
    while !input2.is_empty() {
        match parse_mul(input2) {
            Ok((remainder, (a, b))) => {
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();

                sum.push(a * b);

                // input2 = dbg!(remainder);
                input2 = remainder;
            }
            Err(e) => {
                // panic!("{e:?}");

                if let nom::Err::Error(err) = &e {
                    match err {
                        Error {
                            input: _,
                            code: ErrorKind::Char,
                        } => {
                            if input2.len() > 1 {
                                input2 = &input2[1..];
                            } else {
                                panic!("{e:?}");
                            }
                        }
                        Error {
                            input: _,
                            code: ErrorKind::TakeUntil,
                        } => {
                            // input = &input[1..];
                            input2 = "";
                            // break;
                        }
                        _ => panic!("{e:?}"),
                    }
                } else {
                    panic!("{e:?}");
                }

                // invalid terminator but string is not empty

                // e.map(|err| {
                //     // if let ErrorKind::Char = err.code
                //     //     && input.len() > 1
                //     // {
                //     //     input = &input[1..];
                //     // } else {
                //     //     panic!("{e:?}");
                //     // }

                //     match err.code {
                //         ErrorKind::Char => {
                //             if input.len() > 1 {
                //                 input = &input[1..];
                //             } else {
                //                 panic!("{e:?}");
                //             }
                //         }
                //         _ => panic!("{e:?}"),
                //     }
                // });

                // if e.code == ErrorKind::Char && input.len() > 1 {
                //     input = &input[1..];
                // } else {
                //     panic!("{e:?}");
                // }

                // if let nom::Err::Error(err) = ErrorKind::Char {
                //     input = &input[1..];
                // } else {
                //     panic!("{e:?}");
                // }

                // (input, _) = take_until::<&str, &str, Error<&str>>("mul(")(input).unwrap();

                // match e {
                //     nom::Err::Error(err) => {
                //         dbg!(err);
                //         // input = &input[1..];
                //         input = &input[1..];
                //     }
                //     _ => todo!("moar"), // nom::Err::Failure(_) => {
                //                         //     // input = &input[1..];
                //                         //     input = &input[1..];
                //                         // }
                //                         // nom::Err::Incomplete(_) => {
                //                         //     // input = &input[1..];
                //                         //     input = &input[1..];
                //                         // }
                // }

                // if input.len() > 1 {
                //     input = &input[1..];
                // } else {
                //     break;
                // }
                // continue;
            }
        }
    }

    // while let Ok((remainder, (a, b))) = parse_mul(input) {
    //     let a = a.parse::<usize>().unwrap();
    //     let b = b.parse::<usize>().unwrap();

    //     sum.push(a * b);

    //     input = dbg!(remainder);
    // }

    // if let Err(e) = parse_mul(input) {
    //     dbg!(e);
    // }

    // dbg!(&sum);

    Ok(sum.iter().sum::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("mul(2,4)", "8")]
    #[case("mul(5,5)", "25")]
    #[case("mul(32,64].....", "0")]
    #[case("mul(11,8)", "88")]
    #[case("mul(8,5)", "40")]
    #[case("xmul(2,4)%", "8")]
    #[case("xmul(2,4)%&mul[3,7]", "8")]
    #[case("+mul(32,64]then(", "0")]
    #[case("+mul(32,64]then(mul(11,8)mul(8,5))", "128")]
    #[case("]then(mul(11,8)mul(8,5))", "128")]
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
