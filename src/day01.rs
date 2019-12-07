use std::cmp;

#[aoc(day01, part1, original)]
pub fn original_1a(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<f32>().unwrap())
        .map(|f| ((f / 3.0) as i32) - 2)
        .sum()
}

#[cfg(test)]
mod tests {
    use day01::original_1a;
    use std::fs;
    const ANSWER_1A: i32 = 0;

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_1A,
            original_1a(&fs::read_to_string("input/2019/day1.txt").unwrap().trim())
                        );
