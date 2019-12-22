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
    aoc2019::day01::run();
}
