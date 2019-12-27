use crate::error::Error;

const ROWS: usize = 6;
const COLS: usize = 25;

pub fn run<R>(mut reader: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    // Parse input
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    buf.pop();
    buf.iter_mut().for_each(|b| *b -= 48);

    // Part 1
    let answer1 = match buf
        .chunks(ROWS * COLS)
        .fold((std::usize::MAX, None), |mut state, layer| {
            let num_zeros = bytecount::count(layer, 0);
            if num_zeros < state.0 {
                state = (num_zeros, Some(layer));
            }
            state
        }) {
        (_, Some(layer)) => {
            let num_ones = bytecount::count(layer, 1);
            let num_twos = bytecount::count(layer, 2);
            num_ones * num_twos
        }
        (_, None) => bail!("TODO"),
    };

    // Part 2
    let image = buf
        .chunks(ROWS * COLS)
        .fold([2u8; ROWS * COLS], |mut state, layer| {
            state.iter_mut().enumerate().for_each(|(i, b)| {
                if *b == 2 {
                    *b = layer[i];
                }
            });
            state
        });

    let mut iter = image.iter();
    let mut answer2 = String::new();
    for _ in 0..ROWS {
        for _ in 0..COLS {
            match iter.next() {
                Some(0) => answer2.push('\u{2585}'),
                Some(1) => answer2.push(' '),
                Some(_) => bail!("Bad input: Found digit that is neither 0 nor 1"),
                None => bail!(
                    "Bad input. Must contain {} rows and {} columns.",
                    ROWS,
                    COLS
                ),
            }
        }
        answer2.push('\n');
    }

    Ok((answer1.to_string(), answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08() {
        let part_two_answer = String::from(" ▅▅▅▅▅▅  ▅    ▅▅  ▅▅ ▅▅ ▅\n ▅▅▅▅▅▅▅ ▅ ▅▅▅▅ ▅▅ ▅ ▅▅ ▅\n ▅▅▅▅▅▅▅ ▅   ▅▅ ▅▅▅▅    ▅\n ▅▅▅▅▅▅▅ ▅ ▅▅▅▅ ▅▅▅▅ ▅▅ ▅\n ▅▅▅▅ ▅▅ ▅ ▅▅▅▅ ▅▅ ▅ ▅▅ ▅\n    ▅▅  ▅▅    ▅▅  ▅▅ ▅▅ ▅\n");
        crate::utils::tests::test_full_problem(8, run, "1792", &part_two_answer);
    }
}
