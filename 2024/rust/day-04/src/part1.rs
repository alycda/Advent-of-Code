use std::cmp::Ordering;

use crate::custom_error::AocError;

// fn get_windows(lines: Lines<'_>) -> Vec<Vec<char>> {
//     let height = lines.clone().count();
//     let mut peekable = lines.clone().peekable();
//     peekable.peek();
//     let width = peekable.peek().unwrap().chars().count();

//     let _ = lines
//         .map(|line| line.chars().collect::<Vec<char>>())
//         .inspect(|x| {
//             dbg!(x);
//         })
//         .collect::<Vec<_>>();

//     vec![vec![' ']]
// }
// fn get_grid_windows(grid: &str) -> Vec<Vec<char>> {
//     let height = grid.lines().count();
//     let mut width: usize = 0;
//     let chars: Vec<char> = grid.lines().flat_map(|line| {
//         width = line.chars().count();
//         line.chars() 
//     }).collect();
//     let mut windows = Vec::new();

//     // Horizontal rows
//     for row in 0..width {
//         windows.push(chars[row * width..(row + 1) * width].to_vec());
//     }

//     // Vertical columns
//     for col in 0..height {
//         windows.push((0..height).map(|row| chars[row * height + col]).collect());
//     }

//     // Only the two main diagonals
//     windows.push((0..width).map(|i| chars[i * width + i]).collect());  // Top-left to bottom-right
//     windows.push((0..width).map(|i| chars[i * width + (width - 1 - i)]).collect());  // Top-right to bottom-left

//     windows
// }

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String, AocError> {
    let output = make_strings(input).iter().map(|v| v.iter().collect::<String>()).inspect(|s| {
        dbg!(s);
    })
    .filter_map(|s| {
        let a = s.matches("XMAS").count();
        let b = s.matches("SAMX").count();

        dbg!((a, b));

        match (a + b).cmp(&0) {
            Ordering::Greater => Some((a + b) as usize),
            _ => None
        }
    })
    .sum::<usize>();

    Ok((output*2).to_string())    
}

fn make_strings(input: &str) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let size = grid.len();
    let mut new_strings = Vec::new();

    // existing lines
    new_strings.extend(grid.clone());

    // Add columns
    for col in 0..size {
        let column: Vec<char> = (0..size)
            .map(|row| grid[row][col])
            .collect();
        new_strings.push(column);
    }

    // top-left to bottom-right
    let diagonal_nw_se: Vec<char> = (0..size)
        .map(|i| grid[i][i])
        .collect();
    new_strings.push(diagonal_nw_se);

    // top-right to bottom-left
    let diagonal_ne_sw: Vec<char> = (0..size)
        .map(|i| grid[i][size - 1 - i])
        .collect();
    new_strings.push(diagonal_ne_sw);

    new_strings
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("..X...
.SAMX.
.A..A.
XMAS.S
.X....", "3")]
#[case("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX", "18")]
#[case("....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX", "18")]
    fn test_cases(#[case] input: &'static str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }

    #[test]
    fn make_patterns() {
        let input = "1234
5678
90AB
CDEF";
        let expected = vec![
            vec!['1', '2', '3', '4'], 
            vec!['5', '6', '7', '8'], 
            vec!['9', '0', 'A', 'B'], 
            vec!['C', 'D', 'E', 'F'],
            vec!['1', '5', '9', 'C'],
            vec!['2', '6', '0', 'D'],
            vec!['3', '7', 'A', 'E'],
            vec!['4', '8', 'B', 'F'],
            vec!['1', '6', 'A', 'F'],
            vec!['4', '7', '0', 'C'],
        ];

        assert_eq!(make_strings(input), expected);
    }
}
