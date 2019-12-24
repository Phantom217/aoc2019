use std::fs;
use std::io;

use aoc2019;
use criterion::{criterion_group, criterion_main, Criterion};

fn target_01(c: &mut Criterion) {
    let day01 = fs::read_to_string("input/day01.txt").unwrap();
    c.bench_function("day_01", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day01.as_bytes());
            aoc2019::day01::run(reader).unwrap();
        })
    });
}

fn target_02(c: &mut Criterion) {
    let day02 = fs::read_to_string("input/day02.txt").unwrap();
    c.bench_function("day_02", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day02.as_bytes());
            aoc2019::day02::run(reader).unwrap();
        })
    });
}

fn target_03(c: &mut Criterion) {
    let day03 = fs::read_to_string("input/day03.txt").unwrap();
    c.bench_function("day_03", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day03.as_bytes());
            aoc2019::day03::run(reader).unwrap();
        })
    });
}

fn target_04(c: &mut Criterion) {
    let day04 = fs::read_to_string("input/day04.txt").unwrap();
    c.bench_function("day_04", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day04.as_bytes());
            aoc2019::day04::run(reader).unwrap();
        })
    });
}

criterion_group! {
    name = group;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(5));
    targets = target_01, target_02, target_03, target_04,
}

criterion_main!(group);
