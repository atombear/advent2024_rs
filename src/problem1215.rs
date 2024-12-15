use std::{cmp::max, collections::HashSet};

use crate::utils::{pnum_from_file, print, process_input};

type C = (i64, i64);

fn part0(robot: C, dirs: &String, walls: &HashSet<C>, boxes: &HashSet<C>) -> i64 {
    let mut boxes: HashSet<C> = boxes.clone();

    let mut track_robot: C = robot.clone();
    let mut propose_robot: C;
    let mut new_box: C;
    let mut d_c: C;
    for c in dirs.chars() {
        d_c = match c {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!(),
        };

        propose_robot = (track_robot.0 + d_c.0, track_robot.1 + d_c.1);

        if walls.contains(&propose_robot) {
            // nothing, hit a wall
        } else if !boxes.contains(&propose_robot) {
            // free space
            track_robot = propose_robot;
        } else {
            new_box = propose_robot;
            while boxes.contains(&new_box) {
                new_box = (new_box.0 + d_c.0, new_box.1 + d_c.1);
            }
            if !walls.contains(&new_box) {
                track_robot = propose_robot;
                boxes.remove(&propose_robot);
                boxes.insert(new_box);
            }
        }
    }

    return boxes.iter().map(|(r, c)| 100 * r + c).sum::<i64>();
}

fn is_left(rc: &C, boxes: &HashSet<C>) -> bool {
    let (r, mut c) = rc;
    let mut cnt: i64 = 0;
    while boxes.contains(&(*r, c - 1)) {
        cnt += 1;
        c -= 1;
    }
    return cnt % 2 == 0;
}

fn find_stack(rc: &C, boxes: &HashSet<C>, d: &C, to_move: &mut HashSet<C>) {
    let (rp, cp) = (rc.0 + d.0, rc.1 + d.1);
    let neighbor: C;
    if boxes.contains(&(rp, cp)) && !to_move.contains(&(rp, cp)) {
        if is_left(&(rp, cp), boxes) {
            neighbor = (rp, cp + 1);
        } else {
            neighbor = (rp, cp - 1);
        }
        for c in [(rp, cp), neighbor] {
            to_move.insert(c);
            find_stack(&c, boxes, d, to_move);
        }
    }
}

fn part1(robot: C, dirs: &String, walls: &HashSet<C>, boxes: &HashSet<C>) -> i64 {
    let mut boxes: HashSet<C> = boxes.clone();

    let mut track_robot: C = robot.clone();
    let mut propose_robot: C;
    let mut new_box: C;
    let mut d_c: C;
    let mut to_move: HashSet<C>;
    for c in dirs.chars() {
        d_c = match c {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => panic!(),
        };

        propose_robot = (track_robot.0 + d_c.0, track_robot.1 + d_c.1);

        if walls.contains(&propose_robot) {
            // nothing, hit a wall
        } else if !boxes.contains(&propose_robot) {
            // free space
            track_robot = propose_robot;
        }
        // left and right should be the same.
        else if c == '<' || c == '>' {
            new_box = propose_robot;
            while boxes.contains(&new_box) {
                new_box = (new_box.0 + d_c.0, new_box.1 + d_c.1);
            }
            if !walls.contains(&new_box) {
                track_robot = propose_robot;
                boxes.remove(&propose_robot);
                boxes.insert(new_box);
            }
        } else {
            to_move = HashSet::new();
            find_stack(&track_robot, &boxes, &d_c, &mut to_move);
            // none of the boxes should move into walls!
            if !to_move
                .iter()
                .map(|(r, c)| walls.contains(&(r + d_c.0, c + d_c.1)))
                .any(|x| x)
            {
                for rc in &to_move {
                    boxes.remove(&rc);
                }
                for (r, c) in &to_move {
                    boxes.insert((r + d_c.0, c + d_c.1));
                }
                track_robot = propose_robot;
            }
        }
    }

    return boxes
        .iter()
        .filter(|rc| is_left(rc, &boxes))
        .map(|(r, c)| 100 * r + c)
        .sum::<i64>();
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut boxes: HashSet<C> = HashSet::new();
    let mut robot: C = (0, 0);
    let mut dirs_buf: Vec<String> = vec![];
    let mut walls: HashSet<C> = HashSet::new();

    let mut row: i64 = 0;
    let mut num_cols: i64 = 0;

    let mut moves_now: bool = false;
    let process_line = |line: String| {
        if line == "" || moves_now {
            moves_now = true;
            dirs_buf.push(line);
        } else if !moves_now {
            for (col, c) in line.chars().enumerate() {
                num_cols = max(col as i64, num_cols);

                if c == 'O' {
                    boxes.insert((row, col as i64));
                } else if c == '#' {
                    walls.insert((row, col as i64));
                } else if c == '@' {
                    robot = (row, col as i64);
                }
            }
            row += 1;
        }
    };
    process_input(problem_number, process_line);

    let dirs: String = dirs_buf.join("");

    let result0: i64 = part0(robot, &dirs, &walls, &boxes);

    let mut wide_walls: HashSet<C> = HashSet::new();
    for (r, c) in walls {
        wide_walls.insert((r, 2 * c));
        wide_walls.insert((r, 2 * c + 1));
    }

    let mut wide_boxes: HashSet<C> = HashSet::new();
    for (r, c) in boxes {
        wide_boxes.insert((r, 2 * c));
        wide_boxes.insert((r, 2 * c + 1));
    }

    let wide_robot: C = (robot.0, 2 * robot.1);

    let result1: i64 = part1(wide_robot, &dirs, &wide_walls, &wide_boxes);

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
