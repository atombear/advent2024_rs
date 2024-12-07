use std::{iter::zip, path::PathBuf};

use crate::utils::read_lines;

fn process_line(s: String) -> (i64, Vec<i64>) {
    let mut i = s.split(':');
    let lhs: i64 = i.next().unwrap().parse().unwrap();
    let rhs: Vec<i64> = i
        .next()
        .unwrap()
        .split(' ')
        .filter(|x| x != &"")
        .map(|x| x.parse().unwrap())
        .collect();

    return (lhs, rhs);
}

fn int_concat(x: i64, y: i64) -> i64 {
    let mut x: i64 = x;
    let mut yc: i64 = y.clone();
    while yc > 0 {
        yc /= 10;
        x *= 10;
    }
    return x + y;
}

fn pattern_exists(lhs: i64, rhs: &Vec<i64>, carry: i64, idx: usize, check: bool) -> bool {
    if idx == rhs.len() {
        return carry == lhs;
    } else {
        return pattern_exists(lhs, rhs, rhs[idx] * carry, idx + 1, check)
            || pattern_exists(lhs, rhs, rhs[idx] + carry, idx + 1, check)
            || (pattern_exists(lhs, rhs, int_concat(carry, rhs[idx]), idx + 1, check) && check);
    }
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input7".to_string()].iter().collect();

    let mut lhss: Vec<i64> = vec![];
    let mut rhss: Vec<Vec<i64>> = vec![];

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(x) = line {
                let (lhs, rhs) = process_line(x);
                lhss.push(lhs);
                rhss.push(rhs);
            }
        }
    }
    let mut result0: i64 = 0;
    let mut result1: i64 = 0;
    for (lhs, rhs) in zip(lhss, rhss) {
        result0 += lhs * (pattern_exists(lhs, &rhs, rhs[0], 1, false) as i64);
        result1 += lhs * (pattern_exists(lhs, &rhs, rhs[0], 1, true) as i64);
    }
    return (6, format!("{}", result0), format!("{}", result1));
}
