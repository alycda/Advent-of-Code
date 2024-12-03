use nom::error::{Error, ErrorKind};

use crate::{custom_error::AocError, parse_mul};

#[tracing::instrument]
pub fn process(mut input2: &str) -> miette::Result<String, AocError> {
    let mut sum = Vec::new();

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
                        // invalid terminator but string is not empty
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
                            input2 = "";
                        }
                        _ => panic!("{e:?}"),
                    }
                } else {
                    panic!("{e:?}");
                }
            }
        }
    }

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
    #[case("select()} <*mul(843,597)!~mul(717,524)&?}'mul(928,721)>mul(194,52)'why()]-*select()what(898,458):#*mul(31,582)mul(209,470)'-mul(834,167)>}mul(188,914)where(344,689)select(90,321)where()-when()[{]mul(133,940)#-mul(732,657)why()$when()-how()?!>who(208,16)mul(332,604)?", "2792009")]
    
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
