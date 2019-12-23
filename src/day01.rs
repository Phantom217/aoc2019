use crate::error::Error;

pub fn run<R>(mut input: R) -> Result<(), Error>
where
    R: std::io::BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;


    // Part 1
    let mut reader = std::io::BufReader::new(&content[..]);
    run_part(&mut reader, part_one)?;

    // Part 2
    let mut reader = std::io::BufReader::new(&content[..]);
    run_part(&mut reader, part_two)?;

    Ok(())
}

pub fn run_part<F, R>(input: &mut R, func: F) -> Result<(), Error>
where
    F: Fn(usize) -> usize,
    R: std::io::BufRead,
{
    let mut buffer = String::new();

    let mut total = 0;

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }

        let num = buffer.trim().parse::<usize>()?;

        let fuel = func(num);
        total += fuel;

        buffer.clear();
    }

    println!("{}", total);

    Ok(())
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
