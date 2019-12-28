use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use crate::error::Error;
use crate::utils::Vec3;

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
)))]
use self::normal::Moon;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
use self::simd::Moon;

const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    let mut moons = parse_input(reader)?;

    for _ in 0..1000 {
        moons.step()
    }

    let answer1 = moons.energy();

    Ok((answer1.to_string(), String::new()))
}

fn parse_input<R>(reader: R) -> Result<Moons, Error>
where
    R: std::io::BufRead,
{
    // safety: This is safe because the code below ensures that by the time we
    // would ever try to touch the moons array, all values inside will contain
    // specific values that we have written to it.
    let mut moons: [RefCell<Moon>; 4] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    let mut count = 0;
    for res in reader.lines() {
        if count > 3 {
            bail!("Can only support exactly 4 moons.");
        }

        let line = res?;
        let line = line.trim();

        let mut pos: [i64; 3] = [0i64; 3];
        let mut count_coord = 0;

        for dim in line.split(',') {
            let coord = dim
                .split('=')
                .nth(1)
                .ok_or_else(|| error!("Failed to parse line into a Moon: {:?}", line))?
                .chars()
                .take_while(|&c| c != '>')
                .collect::<String>()
                .parse::<i64>()?;

            pos[count_coord] = coord;
            count_coord += 1;
        }

        if count_coord != 3 {
            bail!("Found {} coordinate(s), but need 3.", count_coord);
        }

        let moon = Moon::new(pos, Vec3::default());
        moons[count] = RefCell::new(moon);
        count += 1;
    }

    if count != 4 {
        bail!("Can only suport exactly 4 moons.");
    }

    Ok(Moons(moons))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Moons([RefCell<Moon>; 4]);

impl Moons {
    fn energy(&self) -> u64 {
        let mut total = 0;

        for moon in self.iter() {
            let moon = moon.borrow();
            let mut potential = 0;
            let mut kinetic = 0;

            for k in 0..3 {
                potential += moon.pos()[k].abs() as u64;
                kinetic += moon.vel()[k].abs() as u64;
            }

            total += potential * kinetic;
        }

        total
    }
}

impl Deref for Moons {
    type Target = [RefCell<Moon>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Moons {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
)))]
mod normal {
    use super::*;

    impl Moons {
        pub(crate) fn step(&mut self) {
            for (i, j) in PAIRS.iter() {
                let moon_i = self.0.get(*i).unwrap();
                let moon_j = self.0.get(*j).unwrap();

                for k in 0..3 {
                    let pos_i = moon_i.borrow().pos()[k];
                    let pos_j = moon_j.borrow().pos()[k];

                    if pos_i < pos_j {
                        moon_i.borrow_mut().vel_mut()[k] += 1;
                        moon_j.borrow_mut().vel_mut()[k] -= 1;
                    } else if pos_i > pos_j {
                        moon_i.borrow_mut().vel_mut()[k] -= 1;
                        moon_j.borrow_mut().vel_mut()[k] += 1;
                    }
                }
            }

            for moon in self.iter_mut() {
                for k in 0..3 {
                    let vel = moon.borrow().vel()[k];
                    moon.borrow_mut().pos_mut()[k] += vel;
                }
            }
        }
    }

    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
    pub(crate) struct Moon {
        pub(crate) pos: Vec3<i64>,
        pub(crate) vel: Vec3<i64>,
    }

    impl Moon {
        pub(crate) fn new<V, U>(pos: V, vel: U) -> Self
        where
            V: Into<Vec3<i64>>,
            U: Into<Vec3<i64>>,
        {
            Self {
                pos: pos.into(),
                vel: vel.into(),
            }
        }

        pub(crate) fn pos(&self) -> &Vec3<i64> {
            &self.pos
        }

        pub(crate) fn pos_mut(&mut self) -> &mut Vec3<i64> {
            &mut self.pos
        }

        pub(crate) fn vel(&self) -> &Vec3<i64> {
            &self.vel
        }

