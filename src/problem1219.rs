use std::collections::HashMap;

use crate::utils::{pnum_from_file, print, process_input};

fn check_towel(t: &String, patterns: &Vec<String>, idx: usize, cache: &mut HashMap<usize, bool>) -> bool {
    if idx == t.len() {
        return true;
    }

    if !cache.contains_key(&idx) {
        let mut can_make: bool = false;
        let mut jdx: usize = 0;
        let mut p: &String;
        while jdx < patterns.len() && !can_make {
            p = &patterns[jdx];
            if t.len() - idx >= p.len() && (t[idx..idx + p.len()] == *p) {
                can_make |= check_towel(t, patterns, idx + p.len(), cache);
            }
            jdx += 1;
        }
        cache.insert(idx, can_make);
    }

    return *cache.get(&idx).unwrap();
}

fn count_towel(t: &String, patterns: &Vec<String>, idx: usize, cache: &mut HashMap<usize, i64>) -> i64 {
    if idx == t.len() {
        return 1;
    }

    if !cache.contains_key(&idx) {
        let mut cnt: i64 = 0;
        let mut jdx: usize = 0;
        let mut p: &String;
        while jdx < patterns.len() {
            p = &patterns[jdx];
            if t.len() - idx >= p.len() && (t[idx..idx + p.len()] == *p) {
                cnt += count_towel(t, patterns, idx + p.len(), cache);
            }
            jdx += 1;
        }
        cache.insert(idx, cnt);
    }

    return *cache.get(&idx).unwrap();
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut patterns: Vec<String> = vec![];
    let mut towels: Vec<String> = vec![];
    let process_line = |line: String| {
        if line.contains(',') {
            for pat in line.split(", ").map(|x| x.to_string()) {
                patterns.push(pat);
            }
        } else if line.len() > 0 {
            towels.push(line.to_string());
        }
    };
    process_input(problem_number, process_line);

    let mut result0: i64 = 0;
    let mut cache: HashMap<usize, bool>;
    for t in &towels {
        cache = HashMap::new();
        result0 += check_towel(&t, &patterns, 0, &mut cache) as i64;
    }

    let mut result1: i64 = 0;
    let mut cnt_cache: HashMap<usize, i64>;
    for t in &towels {
        cnt_cache = HashMap::new();
        result1 += count_towel(&t, &patterns, 0, &mut cnt_cache) as i64;
    }

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
