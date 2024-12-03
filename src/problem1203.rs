use std::path::PathBuf;

use crate::utils::read_lines;

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [data_dir, "src".to_string(), "input3".to_string()].iter().collect();

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(x) = line {}
        }
    }

    return (2, format!("{}", 0), format!("{}", 0));
}
