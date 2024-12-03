use nom::{
    bytes::complete::take_until,
    error::{Error, ErrorKind},
    IResult,
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
            // while let Ok((remaining_section, (a, b))) = parse_mul(current_section) {
            //     let a = a.parse::<usize>().unwrap();
            //     let b = b.parse::<usize>().unwrap();

            //     // dbg!(a, b);

            //     sum.push(a * b);
            //     // Advance the input for next iteration
            //     current_section = remaining_section;
            // }

            match take_until::<&str, &str, Error<&str>>("do()")(input) {
                Ok((remainder, _)) => {
                    let mut current_section = remainder;
                    
                    loop {
                        match parse_mul(current_section) {
                            Ok((remaining_section, (a, b))) => {
                                let a = a.parse::<usize>().unwrap();
                                let b = b.parse::<usize>().unwrap();
                                sum.push(a * b);
                                current_section = remaining_section;
                            },
                            Err(e) => {
                                if let nom::Err::Error(err) = &e {
                                    match err {
                                        Error { code: ErrorKind::Char, .. } => {
                                            if current_section.len() > 1 {
                                                current_section = &current_section[1..];
                                                continue;
                                            }
                                            break;
                                        },
                                        Error { code: ErrorKind::TakeUntil, .. } => break,
                                        _ => break,
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
        
                    if let Some(idx) = remainder.find("don't()") {
                        parse(sum, &remainder[idx..])
                    } else {
                        Ok((remainder, sum.clone()))
                    }
                },
                Err(_) => Ok((input, sum.clone()))
            }

            // // Then check for next "don't()" section
            // if let Some(idx) = remainder.find("don't()") {
            //     parse(sum, &remainder[idx..])
            // } else {
            //     Ok((remainder, sum.clone()))
            // }
        },
        Err(_) => {
            // Base case - no more "do()" patterns found
            Ok((input, sum.clone()))
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
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
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    #[case("don't()<#select()where()when()what()mul(32,31)select()$mul(953,87)^ {)don't(){mul(270,661)@!)who()/!", 0)]
    #[case("don't()<#select()where()when()what()mul(32,31)select()$mul(953,87)^ {)don't(){mul(270,661)@!)who()/!do()?what()who()$why()mul(583,316)*][mul(987,727);<)][^select(769,591)[where()mul(442,27)!$who()where()[^mul(852,777),select()@select(){:?how()mul(255,572)+what()&+!, how()when()mul(558,716)[who()mul(10,703)mul(892,320)how()+) ]do()mul(313,868){~mul(618,505)'select()])>(]! mul(562,800)mul(300,214)/what(),~who()%]~why()mul(255,314)how();^[}-mul(396,152)~^}mul(677,132)}mul(616,398)mul(238,670)when()?*who():%[?#select()mul(23,920)#$;who(){&mul(133/%;mul(15,257)where()-mul(334,84)mul(702,311&mul(372,282)  [from()mul(693,502)@<$+do()why(),[++;~^mul(462,190):-)select()+how()!>$mul(933,126)*'#@!mul(158,156)!<select()>,mul(16,531)*,;:]~mul(992,394)do()@where()select()@{(mul(468,615)<?*&how()'how()mul(444,666)why()mul(715,820^~mul(548,540)*#/#do()(#select()mul(618,741);mul(704,181)>-:^why();](]mul(459,273)&$^]#]when()!when()how()mul(46,732):why()(mul(829,157)[,mul(102,419)+what(48,290){when()&)]what(312,93)mul(963,884[,what()mul(845,518)how();> }&mul(879,886)^[(}{@where()$+?mul(288,560)+~/^mul(529,203)*$#)^~{how()<?mul(73,705)/from()&#'what()&>mul(822,185)+select()~mul(76,474)select()mul(974,326)!:", 9119522)] // 4166561
    #[case("don't()<!,^'$mul(52,436)#?why():mul(488,55)~^)from(111,840)select()when()who()mul(970,182)^#;!:@what()^mul(226,184)[%;&}select()~mul(12,434)#;%mul(983,218)/", 0)]
    #[case("don't()<!,^'$mul(52,436)#?why():mul(488,55)~^)from(111,840)select()when()who()mul(970,182)^#;!:@what()^mul(226,184)[%;&}select()~mul(12,434)#;%mul(983,218)/do()why()?&%@mul(822,567)#mul(919,837)${how()~what(),select()what()how()", 1235277)]
    #[case("don't()why()):what()$from(33,368)]select()mul(743,610)", 0)]
    #[case("don't()&##how(),}%mul(264,642)from()how()-$^how() >}what()mul(869,702)who()+--", 0)]
    #[case("don't()from(505,607))/}-,where())#mul(142,853)-}<", 0)]
    #[case("don't()>how()who()mul(472,905)mul(194,99)mul(717,853)from()^  ,mul(529,565)mul(4,269)from()++", 0)]
    #[case("don't() ]$[[{mul(815,266)from()>when(352,983)when()?*mul(392,42)what()^mul(640,124),+-~mul(96,4)'&}^!mul(371,890)}", 0)]
    #[case("don't()&when()mul(523,426)}-(%;]mul(631,209)'when()when(),what()~where(809,311)* mul(151,849)}}where()mul(17,734)/[?{mul(29,296)#:select()how()who()what()>&: mul(883,275))how()% //>", 0)]
    #[case("don't()~>,]^#*:mul(10,13) select()+when()[select()where()[~", 0)]
    #[case("don't():/how()$where()from()?where(122,41)mul(719,403);from()mul(235,691)<[]&{/who(414,749)*>mul(645,880)&mul(467,964)#[@/!{?:mul(122,185)how()@')+<;when()@mul(966,168)who()who()how()#,(@(what()mul(421,48) ?]*select()+what()what();mul(585,404)> ;!@}&when()#mul(219,32)-what()[$mul(453,862)< ^from()]#>~", 0)]
    #[case("don't()mul(852,168)&where()&/!*{mul(644,741)>;-&where()}{who()@mul(598,266)]!>{*-mul(360,534)- mul(622,4)^,mul(364,523);-where()',}]mul(392,56)(why(786,236){&+mul(421,577)'from()select()*+who(635,837)~mul(813,646)$>!mul(849,457)select()who()?-+mul(345,668)who();mul(456,636)from()mul(524,899)}(! (how()mul(599,822)select(824,479)mul(996,930)}what(756,659))^*%mul(641,495)(#]what()*,'*%(mul(743,569)$[*select()when()~>(+mul(759,831):<from(35,464)what()&from()^%]#mul(268,525)(/-where(691,810){:+&}when()mul(757,376)when() #,)#how()$ mul(238,904)]/[select()#what()?~mul(524,81)select()~where()how()", 0)]
    fn test_parse(#[case] input: &str, #[case] expected: usize) {
        let mut sum: Vec<usize> = Vec::new();
        let (_remainder, v) = parse(&mut sum, input).unwrap();

        assert_eq!(v.iter().sum::<usize>(), expected);

    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
