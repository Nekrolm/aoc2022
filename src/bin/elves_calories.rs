
use std::io::BufRead;

use aoc2022::get_input_file;



fn main() {
    let infile = get_input_file();
    let reader = std::io::BufReader::new(infile);
    let mut lines = reader.lines().map(Result::unwrap);

    let extract_one_elf = move || {
        let lines = lines.by_ref();
        let cal_sum : i64 = lines.take_while(|s| !s.is_empty()).map(|s| {
                let cal_value : i64 = s.parse().expect("should be int");
                cal_value    
            }
            ).sum();
        (cal_sum != 0).then_some(cal_sum)
    };

    let elfs = std::iter::from_fn(extract_one_elf);

    // part one:
    // println!("{:?}", elfs.max());

    let mut elfs = Vec::from_iter(elfs);
    elfs.sort();
    let top3_sum : i64 = elfs.into_iter().rev().take(3).sum();
    println!("{top3_sum}")
}