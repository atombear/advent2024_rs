use crate::utils::{pnum_from_file, process_input};

fn scan_mem(m: &Vec<char>, track_do: bool) -> i64 {
    let mut j: usize;
    let mut num0: i64;
    let mut num1: i64;
    let mut first: bool;
    let mut ret: i64 = 0;
    let mut c: char;
    let mut enable: bool = true;
    for i in 0..m.len() - 4 {
        if (i + 3 < m.len()) && (m[i] == 'd') && (m[i + 1] == 'o') && (m[i + 2] == '(') && (m[i + 3] == ')') {
            enable = true;
        } else if (i + 6 < m.len())
            && (m[i] == 'd')
            && (m[i + 1] == 'o')
            && (m[i + 2] == 'n')
            && (m[i + 3] == '\'')
            && (m[i + 4] == 't')
            && (m[i + 5] == '(')
            && (m[i + 6] == ')')
        {
            enable = false;
        }

        num0 = 0;
        num1 = 0;
        first = true;
        if (m[i] == 'm') & (m[i + 1] == 'u') & (m[i + 2] == 'l') & (m[i + 3] == '(') && (enable || track_do) {
            j = i + 4;

            loop {
                c = m[j];

                if c == ')' {
                    ret += num0 * num1;
                    break;
                } else if c == ',' {
                    first = false;
                } else if !"0123456789".contains(c) {
                    break;
                } else if first {
                    num0 *= 10;
                    num0 += c.to_digit(10).unwrap() as i64;
                } else {
                    num1 *= 10;
                    num1 += c.to_digit(10).unwrap() as i64;
                }

                j += 1;
            }
        }
    }

    return ret;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut result0: i64 = 0;
    let mut result1: i64 = 0;
    let mut y: Vec<char> = vec![];

    let process_line = |x: String| {
        y.extend(x.chars());
    };
    process_input(problem_number, process_line);

    result0 += scan_mem(&y, true);
    result1 += scan_mem(&y, false);

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
