use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Movable,
    Fill,
}

const LINE_SIZE: usize = 7;

type TableLine = [Tile; LINE_SIZE];

fn can_move_left(line: &TableLine) -> bool {
    if line[0] == Tile::Movable {
        return false;
    }

    for idx in 1..LINE_SIZE {
        if line[idx] == Tile::Movable && line[idx - 1] == Tile::Fill {
            return false;
        }
    }
    true
}

fn move_left(line: &mut TableLine) -> bool {
    let can_move = can_move_left(line);
    if can_move {
        for idx in 1..LINE_SIZE {
            if line[idx] == Tile::Movable {
                line[idx - 1] = std::mem::replace(&mut line[idx], Tile::Empty);
            }
        }
    }
    can_move
}

fn can_move_right(line: &TableLine) -> bool {
    if line[LINE_SIZE - 1] == Tile::Movable {
        return false;
    }

    for idx in (0..(LINE_SIZE - 1)).rev() {
        if line[idx] == Tile::Movable && line[idx + 1] == Tile::Fill {
            return false;
        }
    }
    true
}

fn move_right(line: &mut TableLine) -> bool {
    let can_move = can_move_right(line);
    if can_move {
        for idx in (0..(LINE_SIZE - 1)).rev() {
            if line[idx] == Tile::Movable {
                line[idx + 1] = std::mem::replace(&mut line[idx], Tile::Empty);
            }
        }
    }
    can_move
}

fn contains_movable(line: &TableLine) -> bool {
    line.contains(&Tile::Movable)
}

