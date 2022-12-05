#![allow(dead_code)]

use std::io::BufRead;

use aoc2022::get_input_file;

use std::ops::RangeInclusive;

fn parse_range(range: &str) -> RangeInclusive<i64> {
    let Some((begin, end)) = range.split_once("-") else {
        panic!("expected range in x-y format");
    };
    RangeInclusive::new(
        begin.parse().expect("begin expected to be int"),
        end.parse().expect("end expected to be int")
    )
}

fn contains<Idx: PartialOrd<Idx>>(this: &RangeInclusive<Idx>, other: &RangeInclusive<Idx>) -> bool {
    this.contains(other.start()) && this.contains(other.end())
}

fn overlaps<Idx: PartialOrd<Idx>>(first: &RangeInclusive<Idx>, second: &RangeInclusive<Idx>) -> bool {
     first.contains(second.start()) 
    || first.contains(second.end()) 
    || second.contains(first.start()) 
    || second.contains(first.end())
}

fn main() {
    // let infile = get_input_file();

    // let reader = std::io::BufReader::new(infile);
    // let lines = reader.lines().map(Result::unwrap);

    // let cnt = lines.filter(
    //     |s| !s.is_empty()
    // ).map(|line| {
    //     let (first, second) = line.split_once(",").expect("should be two elements");
    //     (parse_range(first), parse_range(second))
    // }).filter(|(first, second)| {
    //     // contains(first, second) || contains(second, first)
    //     overlaps(first, second)
    // }).count();

    // println!("{cnt}")

    main_no_extra_alloc()
}

fn main_no_extra_alloc() {
    let mut infile = std::io::BufReader::new(get_input_file());
    let mut buffer = String::new();
    let read_ranges = move || {
        let _ = infile.read_line(&mut buffer);
        let ranges = buffer.trim_end().split_once(",").map(|(first, second)| {
            (parse_range(first), parse_range(second))
        });
        buffer.clear();
        ranges
    };
    let ranges = std::iter::from_fn(read_ranges);
    let needle_cnt = ranges.filter(|(first, second)| {
        overlaps(first, second)
    }).count();
    println!("{needle_cnt}")
}