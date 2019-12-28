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

fn target_05(c: &mut Criterion) {
    let day05 = fs::read_to_string("input/day05.txt").unwrap();
    c.bench_function("day_05", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day05.as_bytes());
            aoc2019::day05::run(reader).unwrap();
        })
    });
}

fn target_06(c: &mut Criterion) {
    let day06 = fs::read_to_string("input/day06.txt").unwrap();
    c.bench_function("day_06", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day06.as_bytes());
            aoc2019::day06::run(reader).unwrap();
        })
    });
}

fn target_07(c: &mut Criterion) {
    let day07 = fs::read_to_string("input/day07.txt").unwrap();
    c.bench_function("day_07", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day07.as_bytes());
            aoc2019::day07::run(reader).unwrap();
        })
    });
}

fn target_08(c: &mut Criterion) {
    let day08 = fs::read_to_string("input/day08.txt").unwrap();
    c.bench_function("day_08", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day08.as_bytes());
            aoc2019::day08::run(reader).unwrap();
        })
    });
}

fn target_09(c: &mut Criterion) {
    let day09 = fs::read_to_string("input/day09.txt").unwrap();
    c.bench_function("day_09", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day09.as_bytes());
            aoc2019::day09::run(reader).unwrap();
        })
    });
}

fn target_10(c: &mut Criterion) {
    let day10 = fs::read_to_string("input/day10.txt").unwrap();
    c.bench_function("day_10", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day10.as_bytes());
            aoc2019::day10::run(reader).unwrap();
        })
    });
}

fn target_11(c: &mut Criterion) {
    let day11 = fs::read_to_string("input/day11.txt").unwrap();
    c.bench_function("day_11", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day11.as_bytes());
            aoc2019::day11::run(reader).unwrap();
        })
    });
}

fn target_12(c: &mut Criterion) {
    let day12 = fs::read_to_string("input/day12.txt").unwrap();
    c.bench_function("day_12", move |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day12.as_bytes());
            aoc2019::day12::run(reader).unwrap();
        })
    });
}

criterion_group! {
    name = group;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(5));
    targets = target_01, target_02, target_03, target_04, target_05, target_06,
        target_07, target_08, target_09, target_10, target_11, target_12,
}

criterion_main!(group);
