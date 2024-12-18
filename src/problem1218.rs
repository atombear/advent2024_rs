use std::collections::HashSet;

use crate::utils::{pnum_from_file, print, process_input};

type C = (i64, i64);

fn bfs(start: C, end: C, fallen: &HashSet<C>) -> i64 {
    let mut visited: HashSet<C> = HashSet::new();
    let mut to_visit: HashSet<C> = HashSet::from([start]);
    let mut to_visit_next: HashSet<C> = HashSet::new();
    let mut dist: i64 = -1;
    let mut found: bool = false;
    while !found {
        for pos in to_visit.drain() {
            visited.insert(pos);
            if pos == end {
                found = true;
                break;
            }

            for dp in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = (pos.0 + dp.0, pos.1 + dp.1);
                if new_pos.0 >= 0
                    && new_pos.0 <= end.0
                    && new_pos.1 >= 0
                    && new_pos.1 <= end.1
                    && !visited.contains(&new_pos)
                    && !fallen.contains(&new_pos)
                {
                    to_visit_next.insert(new_pos);
                }
            }
        }

        for p in to_visit_next.drain() {
            to_visit.insert(p);
        }

        dist += 1;

        if to_visit.len() == 0 && !found {
            dist = -1;
            break;
        }
    }

    return dist;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut all_bytes: Vec<C> = Vec::new();

    let process_line = |line: String| {
        let xy = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        all_bytes.push((xy[0], xy[1]));
    };
    process_input(problem_number, process_line);

    let num_fallen: i64 = 1024;

    let mut fallen: HashSet<C> = HashSet::new();
    for idx in 0..num_fallen {
        fallen.insert(all_bytes[idx as usize]);
    }

    let result0: i64 = bfs((0, 0), (70, 70), &fallen);

    // binary search by hand.
    let num_fallen1: i64 = 2862;

    let mut fallen1: HashSet<C> = HashSet::new();
    for idx in 0..num_fallen1 {
        fallen1.insert(all_bytes[idx as usize]);
    }

    assert!(bfs((0, 0), (70, 70), &fallen1) == -1);

    let last_byte: C = all_bytes[(num_fallen1 - 1) as usize];
    let result1 = format!("{},{}", last_byte.0, last_byte.1);

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
