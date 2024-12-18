extern crate core;

mod problem1201;
mod problem1202;
mod problem1203;
mod problem1204;
mod problem1205;
mod problem1206;
mod problem1207;
mod problem1208;
mod problem1209;
mod problem1210;
mod problem1211;
mod problem1212;
mod problem1213;
mod problem1214;
mod problem1215;
mod problem1216;
mod problem1217;
mod problem1218;
mod problem1219;
mod problem1220;
mod problem1221;
mod problem1222;
mod problem1223;
mod problem1224;
mod problem1225;
mod utils;

fn process_answer(answers: &mut Vec<String>, idx: usize, ans0: String, ans1: String) {
    while answers.len() <= idx {
        answers.push("".to_string());
    }
    assert_eq!(answers[idx], "");
    answers[idx] = format!("{} {}", ans0, ans1);
}

fn main() {
    let mut answers: Vec<String> = vec![];

    // String, String
    for daily_fn in [
        problem1201::problem,
        problem1202::problem,
        problem1203::problem,
        problem1204::problem,
        problem1205::problem,
        problem1206::problem,
        problem1207::problem,
        problem1208::problem,
        problem1209::problem,
        problem1210::problem,
        problem1211::problem,
        problem1212::problem,
        problem1213::problem,
        problem1214::problem,
        problem1215::problem,
        // problem1216::problem,
        problem1217::problem,
        problem1218::problem,
        problem1219::problem,
        problem1220::problem,
        problem1221::problem,
        problem1222::problem,
        problem1223::problem,
        problem1224::problem,
        problem1225::problem,
    ] {
        let (idx, ans0, ans1) = daily_fn();
        process_answer(&mut answers, idx, ans0, ans1);
    }

    for (idx, ans) in answers.iter().enumerate() {
        println!("Day {} {}", idx + 1, ans);
    }
}
