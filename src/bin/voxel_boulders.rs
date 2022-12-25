type Point = (i64, i64, i64);

fn parse_point(line: &str) -> Point {
    let mut tokens = line.split(',');
    let mut parse = move || -> i64 {
        tokens
            .next()
            .expect("expected token")
            .trim()
            .parse()
            .expect("should be int")
    };
    let x = parse();
    let y = parse();
    let z = parse();
    (x, y, z)
}

fn adjucent((x, y, z): Point) -> impl Iterator<Item = Point> {
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
    .into_iter()
}

fn main() {
    let points: std::collections::HashSet<Point> =
        aoc2022::parse_line_by_line(aoc2022::get_input_file(), parse_point).collect();

    let surface_area = points
        .iter()
        .copied()
        .flat_map(adjucent)
        .filter(|p| !points.contains(p))
        .count();

    println!("{surface_area}");

    part2(points);
}

fn part2(points: std::collections::HashSet<Point>) {
    let (bbx, bby, bbz) = points
        .iter()
        .fold((0, 0, 0), |(bbx, bby, bbz): Point, &(x, y, z)| {
            (bbx.max(x), bby.max(y), bbz.max(z))
        });
    let is_empty = |(x, y, z): Point| -> bool {
        x >= -1
            && x <= bbx + 1
            && y >= -1
            && y <= bby + 1
            && z >= -1
            && z <= bbz + 1
            && !points.contains(&(x, y, z))
    };
    let mut visited = std::collections::HashSet::<Point>::new();
    let mut queue = std::collections::VecDeque::<Point>::new();
    visited.insert((0, 0, 0));
    queue.push_back((0, 0, 0));

    while let Some(cur) = queue.pop_front() {
        queue.extend(
            adjucent(cur)
                .filter(|&p| is_empty(p))
                .filter(|&p| visited.insert(p)),
        );
    }



    let surface_area = points
        .iter()
        .copied()
        .flat_map(adjucent)
        .filter(|p| visited.contains(p))
        .count();


    println!("{surface_area}");
}
