use crate::custom_error::AocError;

use std::collections::HashMap;

struct FileBlock {
    id: usize,
    size: usize,
    start_pos: usize,
}

struct DiskLayout {
    files: Vec<FileBlock>,
    gaps: Vec<(usize, usize)>, // (start, size)
    total_length: usize,
}

impl DiskLayout {
    // Helper for debugging - produces string representation like your original
    fn to_string(&self) -> String {
        let mut result = vec!['.'; self.total_length];
        for file in &self.files {
            for pos in file.start_pos..file.start_pos + file.size {
                result[pos] = char::from_digit(file.id as u32, 10).unwrap();
            }
        }
        result.into_iter().collect()
    }
}

// fn expand(input: &str) -> HashMap<usize, usize> {
//     let mut d: HashMap<usize, usize> = HashMap::new();  // Changed to usize
//     let mut current_position = 0;
//     let mut is_file = true;
//     let mut b = 0;

//     for c in input.chars() {
//         if let Some(length) = c.to_digit(10) {
//             let length = length as usize;
//             if is_file {
//                 // Add file
//                 for loc in 0..length {
//                     d.insert(current_position + loc, b);
//                 }
//                 b += 1;
//                 is_file = false;
//             } else {
//                 is_file = true;
//             }
//             current_position += length;
//         }
//     }

//     d
// }

fn expand(input: &str) -> DiskLayout {
    let mut files = Vec::new();
    let mut gaps = Vec::new();
    let mut current_pos = 0;
    let mut is_file = true;
    let mut file_id = 0;

    for c in input.chars() {
        if let Some(length) = c.to_digit(10) {
            let length = length as usize;
            
            if is_file {
                files.push(FileBlock {
                    id: file_id,
                    size: length,
                    start_pos: current_pos,
                });
                file_id += 1;
            } else {
                gaps.push((current_pos, length));
            }
            
            current_pos += length;
            is_file = !is_file;
        }
    }

    DiskLayout {
        files,
        gaps,
        total_length: current_pos,
    }
}

// For debugging
fn print_disk_state(layout: &DiskLayout) {
    println!("Disk layout: {}", layout.to_string());
    println!("Files:");
    for file in &layout.files {
        println!("  ID: {}, Size: {}, Position: {}", 
                file.id, file.size, file.start_pos);
    }
    println!("Gaps:");
    for (pos, size) in &layout.gaps {
        println!("  Position: {}, Size: {}", pos, size);
    }
}

fn rearrange(mut rearranged: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut left = 0;
    let mut right = *rearranged.keys().max().unwrap_or(&0);
    
    while left < right {
        if let Some(file_id) = rearranged.remove(&right) {
            while rearranged.contains_key(&left) {
                left += 1;
            }
            rearranged.insert(left, file_id);
        }
        right -= 1;
    }

    rearranged
}

fn checksum(expanded_rearranged: &HashMap<usize, usize>) -> u128 {
    expanded_rearranged.iter()
        .map(|(loc, file_id)| (*loc as u128) * (*file_id as u128))  // Cast to u128 for multiplication
        .sum()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut expanded = expand(input);

    todo!("refactoring");
    // expanded = rearrange(expanded.clone());
    
    // Ok(checksum(&expanded).to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;

    // #[rstest]
    // #[case("", "")]
    // fn test_cases(#[case] input: &str, #[case] expected: &str) {
    //     assert_eq!(process(input).unwrap(), expected);
    // }

    // #[test]
    // fn test_expand() -> miette::Result<()> {
    //     let input = "12345";

    //     dbg!(expand(input));
    //     panic!("stop");

    //     // assert_eq!("2858", process(input)?);
    //     Ok(())
    // }

    #[test]
    fn test_expand() {
        let input = "2333133121414131402";
        let layout = expand(input);
        print_disk_state(&layout);
        assert_eq!(
            layout.to_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    // #[test]
    // fn test_expand() -> miette::Result<()> {
    //     let input = "2333133121414131402";

    //     dbg!(expand(input));
    //     panic!("stop");

    //     // assert_eq!("2858", process(input)?);
    //     Ok(())
    // }

    // #[test]
    // fn test_process() -> miette::Result<()> {
    //     let input = "2333133121414131402";
    //     assert_eq!("2858", process(input)?);
    //     Ok(())
    // }
}
