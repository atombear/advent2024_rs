use std::iter::zip;

use crate::utils::{pnum_from_file, process_input};

type C = (i64, i64);

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut pos: Vec<C> = vec![];
    let mut vel: Vec<C> = vec![];

    let process_line = |line: String| {
        let mut pv = line.split(" ");
        let xy: Vec<i64> = pv
            .next()
            .unwrap()
            .split("p=")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let vw: Vec<i64> = pv
            .next()
            .unwrap()
            .split("v=")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        pos.push((xy[0], xy[1]));
        vel.push((vw[0], vw[1]));
    };
    process_input(problem_number, process_line);

    let width: i64 = 101;
    let height: i64 = 103;

    let mut new_pos: Vec<C> = vec![];
    let mut new_x: i64;
    let mut new_y: i64;
    let mut quad_cnt: Vec<i64> = vec![0, 0, 0, 0];
    for ((x, y), (vx, vy)) in zip(&pos, &vel) {
        new_x = (x + 100 * vx) % width;
        new_y = (y + 100 * vy) % height;

        if new_x < 0 {
            new_x += width
        }
        if new_y < 0 {
            new_y += height;
        }
        new_pos.push((new_x, new_y));

        if new_x < (width - 1) / 2 {
            if new_y < (height - 1) / 2 {
                quad_cnt[0] += 1;
            } else if new_y > (height - 1) / 2 {
                quad_cnt[1] += 1;
            }
        } else if new_x > (width - 1) / 2 {
            if new_y < (height - 1) / 2 {
                quad_cnt[2] += 1;
            } else if new_y > (height - 1) / 2 {
                quad_cnt[3] += 1;
            }
        }
    }

    let result0: i64 = quad_cnt.iter().fold(1, |x, y| x * y);

    let mut result1: usize = 0;
    for sidx in 0..10000 {
        for idx in 0..pos.len() {
            let (x, y) = pos[idx];
            let (vx, vy) = vel[idx];
            new_x = (x + vx) % width;
            new_y = (y + vy) % height;

            if new_x < 0 {
                new_x += width
            }
            if new_y < 0 {
                new_y += height;
            }
            pos[idx] = (new_x, new_y);
        }

        let mut temp_pos: Vec<(i64, i64)> = pos.clone();
        temp_pos.sort();

        if (0..temp_pos.len() - 1)
            .map(|idx| temp_pos[idx] != temp_pos[idx + 1])
            .all(|x| x)
        {
            result1 = sidx;
            break;
        }
    }

    return (problem_number, format!("{}", result0), format!("{}", result1 + 1));
}
