use regex::Regex;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CoordinateQuantity {
    x: i32,
    y: i32,
    z: i32,
}

impl CoordinateQuantity {
    fn compute_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Moon {
    position: CoordinateQuantity,
    velocity: CoordinateQuantity,
}

macro_rules! update_velocity {
    ($a:expr, $b:expr, $c:ident) => {
        $a.velocity.$c += if $a.position.$c < $b.position.$c {
            1
        } else if $a.position.$c > $b.position.$c {
            -1
        } else {
            0
        };
    };
}

macro_rules! update_moon_velocity {
    ($a:expr, $b:expr) => {
        update_velocity!($a, $b, x);
        update_velocity!($a, $b, y);
        update_velocity!($a, $b, z);
    };
}

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

const STEPS: usize = 1000;

#[aoc(day12, part1, original)]
pub fn original_12a(input: &[(i32, i32, i32)]) -> i32 {
    let mut bodies: Vec<Moon> = input
        .into_iter()
        .map(|t| Moon {
            position: CoordinateQuantity {
                x: t.0,
                y: t.1,
                z: t.2,
            },
            velocity: CoordinateQuantity { x: 0, y: 0, z: 0 },
        })
        .collect();

    for step in 1..=STEPS {
        for i in 0..bodies.len() {
            for j in 0..bodies.len() {
                if i == j {
                    continue;
                }
                update_moon_velocity!(bodies[i], bodies[j]);
            }
        }
        for mut a in &mut bodies {
            a.position.x += a.velocity.x;
            a.position.y += a.velocity.y;
            a.position.z += a.velocity.z;
            // println!("{}: {:?}", step, a);
        }
    }
    bodies
        .iter()
        .map(|b| b.position.compute_energy() * b.velocity.compute_energy())
        .sum()
}

#[cfg(test)]
mod tests {
    use day12::generator;
    use day12::original_12a;
    use std::fs;

    const ANSWER_12A: i32 = 8310;

    /*
        const PART1_TEST1_INPUT: &str = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";
        const PART1_TEST1_STEPS: usize = 10;
        const PART1_TEST1_OUTPUT: i32 = 179;
        */

    #[test]
    fn original() {
        assert_eq!(
            ANSWER_12A,
            original_12a(&generator(
                &fs::read_to_string("input/2019/day12.txt").unwrap().trim()
            ))
        );
    }
}
