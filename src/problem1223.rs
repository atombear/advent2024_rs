use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use itertools::Itertools;

use crate::utils::{pnum_from_file, print, process_input};

fn step_through_graph<T: Clone + PartialEq + Eq + Hash + Debug>(
    entrance: &T,
    g: &HashMap<T, HashSet<T>>,
    visited: &mut Vec<T>,
    depth: usize,
    max_depth: usize,
    cache: &mut Vec<HashSet<T>>,
) {
    if visited.len() == max_depth {
        if visited[0] == visited[visited.len() - 1] {
            let t: HashSet<T> = HashSet::from_iter(visited.iter().map(|x| x.clone()));
            if !cache.contains(&t) {
                cache.push(t);
            }
        }
    } else {
        for child in g.get(entrance).unwrap() {
            visited.push(child.clone());
            step_through_graph(child, g, visited, depth + 1, max_depth, cache);
            visited.pop();
        }
    }
}

fn is_clique<T: Hash + Eq + PartialEq>(c: &Vec<T>, g: &HashMap<T, HashSet<T>>) -> bool {
    for idx in 0..c.len() {
        for jdx in (0..c.len()).filter(|i| i != &idx) {
            if !g.get(&c[idx]).unwrap().contains(&c[jdx]) {
                return false;
            }
        }
    }
    return true;
}

fn update_bools(bools: &mut Vec<bool>) {
    let mut swap: bool = false;

    for idx in (1..bools.len()).rev() {
        if (bools[idx] == false) && (bools[idx - 1] == true) {
            bools[idx] = true;
            bools[idx - 1] = false;
            swap = true;
            break;
        }
    }

    if !swap {
        let false_count = bools.iter().map(|x| !x as usize).sum::<usize>();
        for idx in 0..bools.len() {
            bools[idx] = true;
        }
        for idx in 0..false_count + 1 {
            bools[idx] = false;
        }
        bools.reverse();
    }
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let mut conns: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_nodes: HashSet<String> = HashSet::new();
    let process_line = |line: String| {
        let mut t = line.split("-");
        let c0: String = t.next().unwrap().to_string();
        let c1: String = t.next().unwrap().to_string();

        for c in [&c0, &c1] {
            if !conns.contains_key(c) {
                conns.insert(c.to_string(), HashSet::new());
            }
        }

        assert!(c0 != c1);

        conns.get_mut(&c0).unwrap().insert(c1.clone());
        conns.get_mut(&c1).unwrap().insert(c0.clone());
        all_nodes.insert(c0.clone());
        all_nodes.insert(c1);
    };
    process_input(problem_number, process_line);

    let mut visited: Vec<String> = vec![];
    let mut cache: Vec<HashSet<String>> = vec![];
    for e in &all_nodes {
        visited.push(e.clone());
        step_through_graph(e, &conns, &mut visited, 0, 4, &mut cache);
        visited.pop();
    }

    let result0: i64 = cache
        .iter()
        .map(|s| s.iter().map(|s| if s.starts_with('t') { 1 } else { 0 }).sum::<i64>())
        .filter(|x| x != &0)
        .count() as i64;

    let mut c: Vec<String> = vec![];
    let mut children: Vec<&String>;
    let mut bools: Vec<bool>;
    let mut done: bool;
    let mut max_clique: Vec<Vec<String>> = vec![];
    for tn in all_nodes {
        c.drain(..);
        c.push(tn.clone());
        children = conns.get(&tn).unwrap().iter().collect::<Vec<&String>>();

        let l = children.len();

        bools = vec![];
        for _ in 0..l {
            bools.push(true);
        }

        done = false;

        while !done {
            for jdx in bools.iter().enumerate().filter(|(_, b)| **b).map(|(x, _)| x) {
                c.push(children[jdx].clone());
            }

            if is_clique(&c, &conns) {
                if max_clique.len() == 0 {
                    max_clique.push(c.clone());
                } else if max_clique[0].len() < c.len() {
                    max_clique[0] = c.clone();
                }
                break;
            }

            c.drain(1..);

            if bools.iter().all(|x| !x) {
                done = true;
            } else {
                update_bools(&mut bools);
            }
        }
    }

    let result1 = max_clique[0].iter().sorted().join(",");

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
