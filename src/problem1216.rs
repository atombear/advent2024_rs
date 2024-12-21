use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use crate::utils::{pnum_from_file, print, process_input};

const BIG_NUM: i64 = 1000000000000;

type C = (i64, i64);

fn rotate_left(dc: C) -> C {
    return match dc {
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        _ => panic!(),
    };
}

fn rotate_right(dc: C) -> C {
    return match dc {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => panic!(),
    };
}

fn min_path_bf(pos_dir: (C, C), end: C, maze: &HashSet<C>) -> i64 {
    let (pos, dir) = pos_dir;

    let mut scores: Vec<i64> = vec![];
    let mut new_pos: C;
    let mut visited: HashSet<C> = HashSet::new();
    let mut to_visit: Vec<(C, C, i64)> = vec![(pos, dir, 0)];
    let mut found: bool = false;

    while !found {
        let (pos, dir, score) = to_visit.pop().unwrap();
        if pos == end {
            found = true;
            scores.push(score);
        }

        visited.insert(pos);

        for new_dir in [dir, rotate_left(dir), rotate_right(dir)] {
            new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if maze.contains(&new_pos) && !visited.contains(&new_pos) {
                to_visit.push((new_pos, new_dir, score + 1 + if new_dir == dir { 0 } else { 1000 }));
            }
        }

        to_visit.sort_by_key(|(_, _, s)| -s);
    }

    return *scores.iter().min().unwrap();
}

fn djikstra(start: C, maze: &HashSet<C>, start_dir: C) -> HashMap<(C, C), i64> {
    let mut unvisited: HashMap<(C, C), i64> = HashMap::new();
    let mut visited: HashMap<(C, C), i64> = HashMap::new();
    for n in maze {
        for dc in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            unvisited.insert((*n, dc), BIG_NUM);
        }
    }
    unvisited.insert((start, start_dir), 0);

    let mut pos: C;
    let mut dir: C;
    let mut new_pos: C;
    let mut score: i64;

    while unvisited.len() > 0 {
        let temp = unvisited
            .iter()
            .map(|(x, y)| (*x, *y))
            .min_by_key(|(_, d)| d.clone())
            .unwrap();
        (pos, dir) = temp.0;
        score = temp.1;

        unvisited.remove(&temp.0);

        visited.insert((pos, dir), score);

        for new_dir in [dir, rotate_left(dir), rotate_right(dir)] {
            new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if unvisited.contains_key(&(new_pos, new_dir)) {
                let old_score: i64 = *unvisited.get(&(new_pos, new_dir)).unwrap();
                unvisited.insert(
                    (new_pos, new_dir),
                    min(old_score, score + 1 + if dir == new_dir { 0 } else { 1000 }),
                );
            }
        }
    }

    return visited;
}

fn trace_visited(pos_dir: (C, C), visited_se: &HashMap<(C, C), i64>, score: i64, all_visit: &mut HashSet<C>) {
    if visited_se.contains_key(&pos_dir) && visited_se.get(&pos_dir).unwrap() == &score {
        let (pos, dir) = pos_dir;
        all_visit.insert(pos);
        for new_dir in [dir, rotate_left(dir), rotate_right(dir)] {
            trace_visited(
                ((pos.0 - dir.0, pos.1 - dir.1), new_dir),
                visited_se,
                *visited_se.get(&pos_dir).unwrap() - 1 - if dir == new_dir { 0 } else { 1000 },
                all_visit,
            );
        }
    }
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut start: C = (0, 0);
    let mut end: C = (0, 0);
    let mut points: HashSet<C> = HashSet::new();

    let mut row: i64 = 0;

    let process_line = |line: String| {
        for (c, char) in line.chars().enumerate() {
            if char == '.' {
                points.insert((row, c as i64));
            } else if char == 'S' {
                start = (row, c as i64);
                points.insert((row, c as i64));
            } else if char == 'E' {
                end = (row, c as i64);
                points.insert((row, c as i64));
            }
        }
        row += 1;
    };
    process_input(problem_number, process_line);

    let result0: i64 = min_path_bf((start, (0, 1)), end, &points);

    let visited_se: HashMap<((i64, i64), (i64, i64)), i64> = djikstra(start, &points, (0, 1));

    let end_dir: C = visited_se
        .iter()
        .map(|(x, y)| (*x, *y))
        .filter(|(x, _)| x.0 == end)
        .min_by_key(|(_, x)| x.clone())
        .unwrap()
        .0
         .1;

    let mut all_visit: HashSet<C> = HashSet::new();
    trace_visited((end, end_dir), &visited_se, result0, &mut all_visit);

    let result1: i64 = all_visit.len() as i64;

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
