use std::fs::File;
use std::io::{self, BufRead};
use std::os::unix::process;
use std::path::{Path, PathBuf};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn extract_number_vec(s: String) -> Vec<i64> {
    return s
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x: &str| x.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}

pub fn process_input<F: FnMut(String)>(problem_number: usize, mut process_line: F) {
    let path: String = format!("input{}", problem_number + 1);
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), path].iter().collect();
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(x) = line {
                process_line(x)
            }
        }
    }
}

pub fn pnum_from_file(file: &str) -> usize {
    return file
        .split(".")
        .nth(0)
        .unwrap()
        .chars()
        .rev()
        .take(2)
        .map(|x| x.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(x, y)| 10_i32.pow(x as u32) as usize * y)
        .sum::<usize>()
        - 1;
}
