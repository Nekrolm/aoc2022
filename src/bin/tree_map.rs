use std::io::{BufRead, BufReader};

use aoc2022::Array2D;

fn filter_peaks<'a>(iter : impl Iterator<Item = (usize, &'a u8)> + 'a) -> impl Iterator<Item = (usize, &'a u8)> + 'a {
    let mut max = None;
    iter.filter(move |(_, &x)| {
        let visible = max < Some(x);
        max = max.max(Some(x));
        visible
    })
}

fn compute_visibility_score<'a>(iter : impl ExactSizeIterator<Item = (usize, &'a u8)> + 'a) -> impl Iterator<Item = (usize, usize)> + 'a {
    let mut items = Vec::with_capacity(iter.size_hint().0);
    items.resize(iter.size_hint().0, (0_usize, 0_u8));

    iter.enumerate().map(move |(idx_in_line, (index, &val))| {
        let mut prev = idx_in_line;
        while prev > 0 && items[prev].1 < val {
            let step = items[prev].0.max(1);
            prev -= step
        }
        items[idx_in_line] = (idx_in_line - prev, val);
        (index, items[idx_in_line].0)
    })
}


fn main() {
    let infile = aoc2022::get_input_file();
    let mut reader = BufReader::new(infile);
    let mut buffer = String::new();
    let mut rows = 0;
    while reader
        .read_line(&mut buffer)
        .ok()
        .filter(|&x| x != 0)
        .is_some()
    {
        while buffer.ends_with(char::is_whitespace) {
            buffer.pop();
        }
        rows += 1;
    }
    let buffer = buffer.into_bytes();
    let cols = buffer.len() / rows;
    let trees = Array2D::from_iter(buffer, (rows, cols)).expect("can't parse tree map");

    let by_rows = trees.rows().enumerate().flat_map(|(idx, row)| {
        let left_to_right = filter_peaks(row.iter().enumerate()).map(move |(jdx, _)| (idx, jdx));
        let right_to_left = filter_peaks(row.iter().enumerate().rev()).map(move |(jdx, _)| (idx, jdx));
        left_to_right.chain(right_to_left)
    });

    let by_cols = trees.cols().enumerate().flat_map(|(jdx, col)| {
        let left_to_right = filter_peaks(col.clone().enumerate()).map(move |(idx, _)| (idx, jdx));
        let right_to_left = filter_peaks(col.enumerate().rev()).map(move |(idx, _)| (idx, jdx));
        left_to_right.chain(right_to_left)
    });

    let visible : std::collections::HashSet<_> = by_rows.chain(by_cols).collect();

    println!("{}", visible.len());


    // part 2

    let by_rows = trees.rows().enumerate().flat_map(|(idx, row)| {
        let left_to_right = compute_visibility_score(row.iter().enumerate()).map(move |(jdx, score)| (idx, jdx, score));
        let right_to_left = compute_visibility_score(row.iter().enumerate().rev()).map(move |(jdx, score)| (idx, jdx, score));
        left_to_right.chain(right_to_left)
    });

    let by_cols = trees.cols().enumerate().flat_map(|(jdx, col)| {
        let left_to_right = compute_visibility_score(col.clone().enumerate()).map(move |(idx, score)| (idx, jdx, score));
        let right_to_left = compute_visibility_score(col.enumerate().rev()).map(move |(idx, score)| (idx, jdx, score));
        left_to_right.chain(right_to_left)
    });

    let shape @ (rows, cols) = trees.shape();
    let mut scores = Array2D::from_iter(vec![1; rows * cols], shape).unwrap();

    let scores = by_cols.chain(by_rows).fold(&mut scores, |scores, (idx, jdx, score)| {
        scores[(idx, jdx)] *= score;
        scores
    });

    let maxscore = scores.rows().flat_map(|r| r.iter()).max();
    println!("{maxscore:?}")

}
