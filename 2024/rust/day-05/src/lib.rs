pub mod custom_error;

pub mod part1;
pub mod part2;

pub fn process_update(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let nums = line.split(',').collect::<Vec<&str>>();

            nums[nums.len()/2].parse::<usize>().unwrap() 
        })
        .sum::<usize>()
}