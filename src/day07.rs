use crossbeam::channel;
use itertools::Itertools;

use crate::computer::{Channel, Computer, Rom};
use crate::error::Error;

fn fact(mut n: usize) -> Result<usize, Error> {
    let mut ans = 1usize;
    loop {
        ans = match ans.checked_mul(n) {
            Some(val) => val,
            None => bail!("Factorial of {} overflows usize.", n),
        };
        if (n - 1) == 0 {
            break;
        } else {
            n -= 1;
        }
    }

    Ok(ans)
}

// Representation of amplification to Intcode computer for Day 7:
//
//     chan4        chan0        chan1         chan2        chan3        chan4
// ...------- com0 ------- com1 -------- com2 ------- com3 ------- com4 -------...
//

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    let ncomputers = 5;
    let nchannels = fact(ncomputers)?;

    let barrier = std::sync::Barrier::new(ncomputers);
    let rom = Rom::from_reader(reader)?;

    let (answer1, answer2) = crossbeam::thread::scope(|s| {
        let (tx_output, rx_output) = channel::bounded(nchannels);

        let mut handles = Vec::new();
        let mut senders = Vec::new();

        for i in 0..ncomputers {
            let (tx_input, rx_input) = channel::bounded(nchannels);
            senders.push(tx_input);

            let barrier = &barrier;
            let rom = &rom;
            let tx_output = tx_output.clone();

            let handle = s.spawn(move |_| {
                loop {
                    let (part, phase_setting, input, output) = match rx_input.recv() {
                        Ok(data) => data,
                        Err(_) => break,
                    };
                    let mut computer = Computer::new(input, output);

                    computer.input_mut().push_back(phase_setting);
                    if i == 0 {
                        computer.input_mut().push_back(0);
                    }

                    barrier.wait();
                    computer.execute(rom, None)?;
                    barrier.wait();

                    if i == 4 {
                        let answer = computer.output_mut().pop_front()?;
                        tx_output.send((part, answer)).unwrap();
                    }
                }
                Ok::<_, Error>(())
            });
            handles.push(handle);
        }

        for (part, range) in (&[(0..5), (5..10)]).iter().cloned().enumerate() {
            for phase_settings in range.map(|i| i as i64).permutations(ncomputers) {
                let channels = (0..ncomputers)
                    .map(|_| Channel::default())
                    .collect::<Vec<_>>();

                let mut outputs = (0..ncomputers).map(|i| channels[i].clone());
                let mut inputs =
                    (0..ncomputers).map(|i| channels[(i + ncomputers - 1) % ncomputers].clone());

                for i in 0..ncomputers {
                    let output = outputs.next().unwrap();
                    let input = inputs.next().unwrap();
                    senders[i]
                        .send((part, phase_settings[i], input, output))
                        .unwrap();
                }
            }
        }

        drop(senders);
        drop(tx_output);

        let (mut answer1, mut answer2) = (0, 0);

        let mut iter = rx_output.iter();
        while let Some((part, output)) = iter.next() {
            match part {
                0 => {
                    if output > answer1 {
                        answer1 = output;
                    }
                }
                1 => {
                    if output > answer2 {
                        answer2 = output;
                    }
                }
                _ => unreachable!(),
            }
        }

        for handle in handles {
            handle.join().unwrap()?;
        }

        Ok::<_, Error>((answer1, answer2))
    })
    .unwrap()?;

    Ok((answer1.to_string(), answer2.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day07() {
        let test_cases_part_one = &[
            ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", "54321"),
            ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", "65210"),
        ];
        let test_cases_part_two = &[
            ("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", "139629729"),
            ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", "18216"),
        ];

        std::thread::spawn(move || {
            for (input, expected1) in test_cases_part_one {
                let reader = std::io::BufReader::new(input.as_bytes());
                let (actual1, _) = run(reader).unwrap();

                assert_eq!(actual1, *expected1);
            }
        });

        std::thread::spawn(move || {
            for (input, expected2) in test_cases_part_two {
                let reader = std::io::BufReader::new(input.as_bytes());
                let (_, actual2) = run(reader).unwrap();

                assert_eq!(actual2, *expected2);
            }
        });
        crate::utils::tests::test_full_problem(7, run, "277328", "11304734");
    }
}
