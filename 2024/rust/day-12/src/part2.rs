use crate::custom_error::AocError;

use std::collections::HashSet;
use glam::IVec2;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_position(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Down => IVec2::Y,
            Direction::Right => IVec2::X,
            Direction::Left => IVec2::NEG_X,
        }
    }    
}

impl<'a> IntoIterator for &'a Direction {
    type Item = Direction;
    type IntoIter = std::vec::IntoIter<Direction>;

    fn into_iter(self) -> Self::IntoIter {
        vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left].into_iter()
    }
}

const DIRECTIONS: [IVec2; 4] = [IVec2::Y, IVec2::NEG_Y, IVec2::X, IVec2::NEG_X];

fn explore(start: IVec2, grid: &Vec<Vec<char>>, seen: &mut HashSet<IVec2>) -> (usize, usize) {
    let target_char = grid[start.y as usize][start.x as usize];
    let mut stack = vec![start];
    let mut region = HashSet::new();
    let mut edges = HashSet::new();
    let mut area = 0;

    // First pass: collect region and edges
    while let Some(pos) = stack.pop() {
        if !region.insert(pos) {
            continue;
        }
        seen.insert(pos);
        area += 1;

        // for &(dx, dy) in DIRECTIONS.iter() {
        for &neighbor in DIRECTIONS.iter() {

            // let new_pos = IVec2::new(pos.x + dx, pos.y + dy);
            let new_pos = pos + neighbor;
            
            if in_bounds(new_pos, grid) && 
               grid[new_pos.y as usize][new_pos.x as usize] == target_char {
                if !region.contains(&new_pos) {
                    stack.push(new_pos);
                }
            } else {
                edges.insert(pos);
            }
        }
    }

    // Second pass: count unique sides
    let mut side_count = 0;
    // for &(dx, dy) in DIRECTIONS.iter() {
    for &neighbor in DIRECTIONS.iter() {
        let mut potential_side = HashSet::new();
        
        // Find potential sides in this direction
        for &pos in &region {
            // let peek = IVec2::new(pos.x + dx, pos.y + dy);
            let peek = pos + neighbor;
            if !region.contains(&peek) {
                potential_side.insert(peek);
            }
        }

        // Remove continuous edges
        let mut to_remove = HashSet::new();
        for &pos in &potential_side {
            let mut temp = IVec2::new(pos.x + neighbor.y, pos.y + neighbor.x);
            while potential_side.contains(&temp) {
                to_remove.insert(temp);
                temp = IVec2::new(temp.x + neighbor.y, temp.y + neighbor.x);
            }
        }

        side_count += potential_side.len() - to_remove.len();
    }

    (area, side_count)
}

fn in_bounds(pos: IVec2, grid: &Vec<Vec<char>>) -> bool {
    pos.x >= 0 && 
    pos.y >= 0 && 
    pos.y < grid.len() as i32 && 
    pos.x < grid[0].len() as i32
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let mut seen = HashSet::new();
    let mut total = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let pos = IVec2::new(col as i32, row as i32);

            if !seen.contains(&pos) {
                let (area, sides) = explore(pos, &grid, &mut seen);
                total += area * sides;
            }
        }
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
