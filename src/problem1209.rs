use crate::utils::{pnum_from_file, process_input};

fn defrag0(files: &Vec<(i64, i64)>, empty: &Vec<i64>) -> i64 {
    let mut files: Vec<(i64, i64)> = files.to_vec();
    let mut f_idx: usize = 0;
    let mut e_idx: usize = 0;
    let mut f_e_idx: usize = files.len() - 1;
    let mut result: i64 = 0;
    let mut loop_cnt: usize = 0;
    let mut idx: i64 = 0;
    loop {
        if loop_cnt % 2 == 0 {
            // consume from front memory
            let (l, cnt) = files[f_idx];
            for _ in 0..cnt {
                result += idx * l;
                idx += 1;
            }
            f_idx += 1
        } else {
            // consume from back memory
            for _ in 0..empty[e_idx] {
                let (l, mut cnt) = files[f_e_idx];
                result += idx * l;
                idx += 1;
                cnt -= 1;

                if cnt == 0 {
                    f_e_idx -= 1;
                } else {
                    files[f_e_idx] = (l, cnt);
                }
            }
            e_idx += 1;
        }

        loop_cnt += 1;

        if f_idx > f_e_idx {
            break;
        }
    }

    return result;
}

fn defrag1(all_locs: &Vec<(i64, i64)>) -> i64 {
    let mut all_locs: Vec<(i64, i64)> = all_locs.to_vec();

    // try to move each once
    let mut idx: usize = all_locs.len() - 1;
    while idx > 0 {
        let (l0, cnt0) = all_locs[idx];
        // try to move the memory at idx
        if l0 != -1 {
            // search until idx, ie, only left moves
            for jdx in 0..idx {
                let (l1, cnt1) = all_locs[jdx];
                // if the space is available, move the memory
                if l1 == -1 && cnt1 >= cnt0 {
                    // empty the right most memory
                    all_locs[idx] = (-1, cnt0);

                    // combine empty memory on the right
                    if idx + 1 != all_locs.len() && all_locs[idx + 1].0 == -1 {
                        all_locs[idx] = (-1, all_locs[idx].1 + all_locs[idx + 1].1);
                        all_locs[idx + 1] = (-1, 0);
                    }
                    let mut t: usize = idx;
                    while all_locs[t - 1].0 == -1 {
                        all_locs[t - 1] = (-1, all_locs[t].1 + all_locs[t - 1].1);
                        all_locs[t] = (-1, 0);
                        t -= 1;
                    }
                    //

                    // update empty memory count
                    all_locs[jdx] = (-1, all_locs[jdx].1 - cnt0);
                    // insert new memory
                    all_locs.insert(jdx, (l0, cnt0));
                    // update idx to account for insert
                    idx += 1;

                    break;
                }
            }
        }
        idx -= 1;
    }

    let mut result: i64 = 0;
    let mut midx: i64 = 0;
    for (l, cnt) in all_locs {
        for _ in 0..cnt {
            if l != -1 {
                result += l * midx;
            }
            midx += 1
        }
    }

    return result;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut inp: Vec<String> = vec![];

    let process_line = |line: String| inp.push(line);
    process_input(problem_number, process_line);

    let mut files: Vec<(i64, i64)> = vec![];
    let mut empty: Vec<i64> = vec![];
    let mut all_locs: Vec<(i64, i64)> = vec![];

    let mut num: i64;
    let mut fnum: i64 = 0;
    for (idx, c) in inp[0].chars().enumerate() {
        num = c.to_digit(10).unwrap() as i64;
        if idx % 2 == 0 {
            files.push((fnum, num));
            all_locs.push((fnum, num));
            fnum += 1;
        } else {
            empty.push(num);
            all_locs.push((-1, num));
        }
    }

    let result0: i64 = defrag0(&files, &empty);
    let result1: i64 = defrag1(&all_locs);

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
