use crate::utils::{extract_number_vec, pnum_from_file, process_input};

fn check(v: &Vec<&i64>) -> bool {
    if v.len() < 2 {
        return true;
    } else if v[1] == v[0] {
        return false;
    } else {
        let sgn: bool = v[1] > v[0];
        let mut ret: bool = true;

        for idx in 0..v.len() - 1 {
            ret &= (v[idx + 1] - v[idx]).abs() >= 1;
            ret &= (v[idx + 1] - v[idx]).abs() <= 3;
            ret &= sgn == (v[idx + 1] > v[idx]);
        }

        return ret;
    }
}

fn bool_to_int(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

fn drop<'a, T>(v: &'a Vec<&T>, d: usize) -> Vec<&'a T> {
    return v
        .iter()
        .enumerate()
        .filter(|(idx, _)| idx != &d)
        .map(|(_, y)| *y)
        .collect();
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut result0: i64 = 0;
    let mut ch0: bool = false;
    let mut result1: i64 = 0;
    let mut ch1: bool = false;

    let process_line = |nums_str: String| {
        let bare_nums: Vec<i64> = extract_number_vec(nums_str);

        // vector of refs
        let nums: Vec<&i64> = bare_nums.iter().map(|x| x).collect();

        ch0 = check(&nums);
        result0 += bool_to_int(ch0);

        result1 += if ch0 {
            1
        } else {
            ch1 = (0..nums.len()).map(|idx| check(&drop(&nums, idx))).any(|x| x);
            bool_to_int(ch1)
        };
    };

    process_input(problem_number, process_line);

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
