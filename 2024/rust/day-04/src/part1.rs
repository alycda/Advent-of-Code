use std::cmp::Ordering;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String, AocError> {
    let output = make_strings(input).iter().map(|v| v.iter().collect::<String>()).inspect(|s| {
        dbg!(s);
    })
    .filter_map(|s| {
        // let a = s.matches("XMAS").count();
        // let b = s.matches("SAMX").count();

        // dbg!((a, b));

        let found = find_xmas(&s);

        match found.cmp(&0) {
            Ordering::Greater => Some(found),
            _ => None
        }
    })
    .sum::<usize>();

    Ok((output).to_string())    
}

fn find_xmas(input: &str) -> usize {
    let sam = input.matches("XMAS").count();
    let max = input.matches("SAMX").count();

    (sam + max) as usize
}

fn make_strings(input: &str) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let size = grid.len();
    let mut new_strings = Vec::new();

    // add current lines
    new_strings.extend(grid.clone());

    // add columns
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
.X....", "4")]
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
    fn make_4_4() {
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

    #[test]
    fn make_10_10() {
        let input = "1234567890
ABCDEFGHIJ
KLMNOPQRST
UVWXYZ!@#$
%^&*()_+-=
[]|{};:',<
.>?/`~abcd
fghijklmno
pqrstuvwxy
zABCDEFGHI";
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

    #[rstest]
    #[case("XMAS", 1)]
    #[case("....XXMAS.", 1)]
    #[case("MMMSXXMASM", 1)]
    fn test_find_xmas(#[case] input: &'static str, #[case] expected: usize) {
        assert_eq!(find_xmas(input), expected);
    }
}
