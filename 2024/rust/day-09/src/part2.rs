use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy)]
struct FileBlock {
    id: usize,
    size: usize,
    start_pos: usize,
}

#[derive(Debug, Clone)]
struct DiskLayout {
    files: Vec<FileBlock>,
    gaps: Vec<(usize, usize)>, // (start, size)
    total_length: usize,
}

impl DiskLayout {

    // fn to_string(&self) -> String {
    //     let mut result = vec!['.'; self.total_length];
    //     for file in &self.files {
    //         for pos in file.start_pos..file.start_pos + file.size {
    //             result[pos] = char::from_digit(file.id as u32, 10).unwrap();
    //         }
    //     }
    //     result.into_iter().collect()
    // }

    fn to_string(&self) -> String {
        let mut result = vec!['.'; self.total_length];
        for file in &self.files {
            for pos in file.start_pos..file.start_pos + file.size {
                result[pos] = if file.id < 10 {
                    char::from_digit(file.id as u32, 10).unwrap()
                } else {
                    '#' 
                };
            }
        }
        result.into_iter().collect()
    }
}

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

fn defrag(mut layout: DiskLayout) -> DiskLayout {
    for i in (0..layout.files.len()).rev() {
        let file = &layout.files[i];
        let file_size = file.size;
        let file_start = file.start_pos;

        // Only look at gaps that are to the left of our current file
        let potential_gaps: Vec<_> = layout.gaps.iter()
            .enumerate()
            .filter(|(_, (gap_start, _))| gap_start < &file_start)
            .collect();

        for (gap_idx, (gap_start, gap_size)) in potential_gaps {
            if gap_size >= &file_size {
                layout.files[i].start_pos = *gap_start;
                
                if gap_size == &file_size {
                    layout.gaps.remove(gap_idx);
                } else {
                    layout.gaps[gap_idx] = (*gap_start + file_size, gap_size - file_size);
                }
                
                layout.gaps.push((file_start, file_size));
                layout.gaps.sort_by_key(|(start, _)| *start);
                break;
            }
        }
    }
    
    layout
}

fn checksum(layout: &DiskLayout) -> u128 {
    layout.files.iter()
        .flat_map(|file| {
            (file.start_pos..file.start_pos + file.size)
                .map(move |pos| (pos as u128) * (file.id as u128))
        })
        .sum()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let layout = expand(input);

    // print_disk_state(&layout);

    let defrag = defrag(layout.clone());

    // print_disk_state(&defrag);
    
    Ok(checksum(&defrag).to_string())
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
        // print_disk_state(&layout);
        assert_eq!(
            layout.to_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
