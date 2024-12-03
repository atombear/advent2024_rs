use std::path::PathBuf;

use crate::utils::read_lines;

pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    // let data_path: PathBuf = [data_dir, "src".to_string(), "input7".to_string()].iter().collect();

    return (6, format!("{}", 0), format!("{}", 0));
}
