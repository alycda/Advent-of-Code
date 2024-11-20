pub mod custom_error;

pub mod part1;
pub mod part2;

fn hit_count(enemy: char) -> u32 {
    match enemy {
        // 'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}

fn hit_count_bonus(enemy: char, bonus: bool) -> u32 {
    let add = if bonus { 1 } else { 0 };

    hit_count(enemy) + add
}
