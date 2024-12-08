use std::collections::{HashMap, HashSet};

use itertools::enumerate;

use crate::utils::{pnum_from_file, process_input};

fn in_bounds(r: i64, c: i64, num_rows: i64, num_cols: i64) -> bool {
    return r >= 0 && r < num_rows && c >= 0 && c < num_cols;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut ant_locs: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut all_ant_locs: HashSet<(i64, i64)> = HashSet::new();
    let mut r: i64 = 0;
    let mut num_cols: i64 = 0;

    let process_line = |line: String| {
        num_cols = line.len() as i64;

        for (c, t) in enumerate(line.chars()) {
            if !ant_locs.contains_key(&t) {
                ant_locs.insert(t, Vec::new());
            }
            ant_locs.get_mut(&t).unwrap().push((r, c as i64));
            all_ant_locs.insert((r, c as i64));
        }
        r += 1;
    };
    process_input(problem_number, process_line);
    ant_locs.remove(&'.');

    let num_rows: i64 = r;

    let mut all_antinodes: HashSet<(i64, i64)> = HashSet::new();
    let mut all_extend_antinodes: HashSet<(i64, i64)> = HashSet::new();

    let mut r: i64;
    let mut c: i64;
    let mut dr: i64;
    let mut dc: i64;

    for (_, a) in ant_locs.iter() {
        for idx in 0..a.len() - 1 {
            for jdx in (idx + 1)..a.len() {
                let (r0, c0) = a[idx];
                let (r1, c1) = a[jdx];

                all_antinodes.insert((-r1 + 2 * r0, -c1 + 2 * c0));
                all_antinodes.insert((-r0 + 2 * r1, -c0 + 2 * c1));

                dr = r1 - r0;
                dc = c1 - c0;

                r = r0;
                c = c0;
                while in_bounds(r, c, num_rows, num_cols) {
                    all_extend_antinodes.insert((r, c));
                    r += dr;
                    c += dc;
                }

                r = r0;
                c = c0;
                while in_bounds(r, c, num_rows, num_cols) {
                    all_extend_antinodes.insert((r, c));
                    r -= dr;
                    c -= dc;
                }
            }
        }
    }

    let result0: i64 = all_antinodes
        .iter()
        .filter(|(r, c)| in_bounds(*r, *c, num_rows, num_cols))
        .map(|_| 1)
        .sum();
    let result1: i64 = all_extend_antinodes
        .iter()
        .filter(|(r, c)| in_bounds(*r, *c, num_rows, num_cols))
        .map(|_| 1)
        .sum();

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
