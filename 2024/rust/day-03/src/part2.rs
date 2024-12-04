// use miette::miette;
// use nom::{
//     branch::alt, bytes::complete::{tag, take_until}, character::complete::{self, anychar}, combinator::value, error::{Error, ErrorKind}, multi::{many1, many_till}, sequence::{delimited, separated_pair}, IResult, Parser
// };

// use std::cmp::Ordering;

use nom::{bytes::complete::take_until, error::{Error, ErrorKind}, IResult};

use crate::{custom_error::AocError, parse_mul, Product};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let sum: Vec<Product> = Vec::new();
    let first_do = input.find("do()");
    let first_dont = input.find("don't()");

    if let (_, None) = (first_do, first_dont) {
        dbg!("that was easy");
        return crate::part1::process(input);
    }

    // all mul() before the first don't() are VALID
    let (first, last) = input.split_at(first_dont.unwrap());

    // dbg!(first, last);
    dbg!(first);
    let easy = dbg!(crate::part1::process(first).unwrap().parse::<usize>().unwrap());

    // sum.push(easy);
    dbg!(last);
    let hard = dbg!(parse(sum, last));

    // panic!("halt");

    Ok((easy + hard.unwrap().1.iter().map(|p| p.value()).sum::<usize>()).to_string())
}

fn parse<'a>(mut sum: Vec<Product>, input: &'a str) -> IResult<&'a str, Vec<Product>> {
    // dbg!(&sum);
    // todo!("halt");
    assert!(&input[0..7] == "don't()");

    match take_until::<&str, &str, Error<&str>>("do()")(input) {
        Ok((remainder, section)) => {
            dbg!(&remainder, section);

            // First try to parse any mul operations in the current section
            let current_section = remainder;
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
                                sum.push(Product::new(a, b));

                                // dbg!(a, b);

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
                        Ok((remainder, sum))
                    }
                },
                Err(_) => Ok((input, sum))
            }
        },
        Err(_) => {
            // Base case - no more "do()" patterns found
            Ok((input, sum))
        }
    }
}

// #[tracing::instrument]
// pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let mut a = 0;
//     let mut sum: Vec<usize> = Vec::new();
//     let first_do = input.find("do()");
//     let first_dont = input.find("don't()");

//     if let (_, None) = (first_do, first_dont) {
//         dbg!("that was easy");
//         return crate::part1::process(input);
//     }

//     // dbg!(first_do, first_dont);

//     // match first_do.cmp(&first_dont) {
//     //     Ordering::Less => {
//     //         panic!("do first");
//     //     }
//     //     Ordering::Greater => {
//             let (first, last) = input.split_at(first_dont.unwrap());

//             // dbg!(first, last);

//             a = crate::part1::process(first).unwrap().parse::<usize>().unwrap();

//             dbg!(a);
//             // todo!();

//             dbg!(last);
//             panic!("halt");
//             if let Ok((_, v)) = parse(sum, last) {
//                 dbg!(&v);
//                 // panic!("halt");
//                 // dbg!(v.len());
//                 return Ok((v.iter().sum::<usize>() + a).to_string())
//             } else {
//                 todo!();
//             }
//     //     }
//     //     _ => panic!("unexpected ordering"),
//     // }

//     // Ok(sum.iter().sum::<usize>().to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
//     #[case(
//         "select()} <*mul(843,597)!~mul(717,524)&?}'mul(928,721)>mul(194,52)'why()]-*select()what(898,458):#*mul(31,582)mul(209,470)'-mul(834,167)>}mul(188,914)where(344,689)select(90,321)where()-when()[{]mul(133,940)#-mul(732,657)why()$when()-how()?!>who(208,16)mul(332,604)?",
//         "2792009"
//     )]
    #[case(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?",
        "8"
    )]
    #[case(
        "don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        "40"
    )]
//     #[case(
//         // "&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", // invalid, input will always begin with a valid mul
//         "mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
//         "153"
//     )]
    fn test_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

    #[rstest]
    #[case("don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", 40)]
