use itertools::Itertools;

use crate::utils::{pnum_from_file, print};

#[derive(Debug, Clone)]
struct Program {
    A: i64,
    B: i64,
    C: i64,
    ops: Vec<i8>,
    out: Vec<i8>,
}

// Combo operands 0 through 3 represent literal values 0 through 3.
// Combo operand 4 represents the value of register A.
// Combo operand 5 represents the value of register B.
// Combo operand 6 represents the value of register C.
// Combo operand 7 is reserved and will not appear in valid programs.

// The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand.
// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.

// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.

// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.

// The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
// if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.

// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)

// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)

// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)

// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)

// 2,4 - A % 8 -> B
// 1,3 - B ^ 3 -> B
// 7,5 - A / 2**B -> C
// 4,7 - B ^ C -> B
// 0,3 - A / 8 -> A
// 1,5 - B ^ 5 -> B
// 5,5 - B % 8 -> out
// 3,0 - A !=0 -> 0

// 1,3 - (A % 8) ^ 3 -> B
// 7,5 - A / 2**B -> C
// 4,7 - B ^ C -> B
// 0,3 - A / 8 -> A
// 1,5 - B ^ 5 -> B
// 5,5 - B % 8 -> out
// 3,0 - A !=0 -> 0

fn get_combo_operand(program: &Program, op: i8) -> i64 {
    return match op {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => program.A,
        5 => program.B,
        6 => program.C,
        _ => panic!(),
    };
}

fn pow(x: i64, y: i64) -> i64 {
    let mut ret = 1;
    let mut y = y;
    while y > 0 {
        ret *= x;
        y -= 1;
    }
    return ret;
}

// 2,4,1,3,7,5,4,7,0,3,1,5,5,5,3,0
// 2,4 - A % 8 -> B
// 1,3 - B ^ 3 -> B
// 7,5 - A / 2**B -> C
// 4,7 - B ^ C -> B
// 0,3 - A / 8 -> A
// 1,5 - B ^ 5 -> B
// 5,5 - B % 8 -> out
// 3,0 - A !=0 -> 0
fn run_this_program(A: i64) -> Vec<i8> {
    let mut A: i64 = A;
    let mut B: i64 = 0;
    let mut ret: Vec<i8> = vec![];

    while A != 0 {
        // B = (A % 8) ^ (A / pow(2, (A % 8) ^ 3)) ^ 6;
        // B = (6 ^ (A & 7) ^ (A >> ((A & 7) ^ 3))) % 8;
        B = 6 ^ (A & 7) ^ ((A >> ((A & 7) ^ 3)) & 7);
        ret.push(B as i8);
        A = A / 8;
    }

    return ret;
}
// [0, 3, 5, 4, 3, 0]
fn run_this_other_program(A: i64) -> Vec<i8> {
    let mut A: i64 = A;
    let mut ret: Vec<i8> = vec![];

    while A != 0 {
        A = A / 8;
        ret.push((A % 8) as i8);
    }

    return ret;
}

fn weird(A: i64) -> i8 {
    return (6 ^ (A & 7) ^ ((A >> ((A & 7) ^ 3)) & 7)) as i8;
}

fn run_program(program: &mut Program) -> Vec<i8> {
    let mut idx: usize = 0;
    let mut did_jump: bool;
    let mut combo_operand: i64;
    let mut literal_operand: i64;
    while idx < program.ops.len() {
        did_jump = false;

        let op: i8 = program.ops[idx];

        match op {
            0 => {
                combo_operand = get_combo_operand(program, program.ops[idx + 1]);
                program.A = program.A / pow(2, combo_operand)
            }
            1 => {
                literal_operand = program.ops[idx + 1] as i64;
                program.B = program.B ^ literal_operand
            }
            2 => {
                combo_operand = get_combo_operand(program, program.ops[idx + 1]);
                program.B = combo_operand % 8
            }
            3 => {
                if program.A != 0 {
                    did_jump = true;
                    literal_operand = program.ops[idx + 1] as i64;
                    idx = literal_operand as usize;
                }
            }
            4 => program.B = program.B ^ program.C,
            5 => {
                combo_operand = get_combo_operand(program, program.ops[idx + 1]);
                program.out.push((combo_operand % 8) as i8);
            }
            6 => {
                combo_operand = get_combo_operand(program, program.ops[idx + 1]);
                program.B = program.A / pow(2, combo_operand)
            }
            7 => {
                combo_operand = get_combo_operand(program, program.ops[idx + 1]);
                program.C = program.A / pow(2, combo_operand)
            }
            _ => panic!(),
        }
        if !did_jump {
            idx += 2;
        }
    }

    return program.out.clone();
}

pub fn problem() -> (usize, String, String) {
    let problem_number: usize = pnum_from_file(file!());

    let p0: Program = Program {
        A: 0,
        B: 0,
        C: 9,
        ops: vec![2, 6],
        out: vec![],
    };
    let p1: Program = Program {
        A: 10,
        B: 0,
        C: 0,
        ops: vec![5, 0, 5, 1, 5, 4],
        out: vec![],
    };
    let p2: Program = Program {
        A: 2024,
        B: 0,
        C: 0,
        ops: vec![0, 1, 5, 4, 3, 0],
        out: vec![],
    };
    let p3: Program = Program {
        A: 0,
        B: 29,
        C: 0,
        ops: vec![1, 7],
        out: vec![],
    };
    let p4: Program = Program {
        A: 0,
        B: 2024,
        C: 43690,
        ops: vec![4, 0],
        out: vec![],
    };
    let p5: Program = Program {
        A: 729,
        B: 0,
        C: 0,
        ops: vec![0, 1, 5, 4, 3, 0],
        out: vec![],
    };

    // for mut p in [p0, p1, p2, p3, p4, p5] {
    //     print(&p);
    //     print(run_program(&mut p, vec![]));
    // }

    let program0: Program = Program {
        A: 52884621,
        B: 0,
        C: 0,
        ops: vec![2, 4, 1, 3, 7, 5, 4, 7, 0, 3, 1, 5, 5, 5, 3, 0],
        out: vec![],
    };

    let result0: String = run_program(&mut program0.clone())
        .iter()
        .map(|x| format!("{}", x))
        .join(",");

    let p6: Program = Program {
        A: 2024,
        B: 0,
        C: 0,
        ops: vec![0, 3, 5, 4, 3, 0],
        out: vec![],
    };

    let mut result1: i64 = 0;
    let digits: Vec<i8> = program0.ops.iter().rev().map(|x| *x).collect::<Vec<i8>>();
    let mut placeholder: Vec<i64> = digits.iter().map(|x| 0).collect();
    let mut continuing: Vec<bool> = digits.iter().map(|x| false).collect();
    let mut idx: usize = 0;
    while idx < digits.len() {
        let mut found: bool = false;
        let offset = if continuing[idx] { 1 } else { 0 };
        for jdx in (offset + placeholder[idx])..8 {
            if weird(result1 + jdx - placeholder[idx]) == digits[idx] {
                result1 -= placeholder[idx];
                result1 += jdx;
                placeholder[idx] = jdx;
                found = true;
                continuing[idx] = true;
                break;
            }
        }
        if found {
            result1 *= 8;
            idx += 1;
        } else {
            result1 /= 8;
            placeholder[idx] = 0;
            continuing[idx] = false;
            idx -= 1;
        }
    }
    result1 /= 8;

    return (problem_number, format!("{}", result0), format!("{}", result1));
}
