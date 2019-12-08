#[aoc(day02, part1, original)]
pub fn original_2a(input: &str) -> u32 {
    let mut prog: Vec<u32> = input
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    prog[1] = 12;
    prog[2] = 2;
    compute(&mut prog)
}

fn compute(mut prog: &mut Vec<u32>) -> u32 {
    for i in (0..prog.len()).step_by(4) {
        println!("{} {}", i, prog[i]);
        match prog[i] {
            1 => op1_add(&mut prog, i),
            2 => op2_mul(&mut prog, i),
            99 => break,
            _ => panic!(),
        }
    }
    prog[0]
}

fn op1_add(prog: &mut Vec<u32>, i: usize) {
    println!("{} {} {}", prog[i + 1], prog[i + 2], prog[i + 3]);
    let addr1: usize = prog[i + 1] as usize;
    let addr2: usize = prog[i + 2] as usize;
    let addr3: usize = prog[i + 3] as usize;
    prog[addr3] = prog[addr1] + prog[addr2];
}

fn op2_mul(prog: &mut Vec<u32>, i: usize) {
    let addr1: usize = prog[i + 1] as usize;
    let addr2: usize = prog[i + 2] as usize;
    let addr3: usize = prog[i + 3] as usize;
    prog[addr3] = prog[addr1] * prog[addr2];
}

#[aoc(day02, part2, original)]
pub fn original_2b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use day02::original_2a;
    use day02::original_2b;
    use std::fs;
    const ANSWER_2A: u32 = 5290681;
    const ANSWER_2B: i32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_2A,
            original_2a(&fs::read_to_string("input/2019/day2.txt").unwrap().trim())
        );
        assert_eq!(
            ANSWER_2B,
            original_2b(&fs::read_to_string("input/2019/day2.txt").unwrap().trim())
        );
    }
}
