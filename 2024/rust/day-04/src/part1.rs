use crate::custom_error::AocError;

fn count_horizontal(line: &str) -> usize {
    line.matches("XMAS")
        // .inspect(|l| {dbg!(l);})
        .count() 
    + line.matches("SAMX")
        // .inspect(|l| {dbg!(l);})
        .count()
}

fn count_vertical(input: &str) -> usize {
    let matrix = input.lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<Vec<_>>>();

    let transposed = transpose(&matrix);
    transposed.iter().enumerate().map(|(_i, line)| {
        let line = line.iter().collect::<String>();

        count_horizontal(&line)
    }).sum::<usize>()
}

fn count_diagonal_ltr(input: &str) -> usize {
    input.lines().enumerate()
    .flat_map(|(i, original_line)| {
        // For each character in the original line, we'll create a diagonal
        original_line.char_indices().map(move |(char_pos, _)| {
            input.lines()
                .skip(i)  // Skip to current line
                .take(4)
                .enumerate()
                .filter_map(move |(idx, diagonal_line)| {
                    // Get character at the diagonal position
                    diagonal_line.chars().nth(char_pos + idx)
                })
                .collect::<String>()
        })
    })
    .map(|line| {
        count_horizontal(&line)
    })
    .sum::<usize>()
}

fn count_diagonal_rtl(input: &str) -> usize {
    let output = input.lines()
    .map(|line| {
        line.chars().rev().collect::<String>()
    })
    .collect::<Vec<_>>().join("\n");

    count_diagonal_ltr(&output)
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    
    // Pre-allocate the transposed matrix
    let mut transposed = vec![vec![matrix[0][0].clone(); rows]; cols];
    
    // Fill the transposed matrix
    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j].clone();
        }
    }

    transposed
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let horizontal_count = input
        .lines()
            .map(|line| {
            count_horizontal(line)
        })
        .sum::<usize>();

    let vertical_count = count_vertical(input);

    let diagonal_ltr_count = count_diagonal_ltr(input);
    let diagonal_rtl_count = count_diagonal_rtl(input);

    // dbg!(horizontal_count, vertical_count, diagonal_ltr_count, diagonal_rtl_count);

    let output = horizontal_count + vertical_count + diagonal_ltr_count + diagonal_rtl_count;

    Ok(output.to_string())
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
#[case("..XXMAS", "1")]
#[case("SAMXMS.", "1")]
#[case("MS.X", "0")]
#[case("XMASAMX.MM", "2")]
#[case("XMAS", "1")]
#[case("SAMX.MM", "1")]
#[case("...XA.A", "0")]
#[case(".X.X.XMASX", "1")]
#[case(".X.XMAS", "1")]
#[case("...XXMA
SAMXMS.
..S..A.
.A.A.MS", "2")]
#[case("X
M
A
S
A
M
X", "2")]
    fn test_cases(#[case] input: &'static str, #[case] expected: &str) {
        assert_eq!(process(input).unwrap(), expected);
    }
}
