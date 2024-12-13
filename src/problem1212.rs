use std::collections::HashSet;

use crate::utils::{pnum_from_file, process_input};

type C = (i64, i64);

fn get_num_lines(pp: HashSet<(i64, i64, i64, i64)>) -> i64 {
    let mut pp: HashSet<(i64, i64, i64, i64)> = pp;

    let mut num_lines: i64 = 0;

    let mut r: i64;
    let mut r0: i64;
    let mut c: i64;
    let mut c0: i64;

    let mut idx: i64;
    while pp.len() > 0 {
        let rc = *pp.iter().next().unwrap();
        pp.remove(&rc);
        (r, r0, c, c0) = rc;

        if r == r0 {
            for tick in [-1, 1] {
                idx = tick;
                while pp.contains(&(r + idx, r0 + idx, c, c0)) {
                    pp.remove(&(r + idx, r0 + idx, c, c0));
                    idx += tick;
                }
            }
            num_lines += 1;
        }
        if c == c0 {
            for tick in [-1, 1] {
                idx = tick;
                while pp.contains(&(r, r0, c + idx, c0 + idx)) {
                    pp.remove(&(r, r0, c + idx, c0 + idx));
                    idx += tick;
                }
            }
            num_lines += 1;
        }
    }
    return num_lines;
}

fn count_fence(r: i64, c: i64, plot: &Vec<Vec<char>>, visited: &mut HashSet<C>) -> (i64, i64) {
    let num_rows: i64 = plot.len() as i64;
    let num_cols: i64 = plot[0].len() as i64;

    let mut to_visit: Vec<C> = vec![(r, c)];

    let mut area: i64 = 0;
    let mut perim: i64 = 0;
    let mut perim_points: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    let mut r: i64;
    let mut c: i64;
    let mut r0: i64;
    let mut c0: i64;
    let mut in_bounds: bool;
    while to_visit.len() > 0 {
        (r, c) = to_visit.pop().unwrap();
        visited.insert((r, c));

        area += 1;

        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            r0 = r + dr;
            c0 = c + dc;

            in_bounds = r0 >= 0 && r0 < num_rows && c0 >= 0 && c0 < num_cols;
            if !in_bounds || plot[r0 as usize][c0 as usize] != plot[r as usize][c as usize] {
                perim += 1;
                perim_points.insert((r, r0, c, c0));
            } else if in_bounds && !visited.contains(&(r0, c0)) && !to_visit.contains(&(r0, c0)) {
                to_visit.push((r0, c0));
            }
        }
    }

    return (area * perim, area * get_num_lines(perim_points));
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut plot: Vec<Vec<char>> = vec![];

    let process_line = |line: String| plot.push(line.chars().collect());
    process_input(problem_number, process_line);

    let mut visited: HashSet<C> = HashSet::new();

    let num_rows: usize = plot.len();
    let num_cols: usize = plot[0].len();

    let mut result0: i64 = 0;
    let mut result1: i64 = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if !visited.contains(&(i as i64, j as i64)) {
                let (r0, r1) = count_fence(i as i64, j as i64, &plot, &mut visited);
                result0 += r0;
                result1 += r1;
            }
        }
    }

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
