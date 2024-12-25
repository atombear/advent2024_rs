use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::{pnum_from_file, print, process_input};

type Inst = (String, String, String, String);

fn run_instructions(wires: &mut HashMap<String, bool>, instrs: &Vec<Inst>) -> i64 {
    let mut instr_idx: HashSet<usize> = HashSet::from_iter(0..instrs.len());

    let mut found_instr: bool;
    let mut idx_to_remove: usize = 0;
    while instr_idx.len() > 0 {
        found_instr = false;
        for idx in &instr_idx {
            let (x, op, y, r) = &instrs[*idx];
            if wires.contains_key(x) && wires.contains_key(y) {
                let val: bool = match op.as_str() {
                    "OR" => wires.get(x).unwrap() | wires.get(y).unwrap(),
                    "AND" => wires.get(x).unwrap() & wires.get(y).unwrap(),
                    "XOR" => wires.get(x).unwrap() ^ wires.get(y).unwrap(),
                    _ => panic!(),
                };
                wires.insert(r.to_string(), val);
                found_instr = true;
                idx_to_remove = *idx;
                break;
            }
        }
        if found_instr {
            instr_idx.remove(&idx_to_remove);
        }
    }

    let mut bits: Vec<bool> = wires
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, v)| (k.split("z").nth(1).unwrap().parse::<usize>().unwrap(), v))
        .sorted_by_key(|(k, _)| *k)
        .map(|(_, v)| *v)
        .collect::<Vec<bool>>();

    bits.reverse();

    let mut val: i64 = 0;
    for b in bits {
        val *= 2;
        val += b as i64;
    }

    return val;
}

fn prepare_numbers(x: i64, y: i64, digits: usize) -> HashMap<String, bool> {
    let mut ret: HashMap<String, bool> = HashMap::new();

    let mut x: i64 = x;
    let mut y: i64 = y;

    let mut labelx: String;
    let mut labely: String;

    for idx in 0..digits {
        labelx = format!("x{:02}", idx);
        labely = format!("y{:02}", idx);

        ret.insert(labelx, x % 2 == 1);
        ret.insert(labely, y % 2 == 1);

        x /= 2;
        y /= 2;
    }

    return ret;
}

fn _load_parents(
    p: &String,
    instrs: &mut Vec<(String, String, String, String)>,
    parents: &mut Vec<(String, String, String, String)>,
) {
    let mut found_idx: usize = 0;
    let mut found: bool = false;
    for (idx, instr) in instrs.iter().enumerate() {
        if instr.3 == *p {
            parents.push(instr.clone());
            // parents.push(y.clone());
            found = true;
            found_idx = idx;
        }
    }

    if found {
        let (x, _, y, _) = instrs[found_idx].clone();
        instrs.remove(found_idx);
        _load_parents(&x, instrs, parents);
        _load_parents(&y, instrs, parents);
    }
}

fn group_instrs(instrs: &Vec<(String, String, String, String)>) -> Vec<Vec<(String, String, String, String)>> {
    let mut ret: Vec<Vec<(String, String, String, String)>> = vec![];

    let mut instr_cp: Vec<Inst> = instrs.clone();

    let mut parents: Vec<(String, String, String, String)>;

    for nl in 0..46 {
        let parent: String = format!("z{:02}", nl);

        parents = vec![];
        _load_parents(&parent, &mut instr_cp, &mut parents);
        ret.push(parents.clone());
    }

    return ret;
}

fn swap_outputs(r0: &String, r1: &String, instrs: &Vec<Inst>) -> Vec<Inst> {
    let mut idx: usize = 0;
    let mut jdx: usize = 0;

    for (kdx, (_, _, _, r)) in instrs.iter().enumerate() {
        if r == r0 {
            idx = kdx
        }
        if r == r1 {
            jdx = kdx;
        }
    }

    let (x0, op0, y0, _) = instrs[idx].clone();
    let (x1, op1, y1, _) = instrs[jdx].clone();

    let mut ret: Vec<Inst> = instrs.clone();

    ret[idx] = (x1, op1, y1, r0.clone());
    ret[jdx] = (x0, op0, y0, r1.clone());

    return ret;
}

pub fn problem() -> (usize, String, String) {
    let mut wires: HashMap<String, bool> = HashMap::new();

    let mut instrs: Vec<Inst> = vec![];
    let mut do_instrs: bool = false;
    let problem_number: usize = pnum_from_file(file!());

    let process_line = |line: String| {
        if line.len() == 0 {
            do_instrs = true;
        } else if do_instrs {
            let mut lr = line.split(" -> ");
            let l: Vec<String> = lr
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let r: String = lr.next().unwrap().to_string();
            instrs.push((l[0].to_owned(), l[1].to_owned(), l[2].to_owned(), r));
        } else {
            let mut kv = line.split(": ");
            wires.insert(
                kv.next().unwrap().to_string(),
                kv.next().unwrap().parse::<i64>().unwrap() != 0,
            );
        }
    };
    process_input(problem_number, process_line);

    let result0: i64 = run_instructions(&mut wires.clone(), &instrs);

    // by inspection
    let swaps: Vec<(String, String)> = vec![
        ("z06".to_string(), "ksv".to_string()),
        ("z20".to_string(), "tqq".to_string()),
        ("nbd".to_string(), "kbs".to_string()),
        ("ckb".to_string(), "z39".to_string()),
    ];
    let result1: String = swaps.iter().map(|(x, y)| [x, y]).flatten().sorted().join(",");

    // let mut new_instrs = instrs.clone();

    // new_instrs = swap_outputs(&"z06".to_string(), &"ksv".to_string(), &new_instrs);
    // new_instrs = swap_outputs(&"z20".to_string(), &"tqq".to_string(), &new_instrs);
    // new_instrs = swap_outputs(&"nbd".to_string(), &"kbs".to_string(), &new_instrs);
    // new_instrs = swap_outputs(&"ckb".to_string(), &"z39".to_string(), &new_instrs);

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
