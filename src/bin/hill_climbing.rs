fn height(c: u8) -> i64 {
    match c as char {
        'S' => 'a' as _,
        'E' => 'z' as _,
        _ => c as _,
    }
}

fn can_step(from: u8, to: u8) -> bool {
    let from = height(from);
    let to = height(to);
    from + 1 >= to
}

fn steps(
    (x, y): (usize, usize),
    (rows, cols): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    const OFFSETS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let x = x as isize;
    let y = y as isize;
    let rows = rows as isize;
    let cols = cols as isize;
    OFFSETS
        .into_iter()
        .map(move |(dx, dy)| {
            let x = dx + x;
            let y = dy + y;
            (x, y)
        })
        .filter_map(move |(x, y)| {
            (x < rows && x >= 0 && y < cols && y >= 0).then_some((x as usize, y as usize))
        })
}

fn main() {
    let infile = aoc2022::get_input_file();
    let mut buffer = Vec::default();
    let width = aoc2022::parse_line_by_line(infile, |line| {
        let bytes = line.as_bytes();
        buffer.extend_from_slice(bytes);
        bytes.len()
    })
    .last()
    .expect("expected at least one line");
    let height = buffer.len() / width;
    let grid = aoc2022::Array2D::from_iter(buffer, (height, width)).expect("should be rect grid");

    // part one
    // let start = grid
    //     .iter_indexed()
    //     .find_map(|(idx, &val)| (val as char == 'S').then_some(idx));

    // part two
    let start = grid
        .iter_indexed()
        .filter_map(|(idx, &val)| matches!(val as char, 'a' | 'S').then_some(idx));

    let end = grid
        .iter_indexed()
        .find_map(|(idx, &val)| (val as char == 'E').then_some(idx))
        .expect("end should be there");

    let mut visited = aoc2022::Array2D::from_shape_and_val((height, width), false);
    let mut queue = std::collections::VecDeque::new();

    queue.extend(
        start
            .into_iter()
            .inspect(|&pos| visited[pos] = true)
            .map(|pos| (0, pos)),
    );

    while let Some((d, pos)) = queue.pop_front() {
        if pos == end {
            println!("{d}");
            break;
        }
        let cur = grid[pos];
        let next = steps(pos, (height, width))
            .filter(|&next| {
                let next = grid[next];
                can_step(cur, next)
            })
            .filter_map(|pos| {
                (!std::mem::replace(&mut visited[pos], true)).then_some((d + 1, pos))
            });
        queue.extend(next)
    }
}
