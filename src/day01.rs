use std::cmp;

#[aoc(day01, part1, original)]
pub fn original_1a(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .map(|m| compute_fuel_required(m))
        .sum()
}

#[aoc(day01, part2, original)]
pub fn original_1b(input: &str) -> i32 {
    let modules = input.lines().map(|l| l.trim().parse::<i32>().unwrap());
    let mut total_fuel: i32 = 0;
    for mass in modules {
        let mut fuel = compute_fuel_required(mass);
        while fuel != 0 {
            total_fuel += fuel;
            fuel = compute_fuel_required(fuel);
        }
    }
    total_fuel
}

fn compute_fuel_required(mass: i32) -> i32 {
    cmp::max((mass / 3) - 2, 0)
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
