pub mod custom_error;

pub mod part1;
pub mod part2;

fn unzip(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (nums[0], nums[1])
        })
        .unzip()
}
