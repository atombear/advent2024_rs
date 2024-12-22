use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    iter::zip,
    mem::swap,
};

use itertools::iproduct;

use crate::utils::{pnum_from_file, print, process_input};

type C = (i64, i64);

fn bfs_paths(start: C, end: C, points: &HashSet<C>) -> Vec<Vec<char>> {
    let mut to_visit: HashSet<(C, Vec<char>)> = HashSet::from([(start, vec![])]);
    let mut to_visit_next: HashSet<(C, Vec<char>)> = HashSet::new();
    let mut visited: HashSet<C> = HashSet::new();

    let mut new_pos: C;
    let mut found: bool = false;
    let mut paths: Vec<Vec<char>> = vec![];

    while to_visit.len() > 0 && !found {
        for (pos, path) in to_visit.drain() {
            if pos == end {
                found = true;
                paths.push(path);
            } else {
                visited.insert(pos);
                for (d, dpos) in zip(['>', '<', 'v', '^'], [(0, 1), (0, -1), (1, 0), (-1, 0)]) {
                    new_pos = (pos.0 + dpos.0, pos.1 + dpos.1);
                    let mut new_path = path.clone();
                    new_path.push(d);
                    if !visited.contains(&new_pos) && points.contains(&new_pos) {
                        to_visit_next.insert((new_pos, new_path));
                    }
                }
            }
        }

        swap(&mut to_visit, &mut to_visit_next);
    }

    return paths;
}

fn get_paths(c0: char, c1: char, k_cp: &HashMap<char, C>) -> Vec<Vec<char>> {
    let pos0: C = *k_cp.get(&c0).unwrap();
    let pos1: C = *k_cp.get(&c1).unwrap();

    let points: HashSet<C> = k_cp.values().map(|x| *x).collect::<HashSet<C>>();

    return bfs_paths(pos0, pos1, &points);
}

fn _buttons(
    start: char,
    end: char,
    k_cps: &Vec<&HashMap<char, C>>,
    cache_paths: &Vec<&HashMap<(char, char), Vec<Vec<char>>>>,
    max_idx: usize,
    idx: usize,
    cache: &mut HashMap<(char, char, usize), i64>,
) -> i64 {
    let this_idx: usize = if idx == 0 { 0 } else { 1 };

    if idx == max_idx {
        // the 1 represents the A push
        return 1 + cache_paths[this_idx].get(&(start, end)).unwrap()[0].len() as i64;
    } else {
        if !cache.contains_key(&(start, end, idx)) {
            let mut ret: i64 = 100000000000;
            let mut path_len: i64;
            let mut istart: char;
            let mut iend: char;
            for path in cache_paths[this_idx].get(&(start, end)).unwrap() {
                // each path must start on and end on A
                path_len = 0;
                for jdx in 0..path.len() {
                    if jdx == 0 {
                        istart = 'A';
                        iend = path[0];
                    } else {
                        istart = path[jdx - 1];
                        iend = path[jdx];
                    }
                    path_len += _buttons(istart, iend, k_cps, cache_paths, max_idx, idx + 1, cache)
                }
                // path length 0 means two consecutive pushes of the same type, eg, <<
                // requires an A push
                if path.len() == 0 {
                    path_len += 1;
                } else {
                    path_len += _buttons(path[path.len() - 1], 'A', k_cps, cache_paths, max_idx, idx + 1, cache);
                }

                ret = min(ret, path_len);
            }

            cache.insert((start, end, idx), ret);
        }
        return *cache.get(&(start, end, idx)).unwrap();
    }
}

fn buttons(
    keys_a: &Vec<char>,
    k_cps: &Vec<&HashMap<char, C>>,
    cache_paths: &Vec<&HashMap<(char, char), Vec<Vec<char>>>>,
    max_idx: usize,
) -> i64 {
    let mut ret: i64 = 0;
    let mut start: char;
    let mut key_a: char;
    let mut cache: HashMap<(char, char, usize), i64> = HashMap::new();
    for jdx in 0..keys_a.len() {
        if jdx == 0 {
            start = 'A';
            key_a = keys_a[0];
        } else {
            start = keys_a[jdx - 1];
            key_a = keys_a[jdx];
        }

        ret += _buttons(start, key_a, k_cps, cache_paths, max_idx, 0, &mut cache);
    }
    return ret;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let process_line = |line: String| {};
    process_input(problem_number, process_line);

    // alphanumeric
    let k_a_cp: HashMap<char, C> = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);

    // keypad
    let k_d_cp: HashMap<char, C> = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    let mut cache_paths_a: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    let mut cache_paths_d: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();

    for (c0, c1) in iproduct!(k_a_cp.keys(), k_a_cp.keys()) {
        cache_paths_a.insert((*c0, *c1), get_paths(*c0, *c1, &k_a_cp));
    }
    for (c0, c1) in iproduct!(k_d_cp.keys(), k_d_cp.keys()) {
        cache_paths_d.insert((*c0, *c1), get_paths(*c0, *c1, &k_d_cp));
    }

    let mut result0: i64 = 0;
    let mut result1: i64 = 0;
    let mut num: i64;
    // for str_val in ["029A", "980A", "179A", "456A", "379A"] {
    for str_val in ["319A", "985A", "340A", "489A", "964A"] {
        num = str_val.chars().take(3).collect::<String>().parse::<i64>().unwrap();
        result0 += num
            * (buttons(
                &str_val.chars().collect::<Vec<char>>(),
                &vec![&k_a_cp, &k_d_cp],
                &vec![&cache_paths_a, &cache_paths_d],
                2,
            ) as i64);
        result1 += num
            * (buttons(
                &str_val.chars().collect::<Vec<char>>(),
                &vec![&k_a_cp, &k_d_cp],
                &vec![&cache_paths_a, &cache_paths_d],
                25,
            ) as i64);
    }

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
