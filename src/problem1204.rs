use std::{iter::zip, path::PathBuf};

use crate::utils::read_lines;

type WS = Vec<Vec<char>>;

fn get_num_rc<T>(v: &Vec<Vec<T>>) -> (usize, usize) {
    return (v.len(), v[0].len());
}

fn find_xmas(ws: &WS, i: usize, j: usize, s: Vec<(i64, i64)>, word: &str) -> bool {
    let (num_rows, num_cols) = get_num_rc(&ws);

    let mut idx: i64;
    let mut jdx: i64;

    let mut ret: bool = true;

    for ((o0, o1), c) in zip(s, word.chars()) {
        idx = o0 + i as i64;
        jdx = o1 + j as i64;

        if idx < 0 || idx >= num_rows as i64 || jdx < 0 || jdx >= num_cols as i64 {
            return false;
        } else {
            ret &= c == ws[idx as usize][jdx as usize];
        }
    }

    return ret;
}

fn bool_to_num(b: bool) -> i64 {
    return if b { 1 } else { 0 };
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input4".to_string()].iter().collect();

    let mut ws: Vec<Vec<char>> = vec![];

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(x) = line {
                ws.push(x.chars().collect());
            }
        }
    }

    let (num_rows, num_cols) = get_num_rc(&ws);

    let mut result0: i64 = 0;
    let mut result1: i64 = 0;

    for i in 0..num_rows {
        for j in 0..num_cols {
            for pattern in [
                vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                vec![(0, 0), (0, -1), (0, -2), (0, -3)],
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
                vec![(0, 0), (1, 1), (2, 2), (3, 3)],
                vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
                vec![(0, 0), (1, -1), (2, -2), (3, -3)],
                vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
            ] {
                result0 += bool_to_num(find_xmas(&ws, i, j, pattern, "XMAS"));
            }

            result1 += bool_to_num(
                (find_xmas(&ws, i, j, vec![(-1, -1), (0, 0), (1, 1)], "MAS")
                    || find_xmas(&ws, i, j, vec![(-1, -1), (0, 0), (1, 1)], "SAM"))
                    && (find_xmas(&ws, i, j, vec![(1, -1), (0, 0), (-1, 1)], "MAS")
                        || find_xmas(&ws, i, j, vec![(1, -1), (0, 0), (-1, 1)], "SAM")),
            );
        }
    }

    return (3, format!("{}", result0), format!("{}", result1));
}
