use crate::utils::{pnum_from_file, process_input};
use core::hash::Hash;
use std::{collections::HashMap, iter::zip};

fn extract_number_pair(s: String) -> (i64, i64) {
    let l: Vec<i64> = s
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x: &str| x.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    return (l[0], l[1]);
}

fn get_count_map<'a, T: Eq + PartialEq + Hash>(l: &'a Vec<T>) -> HashMap<&'a T, i64> {
    // this will return a map whose keys are references to the elements in the original list! no copies!
    let mut ret: HashMap<&T, i64> = HashMap::new();

    for i in l {
        if !ret.contains_key(&i) {
            ret.insert(&i, 0);
        }
        (*ret.get_mut(&i).unwrap()) += 1;
    }

    return ret;
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut l0: Vec<i64> = vec![];
    let mut l1: Vec<i64> = vec![];

    let process_line = |s: String| {
        let (x0, x1) = extract_number_pair(s);
        l0.push(x0);
        l1.push(x1);
    };

    process_input(problem_number, process_line);

    l0.sort();
    l1.sort();

    let result0: i64 = zip(&l0, &l1).map(|(x, y)| (x - y).abs()).sum();

    let d: HashMap<&i64, i64> = get_count_map(&l1);

    let result1: i64 = l0.iter().map(|x| x * d.get(x).unwrap_or(&0)).sum();

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
