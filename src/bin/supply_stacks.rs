use std::io::BufRead;

use aoc2022::get_input_file;

type CrateStack = Vec<char>;

#[derive(Debug, Default)]
struct Crates {
    table: Vec<CrateStack>,
}



struct Command {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_command(s: &str) -> Command {
    let mut split = s.split_whitespace();
    let _move = split.next();
    let count = split
        .next()
        .expect("move x expected")
        .parse()
        .expect("should be int");
    let _from = split.next();
    let from = split
        .next()
        .expect("move x from y expected")
        .parse()
        .expect("should be int");
    let _to = split.next();
    let to = split
        .next()
        .expect("move x from y to z expected")
        .parse()
        .expect("should be int");
    Command { from, to, count }
}

impl Crates {
    fn exec(&mut self, cmd: Command) -> &mut Self {
        let Command { from, to, count } = cmd;

        let from = from - 1;
        let to = to - 1;
        for _ in 0..count {
            let elem = self.table[from].pop();
            self.table[to].extend(elem)
        }

        self
    }

    fn exec_preserve(&mut self, cmd: Command) -> &mut Self {
        let Command { from, to, count } = cmd;

        let from = from - 1;
        let to = to - 1;

        let mut stack_to = std::mem::take(&mut self.table[to]);

        let movable = {
            let stack = &mut self.table[from];
            let drain_range = (stack.len() - count)..;
            stack.drain(drain_range)
        };

        stack_to.extend(movable);

        self.table[to] = stack_to;

        self
    }

    fn topline(&self) -> String {
        self.table.iter().flat_map(|stack| stack.last()).collect()
    }
}


fn parse_crates_line(s: &str) -> impl Iterator<Item = Option<char>> + '_ {
    s.as_bytes()
        .chunks(4)
        .map(std::str::from_utf8)
        .map(Result::unwrap)
        .map(str::trim)
        .map(|elem| elem.chars().nth(1).filter(|c| c.is_ascii_alphabetic()))
}

fn main() {
    let infile = get_input_file();

    let reader = std::io::BufReader::new(infile);
    let mut lines = reader.lines().map(Result::unwrap);

    let crates_lines = lines.by_ref().take_while(|s| !s.is_empty());
    let mut crates = crates_lines.fold(Crates::default(), |mut crates, line| {
        parse_crates_line(&line).enumerate().for_each(|(idx, val)| {
            crates
                .table
                .resize_with(crates.table.len().max(idx + 1), Default::default);
            crates.table[idx].extend(val)
        });
        crates
    });
    crates.table.iter_mut().for_each(|stack| stack.reverse());

    lines
        .filter(|s| !s.is_empty())
        .map(|line| parse_command(&line))
        .fold(&mut crates, Crates::exec_preserve);


    println!("{}", crates.topline())
}
