use std::collections::HashMap;

use crate::utils::{pnum_from_file, process_input};

fn digit_count(x: i64) -> i64 {
    let mut x: i64 = x;
    let mut cnt: i64 = 0;
    while x > 0 {
        x /= 10;
        cnt += 1;
    }
    return cnt;
}

fn track_stones(stone: i64, cnt: i64, stones_map: &mut HashMap<(i64, i64), i64>) -> i64 {
    if cnt == 0 {
        return 1;
    } else {
        if !stones_map.contains_key(&(stone, cnt)) {
            let num_digits: i64 = digit_count(stone);

            let r: i64 = if stone == 0 {
                track_stones(1, cnt - 1, stones_map)
            } else if (num_digits % 2) == 0 {
                let mut left_stone: i64 = stone;
                let mut right_stone: i64 = 0;
                let mut pow: i64 = 1;
                for _ in 0..num_digits / 2 {
                    right_stone += pow * (left_stone % 10);
                    left_stone /= 10;
                    pow *= 10;
                }
                track_stones(left_stone, cnt - 1, stones_map) + track_stones(right_stone, cnt - 1, stones_map)
            } else {
                track_stones(2024 * stone, cnt - 1, stones_map)
            };
            stones_map.insert((stone, cnt), r);
        }

        return *stones_map.get(&(stone, cnt)).unwrap();
    }
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut stones: Vec<i64> = vec![];
    let process_line = |line: String| {
        stones = line
            .split(' ')
            .filter(|x| x != &"")
            .map(|x| x.parse().unwrap())
            .collect()
    };
    process_input(problem_number, process_line);

    let mut stones_map: HashMap<(i64, i64), i64> = HashMap::new();

    let mut result0: usize = 0;
    let mut result1: usize = 0;
    for stone in stones {
        result0 += track_stones(stone, 25, &mut stones_map) as usize;
        result1 += track_stones(stone, 75, &mut stones_map) as usize;
    }

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