//     // #[case("", 0)]
//     #[case("don't()where()^mul(493,697)'%+#[!mul(64,397)*^;)how()}!mul(133,628):-&@%?&;when()(mul(245,450){who()*mul(8,731)!([&mul(862,150)who()why()who()'mul(129,583)@:why()[ mul(557,879)why()%,@mul(969,31)^*mul(243,857)#]'?^~mul(418,611)@who()[-mul(381,404)#? who()}}how(),$mul(992,433)#[~from()%mul(823,119)who();when()from()mul(436,64)when()<{when()how(141,705)$<{select(673,665)@mul(776,698)mul(859,821)where(733,36)}>$who()who()<~don't();mul(531,594) ^>&mul(246,430)select()how() %]/mul(80)')][,+-select()-mul(18,784){mul(360,42)) ~who()@+mul(113,212)/*(/mul(765,643)!mul(853,147)[/ mul(396,209)+[?what()mul(479,669))why();;mul(542,614);#}$select()mul(398,910)mul(687,370)mul(59,590)'&what(656,317)(what()*/+()mul(382,325)#:]when()%<,,where()<mul(124,796)what()$/,<@)mul(227,847);mul(588,764)<@select(),who()where()mul(496,225)where()how()<!mul(998,360)+-^){?where()<mul(584,368)why()why()why()>mul(607,933)[what()*what()mul(301,569)where()%{when();)[*%mul(223,95)[select()-why()where()from()?:[<mul(927,19)] *:^'mul(846,824)'~(&what(206,282)[,mul(791]? -&!mul(482,335)mul(835,793);/@mul(115,602)what()!:what()!&]where()>mul(718,286):}why(),mul(2,974) :/where()?! }what()when()mul(379,171)+]select()][{mul(530,485)>*-why()},-how()!who()mul(643,906)}who()mul(906,628)<;{mul(875,497)%;!#^{!how()(mul(29,450)*$how()+from()what():mul(298,289)-how()*mul(771,685)who(44,541)!?when()who()#{mul(87,962)]#mul(479,616)\nmul(604,576)mul%}}how(153,807);@ what()(mul(273,600)!]mul(106,99) mul(461,886)(mul(121what()]'*@+;!mul(513,885)'why()(how()mul(830,191)(>(where()%how()when()what()mul(562,733)^*';:mul(21,307)@what()select()where()~ select()mul(789,818)]mul(11,673)mul(194,572)$#%[/'</ #mul(10who()^!>&}(mul(162,864);{{mul(548,916)(}><+;(}mul(325,72);mul(722,66)}what()mul(703,168),;where()^,mul(530,109)why()where()~from()mul(471,436)mulwhat()(from()~*why()mul(177,943)[select()when()'<?mul(229,627)&%what()[/{~'how(582,475)mul(56,986)mul(999,466),$where()how()select() ~$]select()mul(576,749)*$who()mul(847,95)mul(702,555)*]@when(),how()>!mul(734,260)( who()+-select()>:*mul(63,684)where()#{,)&mul(531,571)[~$where()^({<mul(502,674)mul(490,264)~why()from()select() mul(540,855):+@ select();,,do()}why()how()select(73,137)mul(268,58)-(where()?/'mul(741,485)when()?select()-)where()*<%don't()#mul? ~mul(990,88)$/mul(986,722)?;from()where(394,512)how(593,980)&+mul(667,464);mul(390,181)]who()what()*}+#;mul(429,936)'<:-'+^}mul(346,607-/where()select()%><who()mul(914,888)mul(781,920):*mul(954,791))({who()?~$)mul(769,183)#['{^)*-mul(330,184)select()%what()'who()where()when()(mul(988,148)&who()*&/[/'mul(327,74)/+;?/select()mul(315,381)don't()>}/-#~why()!*mul(721,722),>why() mul(583,596)when()%}$mul(482,164)$mul(230,264)mul(752,60)@'mul(47,57)(mul(17,292)select()where()%)>@why()mul(302,101)<<how()!-[!from()]mul(649,528)^]@^$mul(577,114)/<mul(579,480)who(808,216), #when()why()?mul(979,878)$+why(712,413)$mul:-from()%&}select()mul(539,991)$?>?when()how(761,642)from()mul(501,428)}#how()'what()/+{<+mul(793,630){[$;:[!:what()%don't()what()>from(825,940)' </^&mul(212,563)~when()mul(943,607)where(802,717)'[when()<who())&how()mul(696,659)mul(156//,(mul(477,156)*when(952,865)#*?!>%>!do()when()$<(mul(351,5);-<;,@!why()mul(234,498)who()when()]^/what()^how()do();$]&mul(120,466)*? -#{mul(815,705)why()mul+''*:<select()mul(363,529)# :(,!do()how()[$select()mul(959,174)]$ ^^&(} what()mul(57,223)]!][mul(887,199)who()! &<mul(344,455)% ^&^&mul#!where(431,184) select() do() !^(:+;!mul(342,49)&who(),mul(265,288)#(^mul(360,144)>mul(82,491)where()(don't()^ when()mul(609,399){:#who()$when()from()+mul(506,128)mul(930,401)*%$?mul(140,278)>who()mul(414,2)(-where()!mul(536,354)when()]select()@**mul(678,285){-+select()who(786,299)>$}mul(271,42)][-,[mul(94,592when(870,228)^'~{what()mul(760,390)#>*from()'mul(944,912)why()@#}&/'why(868,249)mul(538,835']),(<~'mul(50,117)}<mul(877,572)when()mul(228,855)}(when()*mul(633,680)/ mul(570,855)[what()mul(807,384)-mul(637,824)}!{who()?$/%mul,+;/how()when()how()mul(907,786)*[&!]~how()mul(580,380){*what()~~(:mul[{:! mul(801,883)}why():'!,}:'mul(419,843)&do()+'}select(),}{,,{mul(294,229)}when(),mul(967,538)/%;who()>*[what(752,345)how()mul(443,685)!mul(278,194)^mul(490,365)mul(197,282)why() what(444,573)from()!+from()how()mul(303,883))how()>mul(999,385)mul(226,867)$mul(726,25)!mul(630,295)when()mul(154,746)<+>when()how(439,850)$,/&mul(990,848)\nwhen()mul(384,519:what()who(675,184)^-&>&@mul(672,770) /#]~<[mul(193,471)where()how()who()what()mul(758,370)when();;^mul(513,602)*mul(366,919)from()#~how()!mul(553,991)select()@}-}#[(mul(963,558)'~'#how()?don't()mul(217,112))mul(988,793)how()from()% why()$'who()mul(694how()&$mul(806,611)*!mul(124,856)*mul(874,117)'who()::][+mul(751,788)select()#-@from()select()}mul(189,473);,mul(241,105)$(?[mul(333,475)!mul(465,601):(&}what()*(-who()%mul(424,124)mul(817,58)/how()-[^[{$(where()mulwhere()who(511,397)~*{<$!*mul(701,469){mul(98,796) ,*mul(118,235)why()why(594,445)%?,#&!why()+mul(402,882)how()who()@select()why()from()<$>mul(600,328what()mul(844,977)#>mul(194,109)+{mul(109,85)$^${~mul(993,810))]#,*:~?^mul(792,228)?how()]$(*mul(227,886)from()^[#}why()#where()when()mul(366,898),^ don't()}/mul(550,557)select();where()&~}<*{&mul(544,847)why()mul(785,91)@-mul(641,315)select()-from()'/how()mul(578,597) how()':,!-%select()<mul(776,139)}select(){ select()<*from()>mul(214,556)why()how()%mul(78,100)mul(924,882)$*+mul(947,155){[^[from()]+*mul(505,891)(@%&&how()mul(9,90)!>>~mul(403,244))mul(378,525)when()[[#'^,~why()why()mul(679,947)$),!from()']]'%mul(310,173)&^]!mul(412,606)who()mul(590,664):]$mul(746,11%+how()?from()how()'mul(721,332)~(,why()+mul(402,868)where(),(what()*select()where(810,63)mul(483,246),)%(-%mul(613,633)++mul(532,625)mul(408,633)mul(73,76)[%why()^how()don't()/where()$mul(383,839)>why()++:-/mul(977,110)> >what()<how()}$)mul(621,138)!^%when()/@mul(483,513)what(319,175)+*what(304,577)mul(672,314) ,}don't()how()-*]^}+*mul(724,495)(#mul(639,551)mul(714,97)mul(833,445)what()<}&(mul(11,270)mul(702,275):?<from()mul(466,941)what()how()[+[where(265,812) from()do()+when(215,956)where()!#mul(170,386why()mul(166,604)select()&:##>(why()<mul(405,741),~/&$mul(584,632)~mul(940,464)%'select()where()*+*&what()mul(33,565),}-how()*%:mul(381,287)^from(912,230):from()%+^!mul(874,596)mul(878,587)$where())>mul(255,880)??$)mul(886,864/[]how()mul(594,934)(what(669,547),^%~mul(695from()>:{#where()select(),{{where()mul(249,262)'where()from()@/:)*mul(599,339)&[^from()<#where()mul(784,206)/'$(mul(123,960)([~what()}<mul(859,916)!++%*don't()from())~how(),'>how()<#mul(810,625)why()}!mul(890,890)mul(138,681)don't()when()where()[~'(/)mul(763,795)}+<when(383,476)select()?mul(389,579)why()},mul(794,264)why()@*where()/select()[mul(697,918)~'who()mul(795,412)#(]@mul(591,895)#/&from()>![mul(713,10)select()who()*^$%from()+>mul(608,902)+who()@?>how() )mul(695,686)how()]@mul(921,528)who()select()/<!mul(516,125) ](!how() select()>:mul(367,314)/!:>&why()?)#%mul(41,92))$;*mul(35,569)mul(167,823){do()/$select()what()mul(223,84)mul(416,808)mul(81,303)['what()where()!*why()/mul(287,320)};**how()mul(52,317)!(~;;?mul(39,918)%)~who()mul(193,87)@!why()#;,mul(609,5)when(96,492)!{^<!what()mul(515,479)where()do();how()select()what()<mul(534,62)'!what()>[what();who()mul(405,321)#!/usr/bin/perlwhen()>/+ mul(467,220)\n'mul(660,982)}where()what()+>?-'-mul(40,133){/mul(133,385)[#/}when()select()mul(91,822)+>from()from()$%what()~mul(307,329)%mul(603,733)*mul(592,459)how()&/-@mul(594,38)how()$mul(195,687)]&'when()>-!-mul(42,749)what()>-,how()why()mul(290,744)-%>select()@~mul(424,826)mul(506,207)>where()#]mul(624,508)?,,)+what()mul(305,115)/?'{]<}when()mul(882,936)/from()>,mul(969,401)!($, ^&;*mul(864,212)'where()(;,'select()mul<*?,mul(740,752)!<)&&~]how()mul(880,406)%{'who()}why()when()-}from()mul(233,304)[what()how()'mul(523,820)%^&,^;,-]~mul(880,95)$+)select()$select()~>?'mul(789,736)why():]>select()@what(648,823)how()mul(328;'*;?;mul(660,867),mul(467,677)]how()how()^(# *[mul(86,48)!where()mul(681,991)mul(845,346)-~*how(943,730)@mul(631,756)>?(^mul(526}where();&$>from()%)don't()?#how()%mul(437,734)^how()/?),$who()mul(922,116):from()mul(717,839)where()$mul(482,708)what()}@mul(552,757)<;&mul(500,35)~#},$]mul(403!where()}{don't()from()(*%^who(950,484)&,mul(275,360)~^mul(633,22)when()!from(){''mul(192,138)@&select()( who()':mul(336,787)why()#who(135,195)select()$don't()what())from()(what()@where()~how(424,830)@mul(94,582)'how()don't()mul(832,580))]~  who(),[((mul(640,409)+/from()# )mul(469,28)why()*@)*mul(672,877),mul(107,95)who() ({{do()mul(551,389);<>#mul(281,726)mul(109,832)why()mul(429,124))%^'(,mul(564,346)mul^@](mul(836,870)mul(732,724)$$'>mul(802,696),%/]mul(376,742)why()mul(918,947%mul(956,894)};)when()]/mul(978,168)/mul(787,349)select()(*why()#,}who()(where()mul(172,325)()mul(715,639)^*who()mul(119,208)select()#%select()mul(819,355)how()]+-mul(703,499):,+;<mul(75,508$$'mul(370,741)when()what()mul(125,40)when()[how()<>mul(501,610){!+mul(242,771)who()where()who()}:~how(690,244)when()when()#mul(883,139)%<,?^)mul(724,106)+,@;](*>$mul(980,328<);<+^?when()+mul(81,543)how(10,555)mul(608,375)~%]]/<select()(how()what()mul(772,18)mul(9,29){!where())?:@!mul(547,225)^'^%(who()>-),mul(526,816)<<:how()@>who(502,331),why(830,316)-mul(272,110)#mul(551,116)how()(~-;:</[mul(586,524){mul(605,994);[&@mul(691,659),who(889,972)?where()<,?,mul(744,764)$mul(392,795)}>%}&mul(412,731+don't()from()[~~#mul(335,663)mul(790,482)[{why()>mul(297,579);when()why()++select() mul(300,275),[&?who(936,450),*[how()mul(265,542)!&what()why()](-mul(475,924)when() &>(select()<why()(mul(902,616~]when():?how()@>:from()mul(121,602) !'}}> #mul(250,523)what(),why(549,381)!<how()from()mul(422,934)[when()do()[ what()>?from()mul(976,959)->%/mul(840,957)from()$?-+?(:*}mul(566/%why()don't()who()why():{^;when()*who()mul(130,463))>[?>!#&,!mul(216,145) ?/where()'mul(663,743)!mul(569,580)!#%mul(813,913)when()!mul(123,288)!#>/{&^,//do()who()[?@#,;@select()]mul(408,609)where()!$($mul(141,976)[# when(310,195)mul(459,859)mul(643,525)what(101,355)(mul(26,220)$]when()why()mul(638,791)!mul(611,536)~/+who()mul(853,292)from()mul(624,418)how()^how()+from()where())%mul&)@ [ mul(629,413)~^when()!mul(418,615)/}%mul(361,174):,~*when()select()mul(901,104)\n[<['mul(279,208)@])why()don't()mul(902,65)who()]%]~mul(862,126)why())/mul(329,407)~/;from()from()mul(99,704)^$# }@who()]mul(355,170);(;where()do():)when():select()mul(859,599)~+,@>~mul(411,160)~+;^}mul(377,233)*)mul(127,759where()[${where()(where()$>mul(390,173)<[mul(147,551)$^/~[what()~mul+(')when();%mul(324,388)mul(762,361)@+@mul(944'}mul(726,627)what()&:-+:;}mul(463,374)mul(696,477)where(102,685)}#;*?from()how()>mul(707,273)!^&[,^(mul(828,638) what()!@!,&*@mul(714,244)what()$mul(275,818)}when()'when()($,mul(523,833)})}#from(),mul(784,51)how()when(),who()[:~mul(684,848){>:select(397,607)<?&'[ mul(152,194)<'>do()?(?>)#^^*,mul(690,323)>+#mul(106,680)!#$mul(639,592)<where()@}#mul(735,82)don't()-':]@what()}who()mul(832,477)@mul(435,430)+}!!'mul(786,743)why()-mul(217,417)why()where()what()-) do()+mul(873,520)]>*),mul(331,382)from())don't()who()where(){-why(599,2)%why()#-mul(81,730)why();:$mul(130,622)who()when()mul(475,382)@mul(129,264)#^+from(){->mul(183,487)%}from()(when()?&mul(164,385)~select())?$mul(380,295)from()*why()'}<;,#'mul(803,439)~?mul(901,6)*~mul(421,800)^ mul(735,90){%&when()]from() +(mul(699)how()where()why()what()^+why(),{mul(749,237),mul(611,151):$%mul(288,333)mul(154,215)do():!:#where()~mul],mul(656,936)~%&where()@)[what(554,139)mul(231,221)%?[why()*+who()-}[mul(615,700)*-)}%-/~$!mul(618,537))}where()}:what()@!'mul(788,743)when();&~>mul(561,506)]{<how()mul(598,888)what()mul(708,163)<<)don't()%-^mul(400,938)select()!]~{?!mul(826,913)(mul(932,484)}mul(254,714),mul(229)'who()/:when(132,46)*;!mul(188,973))mul(312,148)^(:'[}%when()do()]])what()%+}from(682,977) mul(113,802)mul(700,433)!)'select(310,619)mul(26,157)[(-mul(952,142)!mul(6,50))](from()select()don't()]mul(937,379)select()>;mul(705,21)%mul(560,522)]$%?:!how()mul(356,348)from()who()>[+#do()why()!mul(727,800)[mul(638,20):don't()?mul(542,909)^[select())%*$how()/)mul+$;~how()~$who()mul(298,583)-[[where(348,105)#who()[)mul(217,517):>#>what(),don't()& #mul(196,259)where()mul(575%where(){mul(217,122)-(when()how()why()mul(670,143)mul(311,499)>when()~mul(550,706) @&?-+#)mul(18,100)from()!>*-'mul(607,54):@ ){mul(380,493)'&<+select()%don't()([mul(395,363)%?}from()>}how()&where()mul(381,203)why()from():-mul(768,277)from()+where()what(586,751)-]!where()?mul(666,807))what()what()why()who()mul*mul(454,892)[?-mul~where()what()#<{who(294,31)what()select();mul(278,629)+when()mul(849,706)when()(*mul(729,270!$',:#~[^!mul(733,808)' -}' select()$don't()+#)select()}how(25,907)what()#mul(184,152)@:<mul(413,268)how()'/select()mul(670,925)+when()*how()why()]#'%why()mul(256,595)}from()[+#@what()[how()}don't()&(who()<-?%(how()mul(163,813why()$who()when()(@select()mul(188,453)don't()who()~?mul(990,172<select()((} who()){mul(769,716)']how(996,987);&&)mul(950,749)what()mul(537,165)&!/,<--what():mul(483,428)(why()*select()&{where()who()mul{[from()[what()(]}]mul(267,746)[/what()^+[#don't()mul(498,108)&from()}}%mul(897,862)(from()>,'!?-mul(209,634)&mul(343,899)]}from()<:where()how(){when()mul(636,452)![mul(622,596)do()?'+@mul(375,962)'what() $from()}*(who(739,3)mul(989,755)?!why() /when()<mul(851,753)\n?how(474,9):*~mul(985,168)who():*mul(784where()what()!+;{:mul(526,20)why()*mul(288,311)@who(173,446)]mul(138?*~;{don't()<#select()where()when()what()mul(32,31)select()$mul(953,87)^ {)don't(){mul(270,661)@!)who()/!do()?what()who()$why()mul(583,316)*][mul(987,727);<)][^select(769,591)[where()mul(442,27)!$who()where()[^mul(852,777),select()@select(){:?how()mul(255,572)+what()&+!, how()when()mul(558,716)[who()mul(10,703)mul(892,320)how()+) ]do()mul(313,868){~mul(618,505)'select()])>(]! mul(562,800)mul(300,214)/what(),~who()%]~why()mul(255,314)how();^[}-mul(396,152)~^}mul(677,132)}mul(616,398)mul(238,670)when()?*who():%[?#select()mul(23,920)#$;who(){&mul(133/%;mul(15,257)where()-mul(334,84)mul(702,311&mul(372,282)  [from()mul(693,502)@<$+do()why(),[++;~^mul(462,190):-)select()+how()!>$mul(933,126)*'#@!mul(158,156)!<select()>,mul(16,531)*,;:]~mul(992,394)do()@where()select()@{(mul(468,615)<?*&how()'how()mul(444,666)why()mul(715,820^~mul(548,540)*#/#do()(#select()mul(618,741);mul(704,181)>-:^why();](]mul(459,273)&$^]#]when()!when()how()mul(46,732):why()(mul(829,157)[,mul(102,419)+what(48,290){when()&)]what(312,93)mul(963,884[,what()mul(845,518)how();> }&mul(879,886)^[(}{@where()$+?mul(288,560)+~/^mul(529,203)*$#)^~{how()<?mul(73,705)/from()&#'what()&>mul(822,185)+select()~mul(76,474)select()mul(974,326)!:don't()<!,^'$mul(52,436)#?why():mul(488,55)~^)from(111,840)select()when()who()mul(970,182)^#;!:@what()^mul(226,184)[%;&}select()~mul(12,434)#;%mul(983,218)/do()why()?&%@mul(822,567)#mul(919,837)${how()~what(),select()what()how()don't()why()):what()$from(33,368)]select()mul(743,610)don't()&##how(),}%mul(264,642)from()how()-$^how() >}what()mul(869,702)who()+--don't()from(505,607))/}-,where())#mul(142,853)-}<don't()>how()who()mul(472,905)mul(194,99)mul(717,853)from()^  ,mul(529,565)mul(4,269)from()++don't() ]$[[{mul(815,266)from()>when(352,983)when()?*mul(392,42)what()^mul(640,124),+-~mul(96,4)'&}^!mul(371,890)}don't()&when()mul(523,426)}-(%;]mul(631,209)'when()when(),what()~where(809,311)* mul(151,849)}}where()mul(17,734)/[?{mul(29,296)#:select()how()who()what()>&: mul(883,275))how()% //>don't()~>,]^#*:mul(10,13) select()+when()[select()where()[~don't():/how()$where()from()?where(122,41)mul(719,403);from()mul(235,691)<[]&{/who(414,749)*>mul(645,880)&mul(467,964)#[@/!{?:mul(122,185)how()@')+<;when()@mul(966,168)who()who()how()#,(@(what()mul(421,48) ?]*select()+what()what();mul(585,404)> ;!@}&when()#mul(219,32)-what()[$mul(453,862)< ^from()]#>~don't()mul(852,168)&where()&/!*{mul(644,741)>;-&where()}{who()@mul(598,266)]!>{*-mul(360,534)- mul(622,4)^,mul(364,523);-where()',}]mul(392,56)(why(786,236){&+mul(421,577)'from()select()*+who(635,837)~mul(813,646)$>!mul(849,457)select()who()?-+mul(345,668)who();mul(456,636)from()mul(524,899)}(! (how()mul(599,822)select(824,479)mul(996,930)}what(756,659))^*%mul(641,495)(#]what()*,'*%(mul(743,569)$[*select()when()~>(+mul(759,831):<from(35,464)what()&from()^%]#mul(268,525)(/-where(691,810){:+&}when()mul(757,376)when() #,)#how()$ mul(238,904)]/[select()#what()?~mul(524,81)select()~where()how()\n", 0)]
//     // #[case("", 0)]
//     // #[case("don't()<#select()where()when()what()mul(32,31)select()$mul(953,87)^ {)don't(){mul(270,661)@!)who()/!", 0)]
//     // #[case("don't()<#select()where()when()what()mul(32,31)select()$mul(953,87)^ {)don't(){mul(270,661)@!)who()/!do()?what()who()$why()mul(583,316)*][mul(987,727);<)][^select(769,591)[where()mul(442,27)!$who()where()[^mul(852,777),select()@select(){:?how()mul(255,572)+what()&+!, how()when()mul(558,716)[who()mul(10,703)mul(892,320)how()+) ]do()mul(313,868){~mul(618,505)'select()])>(]! mul(562,800)mul(300,214)/what(),~who()%]~why()mul(255,314)how();^[}-mul(396,152)~^}mul(677,132)}mul(616,398)mul(238,670)when()?*who():%[?#select()mul(23,920)#$;who(){&mul(133/%;mul(15,257)where()-mul(334,84)mul(702,311&mul(372,282)  [from()mul(693,502)@<$+do()why(),[++;~^mul(462,190):-)select()+how()!>$mul(933,126)*'#@!mul(158,156)!<select()>,mul(16,531)*,;:]~mul(992,394)do()@where()select()@{(mul(468,615)<?*&how()'how()mul(444,666)why()mul(715,820^~mul(548,540)*#/#do()(#select()mul(618,741);mul(704,181)>-:^why();](]mul(459,273)&$^]#]when()!when()how()mul(46,732):why()(mul(829,157)[,mul(102,419)+what(48,290){when()&)]what(312,93)mul(963,884[,what()mul(845,518)how();> }&mul(879,886)^[(}{@where()$+?mul(288,560)+~/^mul(529,203)*$#)^~{how()<?mul(73,705)/from()&#'what()&>mul(822,185)+select()~mul(76,474)select()mul(974,326)!:", 9119522)] // 4166561
//     // #[case("don't()<!,^'$mul(52,436)#?why():mul(488,55)~^)from(111,840)select()when()who()mul(970,182)^#;!:@what()^mul(226,184)[%;&}select()~mul(12,434)#;%mul(983,218)/", 0)]
//     // #[case("don't()<!,^'$mul(52,436)#?why():mul(488,55)~^)from(111,840)select()when()who()mul(970,182)^#;!:@what()^mul(226,184)[%;&}select()~mul(12,434)#;%mul(983,218)/do()why()?&%@mul(822,567)#mul(919,837)${how()~what(),select()what()how()", 1235277)]
//     // #[case("don't()why()):what()$from(33,368)]select()mul(743,610)", 0)]
//     // #[case("don't()&##how(),}%mul(264,642)from()how()-$^how() >}what()mul(869,702)who()+--", 0)]
//     // #[case("don't()from(505,607))/}-,where())#mul(142,853)-}<", 0)]
//     // #[case("don't()>how()who()mul(472,905)mul(194,99)mul(717,853)from()^  ,mul(529,565)mul(4,269)from()++", 0)]
//     // #[case("don't() ]$[[{mul(815,266)from()>when(352,983)when()?*mul(392,42)what()^mul(640,124),+-~mul(96,4)'&}^!mul(371,890)}", 0)]
//     // #[case("don't()&when()mul(523,426)}-(%;]mul(631,209)'when()when(),what()~where(809,311)* mul(151,849)}}where()mul(17,734)/[?{mul(29,296)#:select()how()who()what()>&: mul(883,275))how()% //>", 0)]
//     // #[case("don't()~>,]^#*:mul(10,13) select()+when()[select()where()[~", 0)]
//     // #[case("don't():/how()$where()from()?where(122,41)mul(719,403);from()mul(235,691)<[]&{/who(414,749)*>mul(645,880)&mul(467,964)#[@/!{?:mul(122,185)how()@')+<;when()@mul(966,168)who()who()how()#,(@(what()mul(421,48) ?]*select()+what()what();mul(585,404)> ;!@}&when()#mul(219,32)-what()[$mul(453,862)< ^from()]#>~", 0)]
//     // #[case("don't()mul(852,168)&where()&/!*{mul(644,741)>;-&where()}{who()@mul(598,266)]!>{*-mul(360,534)- mul(622,4)^,mul(364,523);-where()',}]mul(392,56)(why(786,236){&+mul(421,577)'from()select()*+who(635,837)~mul(813,646)$>!mul(849,457)select()who()?-+mul(345,668)who();mul(456,636)from()mul(524,899)}(! (how()mul(599,822)select(824,479)mul(996,930)}what(756,659))^*%mul(641,495)(#]what()*,'*%(mul(743,569)$[*select()when()~>(+mul(759,831):<from(35,464)what()&from()^%]#mul(268,525)(/-where(691,810){:+&}when()mul(757,376)when() #,)#how()$ mul(238,904)]/[select()#what()?~mul(524,81)select()~where()how()", 0)]
    // #[case("", 0)]
    #[case("don't()#mul? ~mul(990,88)$/mul(986,722)?;from()where(394,512)how(593,980)&+mul(667,464);mul(390,181)]who()what()*}+#;mul(429,936)'<:-'+^}mul(346,607-/where()select()%><who()mul(914,888)mul(781,920):*mul(954,791))({who()?~$)mul(769,183)#['{^)*-mul(330,184)select()%what()'who()where()when()(mul(988,148)&who()*&/[/'mul(327,74)/+;?/select()mul(315,381)don't()>}/-#~why()!*mul(721,722),>why() mul(583,596)when()%}$mul(482,164)$mul(230,264)mul(752,60)@'mul(47,57)(mul(17,292)select()where()%)>@why()mul(302,101)<<how()!-[!from()]mul(649,528)^]@^$mul(577,114)/<mul(579,480)who(808,216), #when()why()?mul(979,878)$+why(712,413)$mul:-from()%&}select()mul(539,991)$?>?when()how(761,642)from()mul(501,428)}#how()'what()/+{<+mul(793,630){[$;:[!:what()%don't()what()>from(825,940)' </^&mul(212,563)~when()mul(943,607)where(802,717)'[when()<who())&how()mul(696,659)mul(156//,(mul(477,156)*when(952,865)#*?!>%>!", 0)]
    fn test_parse(#[case] input: &str, #[case] expected: usize) {
        let sum: Vec<Product> = Vec::new();
        let (_remainder, v) = parse(sum, input).unwrap();

        assert_eq!(v.iter().map(|p| p.value()).sum::<usize>(), expected);
        // assert_eq!(expected.to_string(), process(input)?);
        // Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
