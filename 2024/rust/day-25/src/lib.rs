//! Day 25: Code Chronicle

use ornaments::{AocError, Solution};

#[derive(Debug, Clone, Copy)]
pub enum PatternType {
    Lock([u8; 5]),
    Key([u8; 5]),
}

pub use crate::Day25 as Day;

#[derive(Debug)]
pub struct Day25(Vec<PatternType>);

impl std::ops::Deref for Day {
    type Target = Vec<PatternType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Day {
    fn helper(line: &str, columns: &mut Vec<u8>) {
        for (idx, c) in line.chars().enumerate() {
                                        
            if c == '#' {
                columns[idx] += 1;
            }
    
        }
    }
}

impl Solution for Day {
    type Output = usize;
    type Item = [u8; 5];

    fn parse(input: &str) -> Self {
        input.split("\n\n")
            .fold(Self(Vec::new()), |mut acc, pattern| {
                let mut peek = pattern.chars().peekable();
                let mut columns = vec![0; 5];

                match peek.next().expect("no input") {
                    '#' => {
                        pattern.lines()
                            .skip(1)
                            .for_each(|line| Self::helper(line, &mut columns));

                        acc.0.push(PatternType::Lock(columns.try_into().unwrap()))
                    },
                    '.' => {
                        pattern.lines()
                            .take(6)
                            .for_each(|line| Self::helper(line, &mut columns));
                        acc.0.push(PatternType::Key(columns.try_into().unwrap()))

                    },
                    _ => todo!(),
                }

                acc
            })
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let locks = self.iter()
            .filter_map(|pattern| {
                match pattern {
                    PatternType::Lock(lock) => Some(lock),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        let keys = self.iter()
            .filter_map(|pattern| {
                match pattern {
                    PatternType::Key(key) => Some(key),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        fn is_compatible(lock: &[u8; 5], key: &[u8; 5]) -> bool {
            lock.iter()
                .zip(key.iter())
                .all(|(&lock_height, &key_height)| lock_height + key_height <= 5)
        }
    
        // let mut matches = Vec::new();
        // for lock in &locks {
        //     for key in &keys {
        //         if is_compatible(lock, key) {
        //             matches.push((lock, key));
        //         }
        //     }
        // }
    
        // Ok(matches.len())

        let compatible_count = locks.iter()
            .flat_map(|lock| keys.iter()
                .filter(|key| is_compatible(lock, key)))
            .count();
        
        Ok(compatible_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
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
        assert_eq!("3", Day::parse(input).solve(Part::One)?);
        Ok(())
    }
}