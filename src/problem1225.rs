use std::iter::zip;

use crate::utils::{pnum_from_file, print, process_input};

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut keys: Vec<Vec<i64>> = vec![];
    let mut locks: Vec<Vec<i64>> = vec![];
    let mut buf: Vec<String> = vec![];

    let process_line = |line: String| {
        if line.len() == 0 {
            buf.drain(..);
        } else {
            buf.push(line);
        }

        let mut new_obj: Vec<i64> = vec![];
        if buf.len() == 7 {
            for idx in 0..buf[0].len() {
                new_obj.push(
                    buf.iter()
                        .map(|s| (s.chars().nth(idx).unwrap() == '#') as i64)
                        .sum::<i64>()
                        - 1,
                )
            }
            if buf[0].chars().nth(0).unwrap() == '.' {
                keys.push(new_obj);
            } else {
                locks.push(new_obj);
            }
        }
    };
    process_input(problem_number, process_line);

    let mut result0: i64 = 0;

    for k in &keys {
        for l in &locks {
            result0 += zip(k, l).map(|(n0, n1)| n0 + n1).all(|x| x <= 5) as i64;
        }
    }

    return (problem_number, format!("{}", result0), format!("{}", 0));
}
