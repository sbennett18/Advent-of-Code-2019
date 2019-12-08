#[aoc(day05, part1, original)]
pub fn original_5a(input: &str) -> i32 {
    let mut prog: Vec<i32> = input
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    compute(&mut prog)
}

fn compute(mut prog: &mut Vec<i32>) -> i32 {
    let mut diag_code: i32 = 0;
    let mut ip: usize = 0;
    loop {
        let mut v: i32 = prog[ip];
        // println!("IP {} = {}", ip, v);
        let opcode = v % 100;
        v -= opcode;
        let param1_mode = (v % 1000) / 100;
        v -= param1_mode;
        let param2_mode = (v % 10000) / 1000;
        v -= param2_mode;
        let param3_mode = v / 10000;
        let param_modes: (i32, i32, i32) = (param1_mode, param2_mode, param3_mode);
        ip += match opcode {
            1 => op1_add(&mut prog, ip, param_modes),
            2 => op2_mul(&mut prog, ip, param_modes),
            3 => op3_input(&mut prog, ip, param_modes),
            4 => op4_output(&mut prog, ip, param_modes, &mut diag_code),
            99 => break,
            _ => panic!(),
        };
    }
    diag_code
}

fn parameter_mode(prog: &Vec<i32>, param: i32, mode: i32) -> i32 {
    match mode {
        0 => prog[param as usize], // Position mode
        1 => param,                // Immediate mode
        _ => panic!(),
    }
}

fn op1_add(prog: &mut Vec<i32>, ip: usize, param_modes: (i32, i32, i32)) -> usize {
    let v1 = parameter_mode(&prog, prog[ip + 1], param_modes.0);
    let v2 = parameter_mode(&prog, prog[ip + 2], param_modes.1);
    let addr3: usize = prog[ip + 3] as usize;
    prog[addr3] = v1 + v2;
    4
}

fn op2_mul(prog: &mut Vec<i32>, ip: usize, param_modes: (i32, i32, i32)) -> usize {
    let v1 = parameter_mode(&prog, prog[ip + 1], param_modes.0);
    let v2 = parameter_mode(&prog, prog[ip + 2], param_modes.1);
    let addr3: usize = prog[ip + 3] as usize;
    prog[addr3] = v1 * v2;
    4
}

const INPUT: i32 = 1;
fn op3_input(prog: &mut Vec<i32>, ip: usize, _param_modes: (i32, i32, i32)) -> usize {
    let addr = prog[ip + 1] as usize;
    prog[addr] = INPUT;
    2
}

fn op4_output(
    prog: &mut Vec<i32>,
    ip: usize,
    param_modes: (i32, i32, i32),
    diag_code: &mut i32,
) -> usize {
    let v = parameter_mode(&prog, prog[ip + 1], param_modes.0);
    *diag_code = v;
    println!("OUTPUT {}", v);
    2
}

#[aoc(day05, part2, original)]
pub fn original_5b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use day05::original_5a;
    use day05::original_5b;
    use std::fs;
    const ANSWER_5A: i32 = 5346030;
    const ANSWER_5B: i32 = 5741;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_5A,
            original_5a(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_5B,
            original_5b(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
        );
    }
}
