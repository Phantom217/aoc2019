use crate::computer::{Computer, Rom};
use crate::error::Error;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    let rom = Rom::from_reader(input)?;
    let mut computer = Computer::default();

    // Part 1
    computer.input_mut().push_back(1);
    computer.execute(&rom, None)?;
    let answer1 = computer
        .output_mut()
        .try_iter()
        .last()
        .ok_or_else(|| error!("Nothing in output channel for part 1."))?;

    // Reset I/O State
    computer.input_mut().try_clear();
    computer.output_mut().try_clear();

    computer.input_mut().push_back(5);
    computer.execute(&rom, None)?;
    let answer2 = computer
        .output_mut()
        .try_iter()
        .last()
        .ok_or_else(|| error!("Nothing in output channel for part 2."))?;

    Ok((answer1.to_string(), answer2.to_string()))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05() {
        crate::utils::tests::test_full_problem(5, run, "15508323", "9006327");
    }
}
