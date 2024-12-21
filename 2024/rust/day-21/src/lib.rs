use std::collections::{HashMap, HashSet};

use miette::Diagnostic;
use thiserror::Error;

pub type Position = glam::IVec2;

pub fn get_all_paths(from: Position, to: Position, skip: Position) -> Vec<String> {
    let mut directions = HashMap::new();
        directions.insert('^', Position::NEG_X);
        directions.insert('v', Position::X);
        directions.insert('<', Position::NEG_Y);
        directions.insert('>', Position::Y);

    let mut moves = Vec::new();
    let signed_manhattan_distance = to - from;

    let mut basic_moves = String::new();
    if signed_manhattan_distance.x < 0 {
        basic_moves.push_str(&"^".repeat((-signed_manhattan_distance.x) as usize));
    } else {
        basic_moves.push_str(&"v".repeat(signed_manhattan_distance.x as usize));
    }
    if signed_manhattan_distance.y < 0 {
        basic_moves.push_str(&"<".repeat((-signed_manhattan_distance.y) as usize));
    } else {
        basic_moves.push_str(&">".repeat(signed_manhattan_distance.y as usize));
    }

    // Generate ALL permutations
    let mut seen = HashSet::new();
    recurse(&basic_moves.chars().collect::<Vec<_>>(), 0, &mut moves, &mut seen);

    dbg!(&moves);

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

    dbg!(moves)
}

fn recurse(chars: &[char], idx: usize, result: &mut Vec<String>, seen: &mut HashSet<String>) {
    if idx == chars.len() {
        let s: String = chars.iter().collect();
        if seen.insert(s.clone()) {
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

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

pub mod part1;
pub mod part2;