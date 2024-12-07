use crate::utils::{pnum_from_file, process_input};

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let process_line = |line: String| {};
    process_input(problem_number, process_line);

    return (problem_number, format!("{}", 0), format!("{}", 0));
}
