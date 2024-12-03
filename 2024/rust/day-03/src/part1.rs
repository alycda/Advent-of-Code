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
    #[case("mul(583,316)mul(987,727);<)][^select(769,591)[where()mul(442,27)!$who()where()[^mul(852,777),select()@select(){:?how()mul(255,572)+what()&+!, how()when()mul(558,716)[who()mul(10,703)mul(892,320)how()+) ]do()mul(313,868){~mul(618,505)'select()])>(]! mul(562,800)mul(300,214)/what(),~who()%]~why()mul(255,314)how();^[}-mul(396,152)~^}mul(677,132)}mul(616,398)mul(238,670)when()?*who():%[?#select()mul(23,920)#$;who(){&mul(133/%;mul(15,257)where()-mul(334,84)mul(702,311&mul(372,282)  [from()mul(693,502)@<$+do()why(),[++;~^mul(462,190):-)select()+how()!>$mul(933,126)*'#@!mul(158,156)!<select()>,mul(16,531)*,;:]~mul(992,394)do()@where()select()@{(mul(468,615)<?*&how()'how()mul(444,666)why()mul(715,820^~mul(548,540)*#/#do()(#select()mul(618,741);mul(704,181)>-:^why();](]mul(459,273)&$^]#]when()!when()how()mul(46,732):why()(mul(829,157)[,mul(102,419)+what(48,290){when()&)]what(312,93)mul(963,884[,what()mul(845,518)how();> }&mul(879,886)^[(}{@where()$+?mul(288,560)+~/^mul(529,203)*$#)^~{how()<?mul(73,705)/from()&#'what()&>mul(822,185)+select()~mul(76,474)select()mul(974,326)!:", "9119522")]
    #[case("do()?'+@mul(375,962)'what() $from()}*(who(739,3)mul(989,755)?!why() /when()<mul(851,753)?how(474,9):*~mul(985,168)who():*mul(784where()what()!+;{:mul(526,20)why()*mul(288,311)@who(173,446)]mul(138?*~;{", "2013816")]
    #[case("do()?what()who()$why()mul(583,316)*][mul(987,727);<)][^select(769,591)[where()mul(442,27)!$who()where()[^mul(852,777),select()@select(){:?how()mul(255,572)+what()&+!, how()when()mul(558,716)[who()mul(10,703)mul(892,320)how()+) ]do()mul(313,868){~mul(618,505)'select()])>(]! mul(562,800)mul(300,214)/what(),~who()%]~why()mul(255,314)how();^[}-mul(396,152)~^}mul(677,132)}mul(616,398)mul(238,670)when()?*who():%[?#select()mul(23,920)#$;who(){&mul(133/%;mul(15,257)where()-mul(334,84)mul(702,311&mul(372,282)  [from()mul(693,502)@<$+do()why(),[++;~^mul(462,190):-)select()+how()!>$mul(933,126)*'#@!mul(158,156)!<select()>,mul(16,531)*,;:]~mul(992,394)do()@where()select()@{(mul(468,615)<?*&how()'how()mul(444,666)why()mul(715,820^~mul(548,540)*#/#do()(#select()mul(618,741);mul(704,181)>-:^why();](]mul(459,273)&$^]#]when()!when()how()mul(46,732):why()(mul(829,157)[,mul(102,419)+what(48,290){when()&)]what(312,93)mul(963,884[,what()mul(845,518)how();> }&mul(879,886)^[(}{@where()$+?mul(288,560)+~/^mul(529,203)*$#)^~{how()<?mul(73,705)/from()&#'what()&>mul(822,185)+select()~mul(76,474)select()mul(974,326)!:", "9119522")]
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
