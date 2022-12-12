use aoc2022::{either::Either, parse_line_by_line};

#[derive(Clone, Copy)]
enum Command {
    Noop,
    Add(i64),
}

fn parse_command(line: &str) -> impl Iterator<Item = Command> {
    let mut tokens = line.split_whitespace();
    let cmd = tokens.next().expect("command expected");
    match cmd {
        "noop" => Either::Left(std::iter::once(Command::Noop)),
        "addx" => {
            let arg: i64 = tokens
                .next()
                .expect("expected add arg")
                .parse()
                .expect("int expected");
            Either::Right([Command::Noop, Command::Add(arg)].into_iter())
        }
        x => panic!("unexpected input: {x}"),
    }
}

pub struct Register(i64);

impl Register {
    fn exec(&mut self, cmd: Command) -> &mut Self {
        if let Command::Add(val) = cmd {
            self.0 += val;
        }
        self
    }

    fn strenght(&self, cycle: usize) -> i64 {
        cycle as i64 * self.0
    }

    fn draw(&self, cycle: usize) -> char {
        if self.0.abs_diff((cycle % 40) as i64) <= 1 {
            '#'
        } else {
            '.'
        }
    }
}

fn main() {
    let file = aoc2022::get_input_file();
    // let interesting_cycles = [20, 60, 100, 140, 180, 220];
    // let mut accum = 0;
    // let mut reg = Register(1);
    // let _ = std::iter::zip(1.., parse_line_by_line(file, parse_command).flatten()).fold(
    //     &mut reg,
    //     |reg, (cycle, cmd)| {
    //         if interesting_cycles.contains(&cycle) {
    //             accum += reg.strenght(cycle);
    //         }
    //         reg.exec(cmd)
    //     },
    // );
    // println!("{accum}")

    let mut reg = Register(1);
    let mut symbols = std::iter::zip(
        0..240,
        parse_line_by_line(file, parse_command)
            .flatten()
            .chain(std::iter::repeat(Command::Noop)),
    )
    .map(|(idx, cmd)| {
        let symbol = reg.draw(idx);
        reg.exec(cmd);
        symbol
    });

    let symbol_line = std::iter::from_fn(move || {
        let line: String = symbols.by_ref().take(40).collect();
        (!line.is_empty()).then_some(line)
    });

    for line in symbol_line {
        println!("{line}")
    }
}
