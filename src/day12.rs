use regex::Regex;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<(i32, i32, i32)> {
    let re = Regex::new(r"<(\w+)=(-?\d+),?\s*(\w+)=(-?\d+),?\s*(\w+)=(-?\d+),?\s*>").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l.trim()).unwrap();
            (
                caps[2].parse::<i32>().unwrap(),
                caps[4].parse::<i32>().unwrap(),
                caps[6].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day12, part1, original)]
pub fn original_12a(input: &[(i32, i32, i32)]) -> u32 {
    for m in input {
        println!("{:?}", m);
    }
    0
}
