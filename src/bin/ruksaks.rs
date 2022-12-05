use std::{io::BufRead, collections::HashSet};


fn priority(c: char) -> i64 {
    match c {
        'a'..='z' => 1 + (u64::from(c) - u64::from('a')) as i64,
        'A'..='Z' => 27 + (u64::from(c) - u64::from('A')) as i64,
        _ => panic!("unexpected symbol")
    }
}
 

fn main() {
    let infile = aoc2022::get_input_file();
    let reader = std::io::BufReader::new(infile);

    // let sum : i64 = reader.lines().map(Result::unwrap).map(|s| {
    //     let len = s.len() / 2;
    //     let first = &s[..len];
    //     let last = &s[len..];
    //     let first = HashSet::<_>::from_iter(first.chars());
    //     let last = HashSet::<_>::from_iter(last.chars());
    //     first.intersection(&last).map(|&c| priority(c)).sum::<i64>()
    // }).sum();

    let mut lines = reader.lines().map(Result::unwrap);
    let group = move || {
        lines.by_ref().take(3).map(|s| HashSet::<_>::from_iter(s.chars())).reduce(|first, second| {
            &first & &second
        })
    };
    let groups = std::iter::from_fn(group);
    let sum : i64 = groups.flat_map(|common| common.into_iter().map(priority)).sum();

    println!("{sum}")
}