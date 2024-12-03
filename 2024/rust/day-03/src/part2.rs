use core::panic;
use std::cmp::Ordering;

use nom::{
    bytes::complete::take_until,
    error::{Error, ErrorKind},
    sequence::preceded, IResult,
};

use crate::{custom_error::AocError, parse_mul};

// fn parse<'a>(sum: &'a mut Vec<usize>, input: &'a str) -> IResult<&'a str, Vec<usize>> {
//     assert!(&input[0..7] == "don't()");

//     // let mut input2 = input;

//     match take_until::<&str, &str, Error<&str>>("do()")(input) {
//         Ok((remainder, _)) => {
//             // find the next don't and parse the string before it
//             // let mut input2 = remainder;

//             if let Some(idx) = remainder.find("don't()") {
//                 while let Ok((_remainder, (a, b))) = parse_mul(&remainder[..idx]) {
//                     let a = a.parse::<usize>().unwrap();
//                     let b = b.parse::<usize>().unwrap();

//                     sum.push(a * b);

//                     // input2 = remainder;
//                     // remainder = remainder;
//                 }

//                 parse(sum, &input[idx..])
//             } else {
//                 // todo!("return vec??");
//                 IResult::Ok(("", sum.clone()))
//             }
//         },
//         Err(e) => todo!("return vec?")
//     }
// }

fn parse<'a>(sum: &'a mut Vec<usize>, input: &'a str) -> IResult<&'a str, Vec<usize>> {
    assert!(&input[0..7] == "don't()");

    match take_until::<&str, &str, Error<&str>>("do()")(input) {
        Ok((remainder, section)) => {
            // dbg!(remainder);

            // First try to parse any mul operations in the current section
            let mut current_section = remainder;
            while let Ok((remaining_section, (a, b))) = parse_mul(current_section) {
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();

                // dbg!(a, b);

                sum.push(a * b);
                // Advance the input for next iteration
                current_section = remaining_section;
            }

            // Then check for next "don't()" section
            if let Some(idx) = remainder.find("don't()") {
                parse(sum, &remainder[idx..])
            } else {
                Ok((remainder, sum.clone()))
            }
        },
        Err(_) => {
            // Base case - no more "do()" patterns found
            Ok((input, sum.clone()))
        }
    }
}

#[tracing::instrument]
pub fn process(mut input: &str) -> miette::Result<String, AocError> {
    let mut sum: Vec<usize> = Vec::new();
    let first_do = input.find("do()");
    let first_dont = input.find("don't()");

    if let (_, None) = (first_do, first_dont) {
        return crate::part1::process(input);
    }

    // dbg!(first_do, first_dont);

    // match first_do.cmp(&first_dont) {
    //     Ordering::Less => {
    //         panic!("do first");
    //     }
    //     Ordering::Greater => {
            let (first, last) = input.split_at(first_dont.unwrap());

            sum.push(crate::part1::process(first).unwrap().parse::<usize>().unwrap());

            // dbg!(last);
            if let Ok((_, v)) = parse(&mut sum, last) {
                // assert_eq!(v.iter().sum::<usize>(), 40);
                // return Ok(v.iter().sum::<usize>().to_string())
            }
    //     }
    //     _ => panic!("unexpected ordering"),
    // }

    // Ok("".to_string())



    // // first pass
    // if let Ok((remainder, (a, b))) = parse_mul(input) {
    //     let a = a.parse::<usize>().unwrap();
    //     let b = b.parse::<usize>().unwrap();

    //     sum.push(a * b);

    //     // input = remainder;
    // } else {
    //     panic!("bad input");
    //     // return Err(AocError::IoError("bad input".to_string()));
    // }

    // while !input.is_empty() {
    //     match take_until::<&str, &str, Error<&str>>("do()")(input) {
    //         Ok((remainder, _)) => match parse_mul(remainder) {
    //             Ok((remainder, (a, b))) => {
    //                 let a = a.parse::<usize>().unwrap();
    //                 let b = b.parse::<usize>().unwrap();

    //                 sum.push(a * b);

    //                 // input = dbg!(remainder);
    //                 input = remainder;
    //             }
    //             _ => {
    //                 // todo!()
    //                 input = &input[1..];
    //             }
    //         },
    //         Err(e) => {
    //             // if let nom::Err::Error(err) = &e {
    //             //     Error {
    //             //         input: _,
    //             //         code: ErrorKind::TakeUntil,
    //             //     } => {
    //             //         input = "";
    //             //     }
    //             //     _ => panic!("{err:?}"),
    //             // } else {
    //             //     panic!("{e:?}");
    //             // }

    //             if let nom::Err::Error(err) = &e {
    //                 match err {
    //                     // invalid terminator but string is not empty
    //                     // Error {
    //                     //     input: _,
    //                     //     code: ErrorKind::Char,
    //                     // } => {
    //                     //     if input.len() > 1 {
    //                     //         input = &input[1..];
    //                     //     } else {
    //                     //         panic!("{e:?}");
    //                     //     }
    //                     // }
    //                     Error {
    //                         input: _,
    //                         code: ErrorKind::TakeUntil,
    //                     } => {
    //                         input = "";
    //                     }
    //                     _ => panic!("{e:?}"),
    //                 }
    //             } else {
    //                 panic!("{e:?}");
    //             }
    //         }
    //     }
    // }

    // dbg!(&sum.len());
    // dbg!(&sum);

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
        "153"
    )]
    fn test_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

    #[rstest]
    #[case("don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", 40)]
    fn test_parse(#[case] input: &str, #[case] expected: usize) {
        let mut sum: Vec<usize> = Vec::new();
        let (remainder, v) = parse(&mut sum, input).unwrap();

        assert_eq!(v.iter().sum::<usize>(), expected);

    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
