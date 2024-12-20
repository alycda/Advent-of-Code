//! Day 9: Disk Fragmenter

use std::collections::HashMap;

use ornaments::Solution;

pub use crate::Day9 as Day;

pub struct Disk(HashMap<usize, usize>);

impl Disk {
    fn checksum(&self) -> u128 {
        self.iter()
            .map(|(loc, file_id)| (*loc as u128) * (*file_id as u128))  // Cast to u128 for multiplication
            .sum()
    }
}

impl std::ops::Deref for Disk {
    type Target = HashMap<usize, usize>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Disk Layout
#[derive(Debug, Clone)]
pub struct Day9 {
    files: Vec<FileBlock>,
    gaps: Vec<(usize, usize)>, // (start, size)
    total_length: usize,
}

#[derive(Debug, Clone, Copy)]
struct FileBlock {
    id: usize,
    size: usize,
    start_pos: usize,
}

impl Day9 {
    fn expand(input: &str) -> Self {
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
    
        Self {
            files,
            gaps,
            total_length: current_pos,
        }
    }

    fn fragment(&mut self) -> Disk {
        let mut disk: HashMap<usize, usize> = HashMap::new();
        
        // First, add files to the hashmap
        for file in &self.files {
            for loc in file.start_pos..file.start_pos + file.size {
                disk.insert(loc, file.id);
            }
        }

        let mut left = 0;
        let mut right = *disk.keys().max().unwrap_or(&0);
        
        while left < right {
            if let Some(file_id) = disk.remove(&right) {
                while disk.contains_key(&left) {
                    left += 1;
                }
                disk.insert(left, file_id);
            }
            right -= 1;
        }

        Disk(disk)
    }

    fn defrag(&mut self) -> Self {
        for i in (0..self.files.len()).rev() {
            let file = &self.files[i];
            let file_size = file.size;
            let file_start = file.start_pos;

            // Only look at gaps that are to the left of our current file
            let potential_gaps: Vec<_> = self.gaps.iter()
                .enumerate()
                .filter(|(_, (gap_start, _))| gap_start < &file_start)
                .collect();

            for (gap_idx, (gap_start, gap_size)) in potential_gaps {
                if gap_size >= &file_size {
                    self.files[i].start_pos = *gap_start;
                    
                    if gap_size == &file_size {
                        self.gaps.remove(gap_idx);
                    } else {
                        self.gaps[gap_idx] = (*gap_start + file_size, gap_size - file_size);
                    }
                    
                    self.gaps.push((file_start, file_size));
                    self.gaps.sort_by_key(|(start, _)| *start);
                    break;
                }
            }
        }
        
        self.clone()
    }

    fn checksum(&self) -> u128 {
        self.files.iter()
            .flat_map(|file| {
                (file.start_pos..file.start_pos + file.size)
                    .map(move |pos| (pos as u128) * (file.id as u128))
            })
            .sum()
    }

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

impl Solution for Day9 {
    type Output = u128;
    type Item = ();

    fn parse(input: &str) -> Self {
        Day9::expand(input)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, ornaments::AocError> {
        Ok(self.fragment().checksum())
    }

    fn part2(&mut self) -> miette::Result<Self::Output, ornaments::AocError> {
        Ok(self.defrag().checksum())
    }

    fn print(_input: &str) {
        print_disk_state(&Day9::expand("2333133121414131402"));
    }
}

// For debugging
fn print_disk_state(layout: &Day9) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;

    #[test]
    fn test_part1() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", Day9::parse(input).solve(Part::One)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", Day9::parse(input).solve(Part::Two)?);
        Ok(())
    }
}