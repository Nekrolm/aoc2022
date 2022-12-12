use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(i64),
    Mult(i64),
    Sqr
}

impl Operation {
    fn apply(self, old: i64) -> i64 {
        match self {
            Self::Add(x) => old + x,
            Self::Mult(x) => old * x,
            Self::Sqr => old * old,
        }
    }
}

#[derive(Debug)]
struct Tester {
    value: i64,
    on_true: usize,
    on_false: usize,
}

impl Tester {
    fn test(&self, val: i64) -> usize {
        if val % self.value == 0 {
            self.on_true
        } else {
            self.on_false
        }
    }
}

fn releaf(val: i64) -> i64 {
    // val / 3
    val
}

#[derive(Debug)]
struct Monkey {
    items: std::collections::VecDeque<i64>,
    op: Operation,
    tester: Tester,
    inspections: usize,
}

impl Monkey {
    fn inspect_next(&mut self, releaf : i64) -> Option<(usize, i64)> {
        let item = self.items.pop_front()?;
        let item = self.op.apply(item);
        self.inspections += 1;
        // let item = releaf(item);
        let item = item % releaf;
        let next = self.tester.test(item);
        Some((next, item))
    }
}

#[derive(Debug)]
struct MonkeyIsland {
    monkeys: Vec<Monkey>,
}

impl MonkeyIsland {
    fn round(&mut self) -> &mut Self {
        let releaf = self.monkeys.iter().map(|m| m.tester.value).fold(1, std::ops::Mul::mul);
        for idx in 0..self.monkeys.len() {
            while let Some((idx, val)) = self.monkeys[idx].inspect_next(releaf) {
                self.monkeys[idx].items.push_back(val)
            }
        }
        self
    }

    fn business(&self) -> usize {
        let mut busines: Vec<_> = self.monkeys.iter().map(|m| m.inspections).collect();
        busines.sort_by(|x, y| x.cmp(y).reverse());
        busines.into_iter().take(2).fold(1, std::ops::Mul::mul)
    }
}

fn main() {
    let mut reader = std::io::BufReader::new(aoc2022::get_input_file());
    let mut lines = reader.lines().map(Result::unwrap).fuse();
    let read_monkey = move || -> Option<Monkey> {
        let _header = lines.next()?;
        let items = lines.next()?;
        let (_, items) = items.split_once(":")?;
        let items = items.split(",").map(str::trim).map(|x| -> i64 { x.parse().expect("int expected") }).collect();
        let operation = lines.next()?;
        let mut operation = operation.split_whitespace();
        let arg = operation.next_back()?;
        let op = operation.next_back()?;
        let op = match (op, arg) {
            ("*", "old") => Operation::Sqr,
            ("*", x) => Operation::Mult(x.parse().expect("should be int")),
            ("+", x) => Operation::Add(x.parse().expect("should be int")),
            (s, arg) => panic!("Unknown op: {s}{arg}")
        };

        let test_value : i64 = lines.next()?.split_whitespace().next_back()?.parse().expect("expect int for test");
        let on_true : usize = lines.next()?.split_whitespace().next_back()?.parse().expect("expected int");
        let on_false : usize = lines.next()?.split_whitespace().next_back()?.parse().expect("expected int");
        
        let _skip = lines.next();

        Some(Monkey {
            items,
            inspections : 0,
            tester : Tester { value: test_value, on_true, on_false },
            op
        })
    };

    let mut island = MonkeyIsland {
        monkeys : std::iter::from_fn(read_monkey).collect()
    };

    for _ in 0..10000 {
        island.round();
    }

    println!("{island:#?}");


    println!("{}", island.business());

}
    