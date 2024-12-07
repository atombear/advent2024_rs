use std::collections::HashSet;

use crate::utils::{pnum_from_file, process_input};

fn parse_rule(s: String) -> (i64, i64) {
    let mut xy = s.split('|');
    return (
        xy.next().unwrap().parse::<i64>().unwrap(),
        xy.next().unwrap().parse::<i64>().unwrap(),
    );
}

fn parse_pages(s: String) -> Vec<i64> {
    return s.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
}

fn process_pages(pv: &Vec<i64>, rs: &HashSet<(i64, i64)>) -> i64 {
    for idx in 0..pv.len() - 2 {
        if !rs.contains(&(pv[idx], pv[idx + 1])) {
            return 0;
        }
    }
    return pv[pv.len() / 2];
}

fn process_and_swap_pages(pv: &mut Vec<i64>, rs: &HashSet<(i64, i64)>) -> i64 {
    for idx in 0..pv.len() - 1 {
        for jdx in (idx + 1)..(pv.len()) {
            if !rs.contains(&(pv[idx], pv[jdx])) {
                let t: i64 = pv[idx];
                pv[idx] = pv[jdx];
                pv[jdx] = t;
                return 0;
            }
        }
    }
    return pv[pv.len() / 2];
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut parse_rule_now: bool = true;

    let mut rules_vec: Vec<(i64, i64)> = vec![];
    let mut pages_vec: Vec<Vec<i64>> = vec![];

    let process_line = |x: String| {
        if x == "" {
            parse_rule_now = false;
        } else if parse_rule_now {
            rules_vec.push(parse_rule(x));
        } else {
            pages_vec.push(parse_pages(x));
        }
    };
    process_input(problem_number, process_line);

    let rules_set: HashSet<(i64, i64)> = rules_vec.iter().map(|x| *x).collect();

    let mut result0: i64 = 0;

    let mut bad_pages: Vec<Vec<i64>> = vec![];
    let mut score: i64;

    for pv in pages_vec {
        score = process_pages(&pv, &rules_set);

        if score == 0 {
            bad_pages.push(pv);
        } else {
            result0 += score;
        }
    }

    let mut result1: i64 = 0;

    for pv in bad_pages {
        let mut m_pv: Vec<i64> = pv.clone();
        score = process_pages(&mut m_pv, &rules_set);
        while score == 0 {
            score = process_and_swap_pages(&mut m_pv, &rules_set);
        }
        result1 += score;
    }

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
