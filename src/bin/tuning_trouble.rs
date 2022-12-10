

fn first_n_unique_detector(input: &str, cnt: usize) -> Option<usize> {
    let mut counter: std::collections::HashMap<char, usize> = Default::default();
    let first = input.chars();
    let mut last = input.chars().enumerate();
    last.by_ref().take(cnt - 1).for_each(|(_, c)| *counter.entry(c).or_default() += 1 );
    std::iter::zip(first, last).find_map(|(old, (idx, new))| {
        *counter.entry(new).or_default() += 1;
        if counter.len() == cnt {
            return Some(idx + 1);
        }
        use std::collections::hash_map::Entry;
        let Entry::Occupied(mut entry) = counter.entry(old)
        else {
            unreachable!("we are deleting existing key")
        };
        let val = entry.get_mut();
        *val -= 1;
        if *val == 0 {
            entry.remove();
        }
        None
    })
}

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let input = input.trim();
    let pos = first_n_unique_detector(input, 14);
    println!("{pos:?}")
}