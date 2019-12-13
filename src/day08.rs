

#[aoc(day08, part1, original)]
pub fn original_8a(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use day08::original_8a;
    use std::fs;
    const TEST_8A_1: i64 = 0;
    const ANSWER_8A: i64 = 3367126;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_8A,
            original_8a(&fs::read_to_string("input/2019/day8.txt").unwrap().trim())
        );
    }
}
