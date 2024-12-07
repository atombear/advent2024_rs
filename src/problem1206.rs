use std::collections::{HashMap, HashSet};

use crate::utils::{pnum_from_file, process_input};

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

fn is_loop(start: (i64, i64), row_obs: &HashMap<i64, HashSet<i64>>, col_obs: &HashMap<i64, HashSet<i64>>) -> bool {
    let mut r: i64;
    let mut c: i64;
    (r, c) = start;

    let mut path: (i64, i64, i64, i64);

    let mut paths: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    let mut d: i64 = 0;

    let mut new_row: &i64 = &0;
    let mut new_col: &i64 = &0;
    let empty: HashSet<i64> = HashSet::from([-1]);

    loop {
        path = match d % 4 {
            // move up
            0 => {
                new_row = col_obs
                    .get(&c)
                    .unwrap_or(&empty)
                    .iter()
                    .filter(|x| x < &&r)
                    .max()
                    .unwrap_or(&-1);
                (r, c, new_row + 1, c)
            }
            // move right
            1 => {
                new_col = row_obs
                    .get(&r)
                    .unwrap_or(&empty)
                    .iter()
                    .filter(|x| x > &&c)
                    .min()
                    .unwrap_or(&-1);
                (r, c, r, new_col - 1)
            }
            // move down
            2 => {
                new_row = col_obs
                    .get(&c)
                    .unwrap_or(&empty)
                    .iter()
                    .filter(|x| x > &&r)
                    .min()
                    .unwrap_or(&-1);
                (r, c, new_row - 1, c)
            }
            // move left
            3 => {
                new_col = row_obs
                    .get(&r)
                    .unwrap_or(&empty)
                    .iter()
                    .filter(|x| x < &&c)
                    .max()
                    .unwrap_or(&-1);
                (r, c, r, new_col + 1)
            }
            _ => panic!(),
        };

        if new_col == &-1 || new_row == &-1 {
            return false;
        }
        d += 1;

        if paths.contains(&path) {
            return true;
        };
        paths.insert(path);
        r = path.2;
        c = path.3;
    }
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut num_cols: i64 = 0;
    let mut num_rows: i64 = 0;

    let mut obs_locs: HashSet<(i64, i64)> = HashSet::new();
    let mut gloc: (i64, i64) = (0, 0);

    let mut idx: usize = 0;

    let process_line = |x: String| {
        num_cols = x.len() as i64;
        num_rows += 1;

        for (jdx, c) in x.chars().enumerate() {
            if c == '#' {
                obs_locs.insert((idx as i64, jdx as i64));
            } else if c == '^' {
                gloc = (idx as i64, jdx as i64);
            }
        }
        idx += 1;
    };
    process_input(problem_number, process_line);

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

    let mut row_obs: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut col_obs: HashMap<i64, HashSet<i64>> = HashMap::new();

    for (r, c) in &obs_locs {
        if !row_obs.contains_key(&r) {
            row_obs.insert(*r, HashSet::new());
        }
        if !col_obs.contains_key(&c) {
            col_obs.insert(*c, HashSet::new());
        }
        row_obs.get_mut(&r).unwrap().insert(*c);
        col_obs.get_mut(&c).unwrap().insert(*r);
    }

    let mut result1: i64 = 0;
    let mut new_loc: (i64, i64);
    for idx in 0..num_rows {
        for jdx in 0..num_cols {
            new_loc = (idx, jdx);
            if !obs_locs.contains(&new_loc) {
                row_obs.get_mut(&idx).unwrap().insert(jdx);
                col_obs.get_mut(&jdx).unwrap().insert(idx);

                result1 += is_loop(gloc, &row_obs, &col_obs) as i64;

                row_obs.get_mut(&idx).unwrap().remove(&jdx);
                col_obs.get_mut(&jdx).unwrap().remove(&idx);
            }
        }
    }

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
