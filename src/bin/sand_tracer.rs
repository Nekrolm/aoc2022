use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector2D {
    x: i64,
    y: i64,
}

impl std::ops::Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn line_from_to(from: Vector2D, to: Vector2D) -> impl Iterator<Item = Vector2D> {
    let mut step = to - from;
    let n = step.x.abs().max(step.y.abs());
    step.x = step.x.signum();
    step.y = step.y.signum();
    (0..=n).map(move |n| from + (step * n))
}

fn vec2d_to_tuple_uidx(this: Vector2D) -> (usize, usize) {
    (this.y as _, this.x as _)
}

struct Line(Vec<Vector2D>);

fn parse_point(s: &str) -> Vector2D {
    let (x, y) = s.split_once(',').expect("two elements are expected");
    Vector2D {
        x: x.parse().expect("int expected"),
        y: y.parse().expect("int expected"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Fill,
    Sand,
}

fn simulate(grid: &mut aoc2022::Array2D<Tile>, mut start: Vector2D) -> bool {
    let (max_y, max_x) = grid.shape();
    let (max_y, max_x) = (max_y as i64, max_x as i64);
    const STEPS: [Vector2D; 3] = [
        Vector2D { x: 0, y: 1 },
        Vector2D { x: -1, y: 1 },
        Vector2D { x: 1, y: 1 },
    ];
    let is_valid =
        |&Vector2D { x, y }: &Vector2D| -> bool { x >= 0 && x < max_x && y >= 0 && y < max_y };
    loop {
        if !is_valid(&start) {
            return false;
        }
        let next = STEPS
            .into_iter()
            .map(|step| start + step)
            .filter(|v| !is_valid(v) || grid[vec2d_to_tuple_uidx(*v)] == Tile::Air)
            .next();
        if let Some(next) = next {
            start = next;
        } else {
            grid[vec2d_to_tuple_uidx(start)] = Tile::Sand;
            return true;
        }
    }
}

fn simulate_v2(grid: &mut aoc2022::Array2D<Tile>, start: Vector2D) -> bool {
    let mut cur_pos = start;
    let (max_y, max_x) = grid.shape();
    let (max_y, max_x) = (max_y as i64, max_x as i64);
    const STEPS: [Vector2D; 3] = [
        Vector2D { x: 0, y: 1 },
        Vector2D { x: -1, y: 1 },
        Vector2D { x: 1, y: 1 },
    ];
    let is_valid =
        |&Vector2D { x, y }: &Vector2D| -> bool { x >= 0 && x < max_x && y >= 0 && y < max_y };
    loop {
        if !is_valid(&cur_pos) {
            panic!("overflow!");
        }
        let next = STEPS
            .into_iter()
            .map(|step| cur_pos + step)
            .filter(|v| !is_valid(v) || grid[vec2d_to_tuple_uidx(*v)] == Tile::Air)
            .next();
        if let Some(next) = next {
            cur_pos = next;
        } else {
            grid[vec2d_to_tuple_uidx(cur_pos)] = Tile::Sand;
            return cur_pos != start;
        }
    }
}

fn show_grid(grid: &aoc2022::Array2D<Tile>) {
    grid.rows()
        .flat_map(|row| {
            row.iter()
                .map(|s| match s {
                    Tile::Air => '.',
                    Tile::Fill => '#',
                    Tile::Sand => 'o',
                })
                .chain(std::iter::once('\n'))
        })
        .for_each(|c| print!("{c}"));
}

fn main() {
    let file = aoc2022::get_input_file();
    let lines: Vec<Line> = aoc2022::parse_line_by_line(file, |line| {
        Line(line.split("->").map(str::trim).map(parse_point).collect())
    })
    .collect();
    let start = Vector2D { x: 500, y: 0 };

    let bounds = lines
        .iter()
        .flat_map(|line| line.0.iter())
        .chain([&start])
        .copied()
        .reduce(|acc, p| Vector2D {
            x: acc.x.max(p.x),
            y: acc.y.max(p.y),
        })
        .expect("expected at least one element");

    // let min_x: Option<i64> = lines
    //     .iter()
    //     .flat_map(|line| line.0.iter().map(|v| v.x))
    //     .min();
    let min_x = -200;
    let start = start - Vector2D { x: min_x, y: 0 };
    let shape = (bounds.y as usize + 1 + 2, (bounds.x - min_x * 2) as usize + 1);

    let mut cave = aoc2022::Array2D::from_shape_and_val(shape, Tile::Air);

    let cave = lines
        .iter()
        .flat_map(|Line(line)| std::iter::zip(&line[..], &line[1..]))
        .flat_map(|(&from, &to)| line_from_to(from, to))
        .map(|v| v - Vector2D { x: min_x, y: 0 })
        .map(vec2d_to_tuple_uidx)
        .fold(&mut cave, |cave, idx| {
            cave[idx] = Tile::Fill;
            cave
        });

    let _ = cave
        .rows_mut()
        .last()
        .map(|row| row.fill(Tile::Fill))
        .expect("expected at least one row");

    for sands in 1.. {
        if !simulate_v2(cave, start) {
            println!("{sands}");
            break;
        }
    }
    show_grid(cave);
}