        pub(crate) fn vel_mut(&mut self) -> &mut Vec3<i64> {
            &mut self.vel
        }
    }
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
mod simd {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref ONE: __m256i = unsafe { _mm256_set_epi64x(1, 1, 1, 0) };
        static ref NEGATIVE_ONE: __m256i = unsafe { _mm256_set_epi64x(-1, -1, -1, 0) };
    }
    impl Moons {
        pub(crate) fn step(&mut self) {
            for (i, j) in PAIRS.iter() {
                let moon_i = self.0.get(*i).unwrap();
                let moon_j = self.0.get(*j).unwrap();

                let pos_i = moon_i.borrow().pos;
                let pos_j = moon_j.borrow().pos;

                // Adding
                let mask_gt = unsafe { _mm256_cmpgt_epi64(pos_i, pos_j) };
                let operand_add = unsafe { _mm256_and_si256(mask_gt, *NEGATIVE_ONE) };

                // Subtracting
                let mask_lt = unsafe { _mm256_cmpgt_epi64(pos_j, pos_i) };
                let operand_sub = unsafe { _mm256_and_si256(mask_lt, *ONE) };

                let operand = unsafe { _mm256_or_si256(operand_add, operand_sub) };

                let mut moon_ref = moon_i.borrow_mut();
                let vel_ref = moon_ref.vel_mut();
                *vel_ref = unsafe { _mm256_add_epi64(*vel_ref, operand) };

                let mut moon_ref = moon_j.borrow_mut();
                let vel_ref = moon_ref.vel_mut();
                *vel_ref = unsafe { _mm256_sub_epi64(*vel_ref, operand) };
            }

            for moon in self.iter_mut() {
                let new_pos = {
                    let moon = moon.borrow();
                    unsafe { _mm256_add_epi64(moon.pos, moon.vel) }
                };
                let mut moon = moon.borrow_mut();
                *moon.pos_mut() = new_pos;
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub(crate) struct Moon {
        pos: __m256i,
        vel: __m256i,
    }

    impl PartialEq for Moon {
        fn eq(&self, other: &Moon) -> bool {
            self.pos() == other.pos() && self.vel() == other.vel()
        }
    }

    impl Eq for Moon {}

    impl Moon {
        pub(crate) fn new<V, U>(pos: V, vel: U) -> Self
        where
            V: Into<Vec3<i64>>,
            U: Into<Vec3<i64>>,
        {
            let pos = {
                let pos = pos.into();
                unsafe { _mm256_set_epi64x(pos.x(), pos.y(), pos.z(), 0) }
            };
            let vel = {
                let vel = vel.into();
                unsafe { _mm256_set_epi64x(vel.x(), vel.y(), vel.z(), 0) }
            };

            Self { pos, vel }
        }

        pub(crate) fn pos(&self) -> Vec3<i64> {
            self.pos.into()
        }

        pub(crate) fn pos_mut(&mut self) -> &mut __m256i {
            &mut self.pos
        }

        pub(crate) fn vel(&self) -> Vec3<i64> {
            self.vel.into()
        }

        pub(crate) fn vel_mut(&mut self) -> &mut __m256i {
            &mut self.vel
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils;

    #[test]
    fn test_day12() {
        let test_cases_part_one = &[
            // input, steps, expected
            (
                "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>",
                10,
                179u64,
                2772u64,
            ),
            (
                "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
                100,
                1940,
                4686774924,
            ),
        ];

        for (input, steps, expected1, _) in test_cases_part_one {
            let reader = std::io::BufReader::new(input.as_bytes());

            let mut moons = parse_input(reader).unwrap();

            for _ in 0..*steps {
                moons.step()
            }

            let actual1 = moons.energy();

            assert_eq!(actual1, *expected1);
            // assert_eq!(actual2, *expected2);
        }

        utils::tests::test_full_problem(12, run, "7928", "");
    }
}
