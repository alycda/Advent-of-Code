//! Day 21: 

use std::{collections::{HashMap, HashSet}, marker::PhantomData};

use ornaments::{AocError, Position, Solution};

pub use crate::Day21 as Day;

/// Positions
struct NumberPad;

/// Directions
struct DirectionPad;

struct ButtonPad<T> {
    map: HashMap<char, Position>,
    _kind: PhantomData<T>,
}

impl<T> std::ops::Deref for ButtonPad<T> {
    type Target = HashMap<char, Position>;
    
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Default for ButtonPad<NumberPad> {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert('7', Position::new(0, 0));
        map.insert('8', Position::new(0, 1));
        map.insert('9', Position::new(0, 2));
        map.insert('4', Position::new(1, 0));
        map.insert('5', Position::new(1, 1));
        map.insert('6', Position::new(1, 2));
        map.insert('1', Position::new(2, 0));
        map.insert('2', Position::new(2, 1));
        map.insert('3', Position::new(2, 2));
        map.insert('0', Position::new(3, 1));
        map.insert('A', Position::new(3, 2));
        //
        map.insert('^', Position::new(0, 1));
        map.insert('a', Position::new(0, 2));
        map.insert('<', Position::new(1, 0));
        map.insert('v', Position::new(1, 1));
        map.insert('>', Position::new(1, 2));
        
        Self {
            map,
            _kind: PhantomData,
        }
    }
}

impl Default for ButtonPad<DirectionPad> {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert('^', Position::NEG_X);
        map.insert('v', Position::X);
        map.insert('<', Position::NEG_Y);
        map.insert('>', Position::Y);
        
        
        Self {
            map,
            _kind: PhantomData,
        }
    }
}

pub struct Day21(Vec<String>);

impl std::ops::Deref for Day21 {
    type Target = Vec<String>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Day {}

#[derive(Hash, Eq, PartialEq, Clone)]
struct MoveKey {
    start: Position,
    end: Position,
    depth: usize,
    repeat: usize,
}

fn process(code: &str, cache: &mut HashMap<MoveKey, usize>, repeat: usize, depth: usize) -> usize {
    let buttons = ButtonPad::<NumberPad>::default();

    let null_button = if depth == 0 { Position::new(3, 0) } else { Position::ZERO };
    
    let start = if depth == 0 {
        buttons.get(&'A').unwrap()
    } else {
        buttons.get(&'a').unwrap()
    };

    let mut length = 0;
    let mut pos = start;

    for c in code.chars() {
        let next = buttons.get(&c).unwrap();
        let key = MoveKey {
            start: *pos,
            end: *next,
            depth,
            repeat,
        };

        if let Some(&cached) = cache.get(&key) {
            length += cached;
        } else {
            let moves = get_moves(*pos, *next, null_button);
            let min_len = if depth == repeat {
                moves[0].len()
            } else {
                moves.iter()
                    .map(|m| process(m, cache, repeat, depth + 1))
                    .min()
                    .unwrap_or(0)
            };
            cache.insert(key, min_len);
            length += min_len;
        }
        pos = next;

    }

    length
}

fn get_moves(from: Position, to: Position, skip: Position) -> Vec<String> {
    let directions = ButtonPad::<DirectionPad>::default();

    let delta = to - from;
    let mut moves = Vec::new();



    // Generate basic moves 
    let mut basic_moves = String::new();
    if delta.x < 0 {
        basic_moves.push_str(&"^".repeat((-delta.x) as usize));
    } else {
        basic_moves.push_str(&"v".repeat(delta.x as usize));
    }
    if delta.y < 0 {
        basic_moves.push_str(&"<".repeat((-delta.y) as usize));
    } else {
        basic_moves.push_str(&">".repeat(delta.y as usize));
    }

    // Generate permutations
    let mut seen = HashSet::new();
    recurse(&basic_moves.chars().collect::<Vec<_>>(), 0, &mut moves, &mut seen);

    // Filter invalid paths
    moves.retain(|path| {
        !path.chars()
            .map(|c| directions.get(&c).unwrap())
            .scan(from, |pos, dir| {
                *pos = *pos + *dir;
                Some(*pos)
            })
            .any(|pos| pos == skip)
    });

    if moves.is_empty() {
        moves.push("a".to_string());
    } else {
        moves = moves.into_iter().map(|s| s + "a").collect();
    }



    moves
}

/// creates a list of all possible permutations
fn recurse(chars: &[char], idx: usize, result: &mut Vec<String>, seen: &mut HashSet<String>) {
    // base case
    if idx == chars.len() {
        let s: String = chars.iter().collect();
        if seen.insert(s.clone()) {
            // no return creates natural backtracking
            result.push(s);
        }
    } else {
        for i in idx..chars.len() {
            let mut chars = chars.to_vec();
            chars.swap(idx, i);
            recurse(&chars, idx + 1, result, seen);
        }
    }
}

impl Solution for Day {
    type Output = usize;
    type Item = ();

    fn parse(input: &str) -> Self {
        Self(input.lines().map(str::to_string).collect())
    }

    fn part1(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut cache = HashMap::new();
        let mut output = 0;

        self.0.iter().for_each(|line| {
            // dbg!(&line);
            let numeric: usize = line[..3].parse().unwrap();
            let length = process(line, &mut cache, 2, 0);

            // dbg!(numeric);

            output += length * numeric
        });

        Ok(output)
    }

    fn part2(&mut self) -> miette::Result<Self::Output, AocError> {
        let mut cache = HashMap::new();
        let mut output = 0;

        self.0.iter().for_each(|line| {
            // dbg!(&line);
            let numeric: usize = line[..3].parse().unwrap();
            let length = process(line, &mut cache, 25, 0);

            // dbg!(numeric);

            output += length * numeric
        });

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", Day::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("154115708116294", Day::parse(input).solve(Part::Two)?);
        Ok(())
    }
}