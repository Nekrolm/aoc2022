#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    nearest_distance: u64,
}

// L, size
type Intervals = std::collections::BTreeMap<i64, i64>;

type Interval = (i64, i64); // [)

fn insert_interval(intervals: &mut Intervals, (left, mut right): Interval) -> &mut Intervals {
    if left == right {
        return intervals;
    }

    let mut greater_than_right = intervals.split_off(&right);

    {
        let mut greater_then_left = intervals.split_off(&left);
        if let Some(o) = greater_then_left.last_entry().filter(|o| *o.get() >= right) {
            right = o.remove();
        }
    }

    if let Some(o) = greater_than_right
        .first_entry()
        .filter(|o| *o.key() == right)
    {
        right = o.remove();
    }

    // greater_than_left may be dropped

    if let Some(mut o) = intervals.last_entry().filter(|o| *o.get() >= left) {
        let val = *o.get();
        *o.get_mut() = val.max(right);
    } else {
        intervals.insert(left, right);
    }

    intervals.append(&mut greater_than_right);

    intervals
}

fn remove_interval(intervals: &mut Intervals, (left, right): Interval) -> &mut Intervals {
    if left == right {
        return intervals;
    }
    let mut greater_than_right = intervals.split_off(&right);

    {
        let mut greater_than_left = intervals.split_off(&left);
        if let Some(o) = greater_than_left.last_entry().filter(|o| *o.get() > right) {
            let right_right = o.remove();
            greater_than_right.entry(right).or_insert(right_right);
        }
    }

    if let Some(mut o) = intervals.last_entry().filter(|o| *o.get() > left) {
        let right_right = std::mem::replace(o.get_mut(), left);
        if right_right > right {
            greater_than_right.entry(right).or_insert(right_right);
        }
    }

    intervals.append(&mut greater_than_right);

    intervals
}

fn parse_line(line: &str) -> (Sensor, (i64, i64)) {
    let (sensor, beacon) = line.split_once(':').expect("expected :");
    let sensor = sensor.trim();
    let beacon = beacon.trim();

    dbg!(sensor);
    dbg!(beacon);

    let extract = |s: &str| -> (i64, i64) {
        let mut tokens = s.split('=');
        let _ = tokens.next();
        let (x, _) = tokens
            .next()
            .map(str::trim)
            .and_then(|s| s.split_once(','))
            .expect("x should be there");
        let y = tokens.next().expect("expected y");
        (
            x.parse().expect("should be int"),
            y.parse().expect("should be int"),
        )
    };

    let beacon @ (bx, by) = extract(beacon);
    let (x, y) = extract(sensor);

    let distance = bx.abs_diff(x) + by.abs_diff(y);

    (
        Sensor {
            x,
            y,
            nearest_distance: distance,
        },
        beacon,
    )
}

impl Sensor {
    fn get_interval_at_y(&self, y: i64) -> (i64, i64) {
        let d = self.y.abs_diff(y);

        if d > self.nearest_distance {
            return (self.x, self.x);
        }

        let ofs = (self.nearest_distance - d) as i64;

        let left = self.x - ofs;
        let right = self.x + ofs + 1;
        (left, right)
    }
}

fn search_in_line(
    sensors: &[Sensor],
    y: i64,
    limits: std::ops::RangeInclusive<i64>,
    intervals: &mut Intervals,
) -> Option<i64> {
    intervals.clear();
    sensors
        .iter()
        .map(|s| s.get_interval_at_y(y))
        .fold(&mut *intervals, insert_interval)
        .iter()
        .skip_while(|&(_left, right)| right <= limits.start())
        .take_while(|&(left, right)| limits.contains(left) || limits.contains(right))
        .flat_map(|(left, &right)| [left - 1, right])
        .find(|x| limits.contains(x))
}

fn main() {
    let (sensors, beacons): (Vec<_>, Vec<_>) =
        aoc2022::parse_line_by_line(aoc2022::get_input_file(), parse_line).unzip();

    const LIMIT: i64 = 4_000_000;

    let mut intervals = Intervals::default();
    let pos = (0..=LIMIT)
        .find_map(move |y| search_in_line(&sensors, y, 0..=LIMIT, &mut intervals).zip(Some(y)))
        .map(|(x, y)| x * LIMIT + y);

    println!("{pos:?}")

    // part one

    // let mut intervals = Intervals::default();

    // let y = 2000000;

    // sensors
    //     .into_iter()
    //     .map(|s| {
    //         dbg!(&s);
    //         dbg!(s.get_interval_at_y(y))
    //     })
    //     .fold(&mut intervals, insert_interval);
    // beacons
    //     .into_iter()
    //     .filter_map(|(bx, by)| (by == y).then_some((bx, bx + 1)))
    //     .fold(&mut intervals, remove_interval);

    // let positions: i64 = intervals.iter().map(|(l, r)| r - l).sum();
    // dbg!(intervals);
    // println!("{positions}");
}

#[test]
fn test_itervals() {
    let mut intervals = Intervals::default();
    insert_interval(&mut intervals, (1, 5));
    insert_interval(&mut intervals, (5, 6));
    dbg!(&intervals);
    insert_interval(&mut intervals, (-5, 10));
    dbg!(&intervals);
    remove_interval(&mut intervals, (2, 5));
    dbg!(&intervals);
    remove_interval(&mut intervals, (9, 10));
    dbg!(&intervals);
    remove_interval(&mut intervals, (-5, -4));
    dbg!(&intervals);
    insert_interval(&mut intervals, (1, 6));
    dbg!(&intervals);
}
