use crate::error::Error;

pub fn run<R>(mut input: R) -> Result<(), Error>
    where R: std::io::BufRead {
    let mut buffer = String::new();

    let mut total = 0;

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }

        let num = buffer.trim().parse::<usize>()?;
        let m = match (num / 3).checked_sub(2) {
            Some(m) => m,
            None => 0,
        };

        total += m;

        buffer.clear();
    }

    println!("{}", total);

    Ok(())
}
