fn rotate<T>(buf: &mut [T], from: usize, shift: i64) {
    let shift = (shift.abs() % (buf.len() - 1) as i64) * shift.signum();

    if shift == 0 {
        return;
    }

    let step = shift.signum();
    let mut cur = from;
    for _ in 0..(shift.abs()) {
        let next = (cur as i64 + step).rem_euclid(buf.len() as i64) as usize;
        buf.swap(cur, next);
        cur = next;
    }
}

fn mixin(arr: Vec<i64>, count: usize) -> Vec<i64> {
    #[derive(Debug)]
    struct Elem {
        val: i64,
        index: usize,
    }

    let mut buffer: Vec<_> = arr
        .into_iter()
        .enumerate()
        .map(|(index, val)| Elem { val, index })
        .collect();

    for _ in 0..count {
        let mut curr_idx = 0_usize;
        let mut cur_number = 0_usize;
        while let Some(pos) = (&buffer[curr_idx..])
            .iter()
            .chain(&buffer[..curr_idx])
            .position(|v| cur_number == v.index)
        {
            curr_idx = (curr_idx + pos) % buffer.len();
            assert_eq!(buffer[curr_idx].index, cur_number);
            let shift = buffer[curr_idx].val;
            rotate(&mut buffer, curr_idx, shift);
            cur_number += 1;
        }
    }

    let mut result: Vec<_> = buffer.into_iter().map(|Elem { val, .. }| val).collect();

    if let Some(pos) = result.iter().position(|&v| v == 0) {
        result.rotate_left(pos);
    }

    result
}

fn main() {
    const KEY: i64 = 811589153;
    let arr: Vec<i64> = aoc2022::parse_line_by_line(aoc2022::get_input_file(), |s| {
        s.parse::<i64>().expect("expected int")
    })
    .map(|v| v * KEY)
    .collect();

    let mixed = mixin(arr, 10);

    let answer: i64 = [1000, 2000, 3000]
        .into_iter()
        .map(|p| p % mixed.len())
        .map(|p| mixed[p])
        .sum();

    // println!("{:?}", &mixed[..100]);
    println!("{answer}")
}
