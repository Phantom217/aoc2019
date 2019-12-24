use crate::error::Error;

pub fn run<R>(mut input: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    let mut buffer = String::new();

    let mut total1 = 0;
    let mut total2 = 0;

    // // does the same as the loop but this has more resource allocations.
    // // a new string is allocated every time we run through this for loop,
    // // whereas in the loop we allocate one string up front and clear it at
    // // the end of each loop and keep reusing it.
    // for res in input.lines() {
    //     let line = res?;
    //     let num = line.parse::<usize>()?;
    //
    //     total1 += part_one(num);
    //     total2 += part_two(num);
    // }

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }

        let num = buffer.trim().parse::<usize>()?;

        total1 += part_one(num);
        total2 += part_two(num);

        buffer.clear();
    }

    Ok((total1.to_string(), total2.to_string()))
}

fn part_one(num: usize) -> usize {
    match (num / 3).checked_sub(2) {
        Some(m) => m,
        None => 0,
    }
}

fn part_two(mut num: usize) -> usize {
    let mut total = 0;
    loop {
        let m = match (num / 3).checked_sub(2) {
            Some(m) => m,
            None => break total,
        };
        total += m;
        num = m;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let test_cases = &[
            // (input1, expected1, expected2)
            ("12", "2", "2"),
            ("14", "2", "2"),
            ("1969", "654", "966"),
            ("100756", "33583", "50346"),
        ];

        for (input, expected1, expected2) in test_cases {
            let reader = std::io::BufReader::new(input.as_bytes());
            let (actual1, actual2) = run(reader).unwrap();
            assert_eq!(*expected1, actual1);
            assert_eq!(*expected2, actual2);
        }
    }
}
