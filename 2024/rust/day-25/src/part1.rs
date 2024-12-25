use std::collections::HashSet;

use crate::AocError;

#[derive(Debug)]
enum PatternType {
    Lock,
    Key,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut locks: HashSet<[u8; 5]> = HashSet::new();
    let mut keys: HashSet<[u8; 5]> = HashSet::new();

    let _ = input.split("\n\n")
        .for_each(|pattern| {
            // dbg!(pattern);

            let mut peek = pattern.chars().peekable();

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
                                }
                            });
                        
                        keys.insert(columns.try_into().unwrap());
                    },
                }


            }
        });

    // dbg!(locks, keys);

    // Ok("".to_string())

    fn is_compatible(lock: &[u8; 5], key: &[u8; 5]) -> bool {
        lock.iter()
            .zip(key.iter())
            .all(|(&lock_height, &key_height)| lock_height + key_height <= 5)
    }

    let mut matches = Vec::new();
    for lock in &locks {
        for key in &keys {
            if is_compatible(lock, key) {
                matches.push((lock, key));
            }
        }
    }

    Ok(matches.len().to_string())
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
