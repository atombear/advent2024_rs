use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use itertools::{iproduct, Itertools};

use rayon::{
    self,
    iter::{ParallelBridge, ParallelIterator},
};

use crate::utils::{pnum_from_file, print, process_input};

type C = (i64, i64);

fn bfs_basic(start: &C, end: &C, path: &HashSet<C>) -> i64 {
    let mut steps: i64 = 0;

    let mut to_visit: HashSet<C> = HashSet::from([*start]);
    let mut to_visit_next: HashSet<C> = HashSet::new();
    let mut visited: HashSet<C> = HashSet::new();

    let mut new_pos: C;
    let mut found: bool = false;

    while to_visit.len() > 0 && !found {
        for pos in to_visit.drain() {
            if pos == *end {
                found = true;
                break;
            } else if !found {
                visited.insert(pos);
                for dpos in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    new_pos = (pos.0 + dpos.0, pos.1 + dpos.1);
                    if !visited.contains(&new_pos) && path.contains(&new_pos) {
                        to_visit_next.insert(new_pos);
                    }
                }
            }
        }

        if !found {
            swap(&mut to_visit, &mut to_visit_next);
            steps += 1;
        }
    }

    if found {
        return steps;
    } else {
        return 1000000;
    }
}

fn distance_from_end(end: &C, path: &HashSet<C>) -> HashMap<C, i64> {
    let mut ret: HashMap<C, i64> = HashMap::new();

    let mut steps: i64 = 0;

    let mut to_visit: HashSet<C> = HashSet::from([*end]);
    let mut to_visit_next: HashSet<C> = HashSet::new();
    let mut visited: HashSet<C> = HashSet::new();

    let mut new_pos: C;

    while to_visit.len() > 0 {
        for pos in to_visit.drain() {
            if !ret.contains_key(&pos) {
                ret.insert(pos, steps);
            }
            visited.insert(pos);
            for dpos in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                new_pos = (pos.0 + dpos.0, pos.1 + dpos.1);

                if !visited.contains(&new_pos) && path.contains(&new_pos) {
                    to_visit_next.insert(new_pos);
                }
            }
        }
        for p in to_visit_next.drain() {
            to_visit.insert(p);
        }
        steps += 1;
    }

    return ret;
}

fn count_paths(
    distances: &HashMap<C, i64>,
    path: &HashSet<C>,
    walls: &HashSet<C>,
    max_steps: i64,
    max_cheats: i64,
) -> i64 {
    let points: Vec<C> = distances
        .iter()
        .sorted_by_key(|(_, &d)| d)
        .map(|(p, _)| *p)
        .rev()
        .collect::<Vec<C>>();

    let path_length: i64 = *distances.values().max().unwrap();

    let new_walls: HashSet<C> = path.union(&walls).map(|x| *x).collect::<HashSet<C>>();
    let mut cnt: i64 = 0;
    let mut pos0: C;
    for idx in 0..points.len() - 1 {
        pos0 = points[idx];
        cnt += iproduct!(-max_cheats..max_cheats + 1, -max_cheats..max_cheats + 1)
            .par_bridge()
            .filter(|dp| dp.0.abs() + dp.1.abs() <= max_cheats)
            .map(|dp| (pos0.0 + dp.0, pos0.1 + dp.1))
            .filter(|p| path.contains(p) && (distances.get(p).unwrap() < distances.get(&pos0).unwrap()))
            .map(|p| {
                let cheat_dist: i64 = bfs_basic(&pos0, &p, &new_walls);
                let short_dist: i64 = path_length - distances.get(&pos0).unwrap() + distances.get(&p).unwrap();
                ((cheat_dist <= max_cheats) && (short_dist + cheat_dist <= max_steps)) as i64
            })
            .sum::<i64>()
    }

    return cnt;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut walls: HashSet<C> = HashSet::new();
    let mut path: HashSet<C> = HashSet::new();
    let mut start: C = (-1, -1);
    let mut end: C = (-1, -1);

    let mut row: i64 = 0;

    let process_line = |line: String| {
        for (col, c) in line.chars().enumerate() {
            let col: i64 = col as i64;
            if c == 'S' {
                start = (row, col);
            } else if c == 'E' {
                end = (row, col);
            } else if c == '.' {
                path.insert((row, col));
            } else if c == '#' {
                walls.insert((row, col));
            }
        }
        row += 1;
    };
    process_input(problem_number, process_line);
    path.insert(start);
    path.insert(end);

    let min_time: i64 = bfs_basic(&start, &end, &path);

    let distances: HashMap<C, i64> = distance_from_end(&end, &path);

    let result0: i64 = count_paths(&distances, &path, &walls, min_time - 100, 2);
    let result1: i64 = count_paths(&distances, &path, &walls, min_time - 100, 20);

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
