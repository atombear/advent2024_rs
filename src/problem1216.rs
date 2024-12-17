use std::collections::{HashMap, HashSet};

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

fn score_path(p: &Vec<(C, C)>) -> i64 {
    let mut ret: i64 = 0;

    for idx in 0..p.len() - 1 {
        if p[idx].1 == p[idx + 1].1 {
            ret += 1;
        } else {
            ret += 1000;
        }
    }
    return ret;
}

fn min_path_df(
    pos_dir: (C, C),
    end: C,
    maze: &HashSet<C>,
    visited: &mut HashSet<(C, C)>,
    score: i64,
    // min_score_box: &mut Vec<i64>,
    cache: &mut HashMap<(C, C), i64>,
    path: &mut Vec<(C, C)>,
) -> i64 {
    // if score > min_score_box[0] {
    //     return score;
    // }

    let (pos, dir) = pos_dir;

    let mut scores: Vec<i64> = vec![];
    let mut new_pos: C;

    if pos == end {
        // if score < min_score_box[0] {
        //     min_score_box[0] = score;
        // }
        // print(pos);
        // print(score_path(path));
        return score;
    } else if !cache.contains_key(&pos_dir) {
        for new_dir in [dir, rotate_left(dir), rotate_right(dir)] {
            new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if maze.contains(&new_pos) && !visited.contains(&(new_pos, (0, 0))) {
                visited.insert((new_pos, (0, 0)));
                path.push((new_pos, new_dir));
                scores.push(min_path_df(
                    (new_pos, new_dir),
                    end,
                    maze,
                    visited,
                    1 + if dir == new_dir { 0 } else { 1000 },
                    cache,
                    path,
                ));
                visited.remove(&(new_pos, (0, 0)));
                path.pop();
            }
        }
        // print(&pos_dir);
        // print(&scores);
        // print(&end);
        cache.insert(pos_dir, *scores.iter().min().unwrap_or(&BIG_NUM));
    }

    return *cache.get(&pos_dir).unwrap() + score;
}

fn search_maze_by_score(
    pos_dir: (C, C),
    end: C,
    maze: &HashSet<C>,
    visited: &mut HashSet<(C, C)>,
    path_points: &mut HashSet<C>,
    score: i64,
    max_score: i64,
) {
    let (pos, dir) = pos_dir;
    if score > max_score {
        return;
    } else if score == max_score && pos == end {
        for (c, _) in visited.iter() {
            path_points.insert(*c);
        }
    } else {
        let mut new_pos: C;
        for new_dir in [dir, rotate_left(dir), rotate_right(dir)] {
            new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if maze.contains(&new_pos) && !visited.contains(&(new_pos, new_dir)) {
                visited.insert((new_pos, new_dir));
                search_maze_by_score(
                    (new_pos, new_dir),
                    end,
                    maze,
                    visited,
                    path_points,
                    score + 1 + if dir == new_dir { 0 } else { 1000 },
                    max_score,
                );
                visited.remove(&(new_pos, new_dir));
            }
        }
    }
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

    // print(start);
    // print(end);
    // print(points);

    let mut visited: HashSet<(C, C)> = HashSet::from([(start, (0, 1))]);
    let mut cache: HashMap<(C, C), i64> = HashMap::new();

    // let mut min_score_box: Vec<i64> = vec![BIG_NUM];
    let result0 = min_path_bf((start, (0, 1)), end, &points);
    // let mut path: Vec<(C, C)> = vec![];
    // let result0: i64 = min_path_df((start, (0, 1)), end, &points, &mut visited, 0, &mut cache,&mut path);
    // let result0: i64 = min_path_df((end, (0, -1)), start, &points, &mut visited, 0, &mut cache,&mut path);

    let mut visited: HashSet<(C, C)> = HashSet::from([(start, (0, 1))]);
    let mut path_points: HashSet<C> = HashSet::new();
    // search_maze_by_score((start, (0, 1)), end, &points, &mut visited, &mut path_points, 0, result0);

    let result1: i64 = path_points.len() as i64;

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