#[derive(Default)]
struct Table {
    floor_level: usize,
    table: std::collections::VecDeque<TableLine>,
    block_first_line: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Block {
    Vertical,
    Horizontal,
    Block2x2,
    LBlock,
    Cross,
}

fn spawn_verical_block(table: &mut Table) {
    const ONE_LINE: TableLine = [
        Tile::Empty,
        Tile::Empty,
        Tile::Movable,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
    ];
    table.table.extend([ONE_LINE; 4]);
    table.block_first_line = Some(table.table.len() - 4);
}

fn spawn_horizontak_block(table: &mut Table) {
    const ONE_LINE: TableLine = [
        Tile::Empty,
        Tile::Empty,
        Tile::Movable,
        Tile::Movable,
        Tile::Movable,
        Tile::Movable,
        Tile::Empty,
    ];
    table.table.push_back(ONE_LINE);
    table.block_first_line = Some(table.table.len() - 1);
}

fn spawn_block2x2(table: &mut Table) {
    const ONE_LINE: TableLine = [
        Tile::Empty,
        Tile::Empty,
        Tile::Movable,
        Tile::Movable,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
    ];
    table.table.extend([ONE_LINE; 2]);
    table.block_first_line = Some(table.table.len() - 2);
}

fn spawn_cross(table: &mut Table) {
    const FIRST_LINE: TableLine = [
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Movable,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
    ];
    const SECOND_LINE: TableLine = [
        Tile::Empty,
        Tile::Empty,
        Tile::Movable,
        Tile::Movable,
        Tile::Movable,
        Tile::Empty,
        Tile::Empty,
    ];

    table.table.extend([FIRST_LINE, SECOND_LINE, FIRST_LINE]);
    table.block_first_line = Some(table.table.len() - 3);
}

fn spawn_lblock(table: &mut Table) {
    const BOTTOM_LINE: TableLine = [
        Tile::Empty,
        Tile::Empty,
        Tile::Movable,
        Tile::Movable,
        Tile::Movable,
        Tile::Empty,
        Tile::Empty,
    ];
    const ONE_LINE: TableLine = [
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Movable,
        Tile::Empty,
        Tile::Empty,
    ];
    table.table.extend([BOTTOM_LINE, ONE_LINE, ONE_LINE]);
    table.block_first_line = Some(table.table.len() - 3);
}

impl Block {
    fn spawn_lines(self, table: &mut Table) {
        match self {
            Block::Block2x2 => spawn_block2x2(table),
            Block::Cross => spawn_cross(table),
            Block::Horizontal => spawn_horizontak_block(table),
            Block::LBlock => spawn_lblock(table),
            Block::Vertical => spawn_verical_block(table),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Left,
    Right,
}

impl Action {
    fn check(self, line: &TableLine) -> bool {
        match self {
            Action::Left => can_move_left(line),
            Action::Right => can_move_right(line),
        }
    }

    fn do_action(self, line: &mut TableLine) -> bool {
        match self {
            Action::Left => move_left(line),
            Action::Right => move_right(line),
        }
    }
}

fn freeze_line(line: &mut TableLine) {
    line.iter_mut()
        .filter(|&&mut t| t == Tile::Movable)
        .for_each(|t| *t = Tile::Fill);
}

fn is_full(line: &TableLine) -> bool {
    line.iter().all(|&t| t == Tile::Fill)
}

fn is_empty(line: &TableLine) -> bool {
    line.iter().all(|&t| t == Tile::Empty)
}

fn can_move_from_to(from: &TableLine, to: &TableLine) -> bool {
    std::iter::zip(from, to)
        .filter(|(&from, _)| from == Tile::Movable)
        .all(|(_, &to)| to != Tile::Fill)
}

fn move_from_to(from: &mut TableLine, to: &mut TableLine) {
    std::iter::zip(from, to)
        .filter(|(&mut from, _)| from == Tile::Movable)
        .for_each(|(from, to)| *to = std::mem::replace(from, Tile::Empty));
}

impl Table {
    fn spawn_new(&mut self, block: Block) -> &mut Self {
        self.table
            .extend(std::iter::repeat([Tile::Empty; LINE_SIZE]).take(3));
        block.spawn_lines(self);
        self
    }

    fn has_block(&self) -> bool {
        self.block_first_line.is_some()
    }

    fn do_action(&mut self, action: Action) -> &mut Self {
        if let Some(block_line) = self.block_first_line {
            let can_do_action = self
                .table
                .iter()
                .skip(block_line)
                .take_while(|line| contains_movable(line))
                .all(|line| action.check(line));

            if can_do_action {
                self.table
                    .iter_mut()
                    .skip(block_line)
                    .take_while(|line| contains_movable(line))
                    .for_each(|line| {
                        action.do_action(line);
                    });
            }
        }
        self
    }

    fn tick(&mut self) -> bool {

        let Some(block_first_line) = self.block_first_line.as_mut() else {
            return false;
        };

        let can_move = *block_first_line > 0 &&  {
            let mut can_down = true;
            for idx in *block_first_line..self.table.len() {
                let line = &self.table[idx];
                if !contains_movable(line) {
                    break;
                }
    
                let next = &self.table[idx - 1];
                can_down = can_down && can_move_from_to(line, next);
            }
            can_down
        };

        if !can_move {
            self.table.iter_mut().skip(*block_first_line).take_while(|line| contains_movable(line)).for_each(freeze_line);
            self.block_first_line = None;
            // self.trim_bottom();
            // self.test_period();
            return false;
        }

        
        for idx in *block_first_line..self.table.len() {
            let mut line = self.table[idx];
            if !contains_movable(&line) {
                break;
            }

            let next = &mut self.table[idx - 1];
            move_from_to(&mut line, next);
            self.table[idx] = line;
        }
        
        *block_first_line -= 1;
        if self.table.back().filter(|line| is_empty(line)).is_some() {
                self.table.pop_back();
        }
        true
    }

    fn height(&self) -> usize {
        self.floor_level + self.table.len()
    }

    fn trim_bottom(&mut self) {

        let Some(trim_idx) = self.table.iter().enumerate().rev().find_map(|(idx, line)| is_full(line).then_some(idx)) else {
            return;
        };

        self.floor_level += trim_idx + 1;
        for _ in 0..=trim_idx {
            self.table.pop_front();
        }

        if let Some(ref mut line) = self.block_first_line {
            *line -= trim_idx - 1;
        }
    }

    fn show(&self) {
        for line in self.table.iter().rev() {
            line.iter().map(|t| match t {
                Tile::Empty => '.',
                Tile::Fill => '#',
                Tile::Movable => '@'
            }).chain(['\n']).for_each(|c| print!("{c}"))
        }
    }

    fn test_period(&self) {

        if self.table.back().filter(|line| is_full(line)).is_some() {
            println!("FULL! {}", self.table.len());
        }

        let n = self.table.len();
        if n % 2 != 0 {
            return;
        }

        let first = self.table.iter().take(n/2);
        let last = self.table.iter().skip(n/2);

        if first.eq(last) {
            println!("period = {}", n/2)
        }
    }
}

struct Simutalion<Blocks: Iterator<Item = Block>> {
    table: Table,
    blocks: Blocks,
}

fn init_simitation() -> Simutalion<impl Iterator<Item = Block>> {
    Simutalion {
        table: Default::default(),
        blocks: [
            Block::Horizontal,
            Block::Cross,
            Block::LBlock,
            Block::Vertical,
            Block::Block2x2,
        ]
        .into_iter()
        .cycle(),
    }
}

impl<B: Iterator<Item = Block>> Simutalion<B> {
    // return true if block is stopped after tick
    fn tick(&mut self, action: Action) -> bool {
        if !self.table.has_block() {
            self.table
                .spawn_new(self.blocks.next().expect("expected block"));
        }

        self.table.do_action(action);
        self.table.tick();

        !self.table.has_block()
    }
}

fn map_action(c: char) -> Action {
    match c {
        '<' => Action::Left,
        '>' => Action::Right,
        _ => unreachable!(),
    }
}

fn find_period(vals: &[i64]) -> Option<usize> {
    for n in (1..=(vals.len() / 2)).rev() {
        let first = vals.iter().rev().take(n);
        let last = vals.iter().rev().skip(n).take(n);
        if first.eq(last) {
            return Some(n)
        }
    }
    None
}

fn main() {
    let pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let mut pattern = String::new();

    let _ = aoc2022::get_input_file().read_to_string(&mut pattern);

    let actions = pattern.trim().chars().cycle().map(map_action);

    let mut simulation = init_simitation();
    const BLOCKS_CNT: usize = 8000;
    let mut counter = 0;
    let mut heights = Vec::default();
    for (idx, act) in actions.enumerate() {
        if simulation.tick(act) {
            counter += 1;
            heights.push(simulation.table.height() as i64);
            if counter == BLOCKS_CNT {
                // simulation.table.show();
                println!("--------------------------");
                println!("{}", simulation.table.height());
                break;
            }
        }
        // simulation.table.show();
        // println!("---------------------------");
        // if idx == 10 {
            // break;
        // }
    }

    let diff : Vec<_> = heights.windows(2).map(|w| w[1] - w[0]).collect();

    println!("{diff:?}");

    let period = find_period(&diff).expect("expected period");
    let deltas = &diff[(diff.len() - period).. ];

    let sum_per_period : i64 = deltas.iter().sum();

    let cur_height = simulation.table.height();
    let already_fall = BLOCKS_CNT;

    let NEED = 1000000000000;
    let extra = NEED - already_fall;

    let full_periods = (extra / period) as i64;

    let rest = extra % period;

    let answer : i64 = cur_height as i64 + (sum_per_period * full_periods);
    let extra_add : i64 = deltas.iter().take(rest).sum();

    println!("{}", answer + extra_add)
}
