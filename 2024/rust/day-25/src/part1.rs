use std::collections::HashSet;

use crate::AocError;

// use nom::{
//     bytes::complete::take_until, character::complete::{char, line_ending}, multi::separated_list1, sequence::{pair, terminated}, IResult
// };

// // First, split on double newline
// fn split_patterns(input: &str) -> IResult<&str, Vec<&str>> {
//     separated_list1(
//         pair(line_ending, line_ending),  // matches \n\n
//         take_until("\n\n")
//     )(input)
// }

#[derive(Debug)]
enum PatternType {
    Lock, // ([u8; 5])
    Key, // ([u8; 5])
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut locks: HashSet<[u8; 5]> = HashSet::new();
    let mut keys: HashSet<[u8; 5]> = HashSet::new();

    // let (patterns, _) = split_patterns(input).unwrap();
    let output = input.split("\n\n")
        .for_each(|pattern| {
            // dbg!(pattern);

            let mut peek = pattern.chars().peekable();

            // pattern.lines()
            //     .for_each(|line| {
            //         dbg!(line);

                    if let Some(c) = peek.next() {
                        // dbg!(c);

                        let pat = match c {
                            '#' => {
                                dbg!("found a lock");
                                PatternType::Lock
                            },
                            '.' => {
                                dbg!("found a key");

                                PatternType::Key
                            },
                            _ => todo!("{c}"),
                        };

                        // dbg!(pattern);
                        let mut columns = vec![0; 5];

                        match pat {
                            PatternType::Lock => {
                                pattern.lines()
                                    .skip(1)
                                    .for_each(|line| {
                                        // dbg!(line);
                                        for (idx, c) in line.chars().enumerate() {
                                            
                                            if c == '#' {
                                                columns[idx] += 1;
                                            }

                                            // match pat {
                                            //     PatternType::Lock => {
                                            //         if c == '#' {
                                            //             columns[idx] += 1;
                                            //         }
                                            //     },
                                            //     PatternType::Key => {
                                            //         if c == '.' {
                                            //             columns[idx] += 1;
                                            //         }
                                            //     },
                                            // }
                                        }
                                    });

                                locks.insert(columns.try_into().unwrap());
                            },
                            PatternType::Key => {
                                pattern.lines()
                                    .take(6)
                                    .for_each(|line| {
                                        // dbg!(line);
                                        for (idx, c) in line.chars().enumerate() {
                                            
                                            if c == '#' {
                                                columns[idx] += 1;
                                            }
        
                                            // match pat {
                                            //     PatternType::Lock => {
                                            //         if c == '#' {
                                            //             columns[idx] += 1;
                                            //         }
                                            //     },
                                            //     PatternType::Key => {
                                            //         if c == '.' {
                                            //             columns[idx] += 1;
                                            //         }
                                            //     },
                                            // }
                                        }
                                    });
                                
                                keys.insert(columns.try_into().unwrap());
                            },
                        }

                        // pattern.lines()
                        //     .skip(1)
                        //     .for_each(|line| {
                        //         // dbg!(line);
                        //         for (idx, c) in line.chars().enumerate() {
                                    
                        //             if c == '#' {
                        //                 columns[idx] += 1;
                        //             }

                        //             // match pat {
                        //             //     PatternType::Lock => {
                        //             //         if c == '#' {
                        //             //             columns[idx] += 1;
                        //             //         }
                        //             //     },
                        //             //     PatternType::Key => {
                        //             //         if c == '.' {
                        //             //             columns[idx] += 1;
                        //             //         }
                        //             //     },
                        //             // }
                        //         }
                        //     });

                        // dbg!(columns);

                    }

            //     });
        });
        // .map()


    dbg!(locks, keys);

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

//     use rstest::rstest;

//     #[rstest]
//     #[case("#####
// .####
// .####
// .####
// .#.#.
// .#...
// .....", "0,5,3,4,3")]
//     #[case(".....
// #....
// #....
// #...#
// #.#.#
// #.###
// #####", "5,0,2,1,3")]
//     // #[case("", "")]
//     // #[case("", "")]
//     // #[case("", "")]
//     // #[case("", "")]
//     // #[case("", "")]
//     fn test_cases(#[case] input: &str, #[case] expected: &str) {
//         todo!();
//         // assert_eq!(process(input).unwrap(), expected);
//     }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
