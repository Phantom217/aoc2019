use crate::error::Error;

struct Computer {
    rom: Vec<usize>,
    ram: Vec<usize>,
    pc: usize,
}

impl Computer {
    fn new<R>(mut rom_reader: R) -> Result<Self, Error>
    where
        R: std::io::BufRead,
    {
        let mut buffer = String::new();
        rom_reader.read_to_string(&mut buffer)?;
        let rom = buffer
            .trim()
            .split(",")
            .map(|s| Ok(s.parse::<usize>()?))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Self {
            rom,
            ram: Vec::new(),
            pc: 0,
        })
    }

    fn execute(&mut self, noun: usize, verb: usize) -> Result<usize, Error> {
        self.ram = self.rom.clone();

        self.pc = 0;
        self.ram[1] = noun;
        self.ram[2] = verb;

        loop {
            let opcode = self.ram[self.pc];
            match opcode {
                1 | 2 => {
                    let a_ptr = self.ram[self.pc + 1];
                    let b_ptr = self.ram[self.pc + 2];
                    let w_ptr = self.ram[self.pc + 3];
                    let a = self.ram[a_ptr];
                    let b = self.ram[b_ptr];
                    self.ram[w_ptr] = if opcode == 1 { a + b } else { a * b };
                    self.pc += 4;
                }
                99 => break,
                _ => bail!("Invalid opcode {}.", opcode),
            }
        }

        Ok(self.ram[0])
    }
}

pub fn run<R>(input: R) -> Result<(), Error>
where
    R: std::io::BufRead,
{
    let mut computer = Computer::new(input)?;
    let answer = computer.execute(12, 2)?;

    println!("{}", answer);

    let mut answer = Err(error!(
        "Invalid input. Unable to find noun/verb combination that outputs 19690720."
    ));
    for noun in 0..=99 {
        for verb in 0..=99 {
            if computer.execute(noun, verb)? == 19690720 {
                answer = Ok(100 * noun + verb);
            }
        }
    }

    println!("{}", answer?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let test_cases = &[
            // (input, noun, verb, expected_ram)
            (
                "1,9,10,3,2,3,11,0,99,30,40,50",
                9,
                10,
                "3500,9,10,70,2,3,11,0,99,30,40,50",
            ),
            ("1,0,0,0,99", 0, 0, "2,0,0,0,99"),
            ("2,3,0,3,99", 3, 0, "2,3,0,6,99"),
            ("2,4,4,5,99,0", 4, 4, "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", 1, 1, "30,1,1,4,2,5,6,0,99"),
        ];

        for (input, noun, verb, expected_ram) in test_cases {
            let reader = std::io::BufReader::new(input.as_bytes());
            let mut computer = Computer::new(reader).unwrap();
            let _ = computer.execute(*noun, *verb).unwrap();
            let expected_ram = expected_ram
                .split(",")
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(computer.ram, expected_ram);
        }
    }
}