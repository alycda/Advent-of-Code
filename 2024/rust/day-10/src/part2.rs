use std::collections::HashMap;
use glam::IVec2;

#[derive(Debug)]
struct Grid {
    // HashMap mapping coordinates to heights
    heights: HashMap<IVec2, u32>,
    // Store dimensions for convenience
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut heights = HashMap::new();
        // Get dimensions from input
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        
        // Parse input string into grid
        for (x, line) in input.lines().enumerate() {
            for (y, height) in line.chars().enumerate() {
                if let Some(digit) = height.to_digit(10) {
                    heights.insert(IVec2::new(x as i32, y as i32), digit);
                }
            }
        }
        
        Self { heights, rows, cols }
    }

    // Find all positions containing 0
    fn get_trail_heads(&self) -> Vec<IVec2> {
        self.heights
            .iter()
            .filter(|&(_, &height)| height == 0)
            .map(|(&pos, _)| pos)
            .collect()
    }

    fn get_rating(&self, pos: IVec2, path: &mut Vec<IVec2>) -> usize {
        // Base case: found a 9 (reached a peak)
        if self.heights.get(&pos) == Some(&9) {
            return 1;  // Count this as one valid path
        }
    
        let current_height = *self.heights.get(&pos).unwrap();
        let mut total = 0;
    
        // Try all possible next steps
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_pos = IVec2::new(pos.x + dx, pos.y + dy);
            
            // Only follow paths that increase by exactly 1
            if let Some(&height) = self.heights.get(&next_pos) {
                if height == current_height + 1 && !path.contains(&next_pos) {
                    total += self.get_rating(next_pos, path);
                }
            }
        }
    
        total
    }

}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, ornaments::AocError> {
    let map = Grid::new(input);
    let mut total = 0;

    println!("\nInput grid:");
    for x in 0..map.rows {
        for y in 0..map.cols {
            print!("{}", map.heights.get(&IVec2::new(x as i32, y as i32)).unwrap());
        }
        println!();
    }

    for start_pos in map.get_trail_heads() {
        println!("\nStarting from zero at {:?}", start_pos);
        let paths = map.get_rating(start_pos, &mut Vec::new());
        println!("Found {} paths from this zero", paths);
        total += paths;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

//     use rstest::rstest;

//     #[rstest]
//     #[case("10..9..
// 2...8..
// 3...7..
// 4567654
// ...8..3
// ...9..2
// .....01", "2")]
//     fn test_cases(#[case] input: &str, #[case] expected: &str) {
//         assert_eq!(process(input).unwrap(), expected);
//     }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
