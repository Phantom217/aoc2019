use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

use crossbeam::channel::{Receiver, Sender};
use crossbeam::thread;

use crate::computer::{Channel, ComputerMT, Rom};
use crate::error::Error;
use crate::utils::Vec2;

type Point = Vec2<i64>;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    // Part 1
    let rom = Rom::from_reader(reader)?;
    let robot = Robot::run(&rom, Color::Black)?;
    let answer1 = robot.grid.keys().count();

    // Part 2
    let robot = Robot::run(&rom, Color::White)?;
    let answer2 = robot.to_string();

    Ok((answer1.to_string(), answer2))
}

struct Robot {
    grid: HashMap<Point, Color>,
    location: Location,
}

impl Robot {
    fn run(rom: &Rom, color: Color) -> Result<Self, Error> {
        thread::scope(|s| {
            let mut robot = Self {
                grid: HashMap::default(),
                location: Location {
                    point: Point::new(0, 0),
                    direction: Direction::North,
                },
            };
            robot.grid.insert(robot.location.point, color);

            let (input, output) = (Channel::default(), Channel::default());
            let (sender, _) = input.clone().into_parts();
            let (_, receiver) = output.clone().into_parts();

            let handle = s.spawn(|_| {
                let mut computer = ComputerMT::new(rom, input, output);
                computer.run()?;

                Ok::<_, Error>(())
            });

            loop {
                if robot.step(&sender, &receiver)?.is_none() {
                    break;
                }
            }

            handle.join().unwrap()?;

            Ok(robot)
        })
        .unwrap()
    }

    fn step(
        &mut self,
        sender: &Sender<i64>,
        receiver: &Receiver<i64>,
    ) -> Result<Option<()>, Error> {
        let color = *self.grid.get(&self.location.point).unwrap_or(&Color::Black);
        if sender.send(color as i64).is_err() {
            return Ok(None);
        }

        let new_color = match receiver.recv() {
            Ok(color) => Color::try_from(color)?,
            Err(_) => return Ok(None),
        };
        self.grid.insert(self.location.point, new_color);

        let turn = match receiver.recv() {
            Ok(t) => Turn::try_from(t)?,
            Err(_) => return Ok(None),
        };
        self.location = self.location.next(turn);

        Ok(Some(()))
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_y = self.grid.keys().map(|p| p.y()).min().unwrap();
        let max_y = self.grid.keys().map(|p| p.y()).max().unwrap();
        let min_x = self.grid.keys().map(|p| p.x()).min().unwrap();
        let max_x = self.grid.keys().map(|p| p.x()).max().unwrap();

        let rows = (max_y - min_y) as usize + 1;
        let cols = (max_x - min_x) as usize + 1;

        let len = rows * (cols + 1);

        let mut buf = vec![b' '; len];

        for point in self
            .grid
            .iter()
            .filter(|(_, color)| **color == Color::White)
            .map(|(point, _)| point)
        {
            let x = (point.x() - min_x) as usize;
            let y = (point.y() - min_y) as usize;
            buf[(rows - 1 - y) * cols + x] = b'#';
        }

        for row in 0..(rows - 1) {
            buf[row * cols + cols - 1] = b'\n';
        }

        let s = String::from_utf8(buf).unwrap();

        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Color {
    Black = 0,
    White = 1,
}

impl TryFrom<i64> for Color {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let color = match value {
            0 => Color::Black,
            1 => Color::White,
            _ => bail!("Cannot parse `{}` into a `Color`", value),
        };

        Ok(color)
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next(&self, turn: Turn) -> Self {
        match (self, turn) {
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::North, Turn::Right) => Direction::East,
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::East, Turn::Left) => Direction::North,
            (Direction::East, Turn::Right) => Direction::South,
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::West, Turn::Right) => Direction::North,
        }
    }
}

struct Location {
    point: Point,
    direction: Direction,
}

impl Location {
    fn next(&self, turn: Turn) -> Self {
        let (x, y) = (self.point.x(), self.point.y());
        let next_direction = self.direction.next(turn);
        let next_point = match next_direction {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
        .into();

        Self {
            direction: next_direction,
            point: next_point,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Turn {
    Left = 0,
    Right = 1,
}

impl TryFrom<i64> for Turn {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let turn = match value {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => bail!("Cannot parse `{}` into a `Turn`", value),
        };

        Ok(turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_day11() {
        let expected2 = " ###  #  # #### ###   ##  #### ###  ###   \n #  # # #  #    #  # #  #    # #  # #  #  \n #  # ##   ###  #  # #  #   #  #  # #  #  \n ###  # #  #    ###  ####  #   ###  ###   \n #    # #  #    #    #  # #    # #  #     \n #    #  # #    #    #  # #### #  # #            ";

        utils::tests::test_full_problem(11, run, "2238", expected2);
    }
}
