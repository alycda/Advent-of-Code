use std::cmp::Ordering;

pub mod custom_error;

pub mod part1;
pub mod part2;

fn line_to_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

/// The numbers are either all increasing or all decreasing.
/// Any two adjacent numbers differ by at least one and at most three.
fn is_safe(nums: Vec<i32>) -> bool {
    let mut safe = false;

    let nums2 = nums.clone();
    let mut last = 0;
    let mut direction = ' ';

    for n in nums {
        match direction {
            '+' => {
                if n > last {
                    // dbg!((last - n).abs());
                    match (last - n).abs() {
                        1..=3 => {
                            safe = true;
                        }
                        _ => {
                            safe = false;
                            break;
                        }
                    }
                } else {
                    safe = false;
                    break;
                }
            }
            '-' => {
                if n < last {
                    match (n - last).abs() {
                        1..=3 => {
                            safe = true;
                        }
                        _ => {
                            safe = false;
                            break;
                        }
                    }
                } else {
                    safe = false;
                    break;
                }
            }
            // first pass
            _ => match n.cmp(&nums2[1]) {
                Ordering::Greater => {
                    direction = '-';
                }
                Ordering::Less => {
                    direction = '+';
                }
                Ordering::Equal => {
                    safe = false;
                    break;
                }
            },
        }
        last = n;
    }

    safe
}
