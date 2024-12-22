use std::collections::HashMap;

use crate::utils::{pnum_from_file, print, process_input};

fn update_secret(x: i64) -> i64 {
    let mut x: i64 = x;
    let mut y: i64 = x * 64;
    x = (x ^ y) % 16777216;

    y = x / 32;
    x = (x ^ y) % 16777216;

    y = x * 2048;
    x = (x ^ y) % 16777216;

    return x;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut nums: Vec<i64> = vec![];
    let process_line = |line: String| {
        nums.push(line.parse::<i64>().unwrap());
    };
    process_input(problem_number, process_line);

    // let nums: Vec<i64> = vec![123];
    // let nums: Vec<i64> = vec![1, 2, 3, 2024];

    let steps: usize = 2000;

    let mut result0: i64 = 0;
    let mut prices: Vec<Vec<i64>> = vec![];
    for num in &nums {
        let mut t: Vec<i64> = vec![];
        let mut x: i64 = *num;
        t.push(x % 10);
        for _ in 0..steps {
            x = update_secret(x);
            t.push(x % 10);
        }
        result0 += x;
        prices.push(t);
    }

    let mut diff_maps: Vec<HashMap<(i64, i64, i64, i64), i64>> = vec![];
    for t in prices {
        let mut dm: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        for idx in 4..steps {
            let d: (i64, i64, i64, i64) = (
                t[idx - 4 + 1] - t[idx - 4],
                t[idx - 3 + 1] - t[idx - 3],
                t[idx - 2 + 1] - t[idx - 2],
                t[idx - 1 + 1] - t[idx - 1],
            );
            if !dm.contains_key(&d) {
                dm.insert(d, t[idx]);
            }
        }
        diff_maps.push(dm);
    }

    let mut all_prices: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for dm in diff_maps {
        for (k, v) in dm.iter() {
            if !all_prices.contains_key(&k) {
                all_prices.insert(*k, 0);
            }
            *all_prices.get_mut(k).unwrap() += v;
        }
    }

    let result1: i64 = *all_prices.values().max().unwrap();

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
