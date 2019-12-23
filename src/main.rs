use std::fs;
use std::io;
use std::path::PathBuf;

use structopt::StructOpt;

use aoc2019::{self, Reader};

#[derive(Debug, StructOpt)]
struct Opt {
    /// Day
    day: usize,

    /// Input file path, if not supplied will read from stdin
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let stdin = io::stdin();

    let input = match opt.input {
        Some(path) => {
            let file = fs::File::open(path).unwrap();
            let reader = io::BufReader::new(file);
            Reader::File( reader )
        },
        None => {
            let guard = stdin.lock();
            Reader::Stdin(guard)
        },
    };

    match opt.day {
        1 => aoc2019::day01::run(input),
        2 => aoc2019::day02::run(input),
        n if n > 1 && n < 26 => panic!("Day {} is not yet implemented.", n),
        _ => panic!("Day must be between 1 and 25, inclusive."),
    }
}
