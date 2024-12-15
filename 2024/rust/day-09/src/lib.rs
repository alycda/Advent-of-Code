use std::collections::HashMap;

use ornaments::Solution;

pub mod custom_error;

pub mod part2;

pub struct Day9(HashMap<usize, usize>);

impl std::ops::Deref for Day9 {
    type Target = HashMap<usize, usize>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Day9 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Day9 {
    fn expand(input: &str) -> Self {
        let mut d: HashMap<usize, usize> = HashMap::new();
        let mut current_position = 0;
        let mut is_file = true;
        let mut b = 0;
    
        for c in input.chars() {
            if let Some(length) = c.to_digit(10) {
                let length = length as usize;
                if is_file {
                    for loc in 0..length {
                        d.insert(current_position + loc, b);
                    }
                    b += 1;
                    is_file = false;
                } else {
                    is_file = true;
                }
                current_position += length;
            }
        }
    
        Self(d)
    }

    fn rearrange(&mut self) -> Self {
        let mut left = 0;
        let mut right = *self.keys().max().unwrap_or(&0);
        
        while left < right {
            if let Some(file_id) = self.remove(&right) {
                while self.contains_key(&left) {
                    left += 1;
                }
                self.insert(left, file_id);
            }
            right -= 1;
        }

        Day9(self.clone())
    }

    fn checksum(&self) -> u128 {
        self.iter()
            .map(|(loc, file_id)| (*loc as u128) * (*file_id as u128))  // Cast to u128 for multiplication
            .sum()
    }
}

impl Solution for Day9 {
    type Output = u128;
    type Item = ();

    fn parse(input: &str) -> Self {
        Day9::expand(input)
    }

    fn part1(&mut self) -> miette::Result<Self::Output, ornaments::AocError> {
        Ok(self.rearrange().checksum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ornaments::Part;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", Day9::parse(input).solve(Part::One)?);
        Ok(())
    }
}