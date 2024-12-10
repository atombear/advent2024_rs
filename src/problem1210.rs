use std::collections::HashSet;

use crate::utils::{pnum_from_file, process_input};

fn find_paths(th: (i64, i64), map: &Vec<Vec<i64>>, val: i64, path: &mut HashSet<(i64, i64)>, check_path: bool) -> i64 {
    if path.contains(&th) && check_path {
        return 0;
    } else if val == 9 {
        path.insert(th);
        return 1;
    } else {
        path.insert(th);
        let num_rows: i64 = map.len() as i64;
        let num_cols: i64 = map[0].len() as i64;

        let mut ret: i64 = 0;

        let (r, c) = th;

        for (nr, nc) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
            .iter()
            .filter(|(r, c)| r >= &0 && r < &num_rows && c >= &0 && c < &num_cols)
        {
            if map[*nr as usize][*nc as usize] == val + 1 {
                ret += find_paths((*nr, *nc), map, val + 1, path, check_path);
            }
        }

        return ret;
    }
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut map: Vec<Vec<i64>> = vec![];

    let process_line = |line: String| {
        map.push(line.chars().map(|x| x.to_digit(10).unwrap() as i64).collect());
    };
    process_input(problem_number, process_line);

    let mut theads: HashSet<(i64, i64)> = HashSet::new();

    let num_rows: usize = map.len();
    let num_cols: usize = map[0].len();

    for idx in 0..num_rows {
        for jdx in 0..num_cols {
            if map[idx][jdx] == 0 {
                theads.insert((idx as i64, jdx as i64));
            }
        }
    }

    let mut result0: i64 = 0;
    let mut result1: i64 = 0;

    let mut path: HashSet<(i64, i64)> = HashSet::new();
    for th in theads {
        result0 += find_paths(th, &map, 0, &mut path, true);
        result1 += find_paths(th, &map, 0, &mut path, false);
        path.drain();
    }

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
