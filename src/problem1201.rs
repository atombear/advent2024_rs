use core::hash::Hash;
use std::{collections::HashMap, iter::zip, path::PathBuf};

use crate::utils::read_lines;

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
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input1".to_string()]
        .iter()
        .collect();

    let mut x0: i64;
    let mut x1: i64;

    let mut l0: Vec<i64> = vec![];
    let mut l1: Vec<i64> = vec![];

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_pair) = line {
                (x0, x1) = extract_number_pair(num_pair);
                l0.push(x0);
                l1.push(x1);
            }
        }
    }

    l0.sort();
    l1.sort();

    let result0: i64 = zip(&l0, &l1).map(|(x, y)| (x - y).abs()).sum();

    let d: HashMap<&i64, i64> = get_count_map(&l1);

    let result1: i64 = l0.iter().map(|x| x * d.get(x).unwrap_or(&0)).sum();

    return (0, format!("{}", result0), format!("{}", result1));
}
