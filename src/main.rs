use std::fs;
use std::io;
use std::path::PathBuf;

use structopt::StructOpt;

use aoc2019;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Day
    day: usize,

    /// Input file path, if not supplied will read from stdin
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    match opt.input {
        Some(path) => {
            let file = fs::File::open(path).unwrap();
            let reader = io::BufReader::new(file);
            let answer = aoc2019::day01::run(reader);
            println!("{}", answer);
        },
        None => unimplemented!(),
    }
}
