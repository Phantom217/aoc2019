pub fn run<R>(mut input: R)
    where R: std::io::BufRead {
    let mut buffer = String::new();

    let mut total = 0;

    loop {
        if input.read_line(&mut buffer).unwrap() == 0 {
            break;
        }

        let num = buffer.trim().parse::<usize>().unwrap();
        let m = match (num / 3).checked_sub(2) {
            Some(m) => m,
            None => 0,
        };

        total += m;

        buffer.clear();
    }

    println!("{}", total);
}
