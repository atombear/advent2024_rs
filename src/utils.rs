use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
