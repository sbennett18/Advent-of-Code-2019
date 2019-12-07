use std::cmp;

#[aoc(day01, part1, original)]
pub fn original_1a(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .map(|f| cmp::max((f / 3) - 2, 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use day01::original_1a;
    use std::fs;
    const ANSWER_1A: i32 = 3367126;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_1A,
            original_1a(&fs::read_to_string("input/2019/day1.txt").unwrap().trim())
                        );
    }
}
