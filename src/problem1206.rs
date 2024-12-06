use std::{collections::HashSet, path::PathBuf};

use crate::utils::read_lines;

struct Guard {
    loc: (i64, i64),
    update: (i64, i64),
}

fn turn_update(v: (i64, i64)) -> (i64, i64) {
    match v {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!("UNKNOWN!"),
    }
}

fn turn_guard(g: &Guard) -> Guard {
    return Guard {
        loc: g.loc,
        update: turn_update(g.update),
    };
}

fn move_guard(g: &Guard) -> Guard {
    return Guard {
        loc: (g.loc.0 + g.update.0, g.loc.1 + g.update.1),
        update: g.update,
    };
}

fn is_loop(start: (i64, i64), obs: &HashSet<(i64, i64)>) -> bool {
    let mut r: i64;
    let mut c: i64;
    (r, c) = start;

    let mut path: (i64, i64, i64, i64);

    let mut paths: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    loop {
        // move up
        let new_row: i64 = *obs
            .iter()
            .filter(|(x, y)| y == &c && x < &r)
            .map(|(x, _)| x)
            .max()
            .unwrap_or(&-1);
        if new_row == -1 {
            return false;
        }
        path = (r, c, new_row + 1, c);
        if paths.contains(&path) {
            return true;
        };
        paths.insert(path);
        r = path.2;
        c = path.3;

        // move right
        let new_col: i64 = *obs
            .iter()
            .filter(|(x, y)| x == &r && y > &c)
            .map(|(_, y)| y)
            .min()
            .unwrap_or(&-1);
        if new_col == -1 {
            return false;
        }
        path = (r, c, r, new_col - 1);
        if paths.contains(&path) {
            return true;
        };
        paths.insert(path);
        r = path.2;
        c = path.3;

        // move down
        let new_row: i64 = *obs
            .iter()
            .filter(|(x, y)| y == &c && x > &r)
            .map(|(x, _)| x)
            .min()
            .unwrap_or(&-1);
        if new_row == -1 {
            return false;
        }
        path = (r, c, new_row - 1, c);
        if paths.contains(&path) {
            return true;
        };
        paths.insert(path);
        r = path.2;
        c = path.3;

        // move left
        let new_col: i64 = *obs
            .iter()
            .filter(|(x, y)| x == &r && y < &c)
            .map(|(_, y)| y)
            .max()
            .unwrap_or(&-1);
        if new_col == -1 {
            return false;
        }
        path = (r, c, r, new_col + 1);
        if paths.contains(&path) {
            return true;
        };
        paths.insert(path);
        r = path.2;
        c = path.3;
    }
}

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input6".to_string()].iter().collect();

    let mut num_cols: i64 = 0;
    let mut num_rows: i64 = 0;

    let mut obs_locs: HashSet<(i64, i64)> = HashSet::new();
    let mut gloc: (i64, i64) = (0, 0);

    if let Ok(lines) = read_lines(data_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(x) = line {
                num_cols = x.len() as i64;
                num_rows += 1;

                for (jdx, c) in x.chars().enumerate() {
                    if c == '#' {
                        obs_locs.insert((idx as i64, jdx as i64));
                    } else if c == '^' {
                        gloc = (idx as i64, jdx as i64);
                    }
                }
            }
        }
    }

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let mut guard = Guard {
        loc: gloc,
        update: (-1, 0),
    };
    visited.insert(guard.loc);

    let mut new_guard: Guard;
    let mut r: i64;
    let mut c: i64;
    loop {
        new_guard = move_guard(&guard);

        (r, c) = new_guard.loc;

        if obs_locs.contains(&new_guard.loc) {
            guard = turn_guard(&guard);
        } else if r == 0 || r == num_rows - 1 || c == 0 || c == num_cols - 1 {
            visited.insert(new_guard.loc);
            break;
        } else {
            visited.insert(new_guard.loc);
            guard = new_guard;
        }
    }

    let result0: usize = visited.len();

    let mut result1: i64 = 0;
    let mut new_loc: (i64, i64);
    for idx in 0..num_rows {
        for jdx in 0..num_cols {
            new_loc = (idx, jdx);
            if !obs_locs.contains(&new_loc) {
                obs_locs.insert(new_loc);
                result1 += if is_loop(gloc, &obs_locs) { 1 } else { 0 };
                obs_locs.remove(&new_loc);
            }
        }
    }

    return (5, format!("{}", result0), format!("{}", result1));
}
