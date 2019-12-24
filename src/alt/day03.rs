use std::collections::HashMap;
use std::convert::TryFrom;

use crate::error::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(i32, i32);

type State = HashMap<Point, [Option<u32>; 2]>;

// #[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let direction = match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'R' => Self::Right,
            'L' => Self::Left,
            _ => bail!("Unable to parse {} into a direction.", c),
        };

        Ok(direction)
    }
}

// #[derive(Copy, Clone, Debug)]
struct Instruction {
    dir: Direction,
    dist: u32,
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let dir = Direction::try_from(bytes[0] as char)?;
        let dist = atoi::atoi::<u32>(&bytes[1..])
            .ok_or_else(|| error!("Unable to parse {} into an instruction", s))?;

        Ok(Instruction { dir, dist })
    }
}

fn process_instruction(
    id: usize,
    steps: u32,
    origin: Point,
    instruction: &Instruction,
    state: &mut State,
) -> Point {
    let (i, j) = match instruction.dir {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Right => (1, 0),
        Direction::Left => (-1, 0),
    };

    let mut destination = origin;
    for n in 1..=instruction.dist {
        let point = Point(origin.0 + i * n as i32, origin.1 + j * n as i32);
        let value = state.entry(point).or_insert_with(|| [None, None]);

        if value[id].is_none() {
            value[id] = Some(steps + n);
        }

        destination = point;
    }

    destination
}

const ORIGIN: Point = Point(0, 0);

fn manhattan_distance(a: Point, b: Point) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    let mut state: State = HashMap::new();

    let mut id = 0;

    for res in input.lines() {
        assert!(id < 2);

        let line = res?;

        let mut point = ORIGIN;
        let mut steps = 0;

        for s in line.trim().split(',').map(|s| s.trim()) {
            let instruction = s.parse::<Instruction>()?;
            let new_point = process_instruction(id, steps, point, &instruction, &mut state);
            steps += instruction.dist;
            point = new_point;
        }

        id += 1;
    }

    let (answer1, answer2) = state
        .iter()
        .filter(|(_, v)| v[0].is_some() && v[1].is_some())
        .fold(
            (std::u32::MAX, std::u32::MAX),
            |(mut min_dist, mut min_steps), (point, array)| {
                let dist = manhattan_distance(*point, ORIGIN);
                if dist < min_dist {
                    min_dist = dist
                }

                let steps = array[0].unwrap() + array[1].unwrap();
                if steps < min_steps {
                    min_steps = steps;
                }

                (min_dist, min_steps)
            },
        );

    Ok((answer1.to_string(), answer2.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03() {
        let test_cases = &[
            // (input, expected1, expected2)
            ("R8,U5,L5,D3\nU7,R6,D4,L4", "6", "30"),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                "159",
                "610",
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                "135",
                "410",
            ),
        ];

        for (input, expected1, expected2) in test_cases {
            let reader = std::io::BufReader::new(input.as_bytes());
            let (actual1, actual2) = run(reader).unwrap();
            assert_eq!(&actual1, expected1);
            assert_eq!(&actual2, expected2);
        }

        crate::utils::tests::test_full_problem(3, run, "4981", "164012");
    }
}
