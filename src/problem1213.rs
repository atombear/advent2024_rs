use crate::utils::{pnum_from_file, print, process_input};

type C = (i64, i64);
const TOKEN_A: i64 = 3;
const TOKEN_B: i64 = 1;

fn get_tokens(x0: i64, y0: i64, x1: i64, y1: i64, x_prize: i64, y_prize: i64) -> i64 {
    let lhs: i64 = x1 * y0 - x0 * y1;
    let rhs: i64 = x_prize * y0 - y_prize * x0;

    let n: i64;
    let m_x0: i64;
    let m: i64;

    if rhs % lhs == 0 {
        n = rhs / lhs;
        m_x0 = x_prize - n * x1;
        if m_x0 % x0 == 0 {
            m = m_x0 / x0;
            return TOKEN_A * m + TOKEN_B * n;
        }
    }

    return 0;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut button_a: Vec<C> = vec![];
    let mut button_b: Vec<C> = vec![];
    let mut prize: Vec<C> = vec![];

    let process_line = |line: String| {
        if line.contains("Button A") {
            let mut xy = line.split("Button A: ").nth(1).unwrap().split(", ");
            let x = xy.next().unwrap().split("X+").nth(1).unwrap().parse::<i64>().unwrap();
            let y = xy.next().unwrap().split("Y+").nth(1).unwrap().parse::<i64>().unwrap();
            button_a.push((x, y));
        } else if line.contains("Button B: ") {
            let mut xy = line.split("Button B: ").nth(1).unwrap().split(", ");
            let x = xy.next().unwrap().split("X+").nth(1).unwrap().parse::<i64>().unwrap();
            let y = xy.next().unwrap().split("Y+").nth(1).unwrap().parse::<i64>().unwrap();
            button_b.push((x, y));
        } else if line.contains("Prize: ") {
            let mut xy = line.split("Prize: ").nth(1).unwrap().split(", ");
            let x = xy.next().unwrap().split("X=").nth(1).unwrap().parse::<i64>().unwrap();
            let y = xy.next().unwrap().split("Y=").nth(1).unwrap().parse::<i64>().unwrap();
            prize.push((x, y));
        }
    };
    process_input(problem_number, process_line);

    let error: i64 = 10000000000000;
    let mut result0: i64 = 0;
    let mut result1: i64 = 0;
    let mut x0: i64;
    let mut y0: i64;
    let mut x1: i64;
    let mut y1: i64;
    let mut x_prize: i64;
    let mut y_prize: i64;

    for idx in 0..prize.len() {
        (x0, y0) = button_a[idx];
        (x1, y1) = button_b[idx];
        (x_prize, y_prize) = prize[idx];

        result0 += get_tokens(x0, y0, x1, y1, x_prize, y_prize);
        result1 += get_tokens(x0, y0, x1, y1, x_prize + error, y_prize + error);
    }

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
