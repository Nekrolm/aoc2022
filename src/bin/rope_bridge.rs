use std::{collections::HashSet};



#[derive(Debug, Clone, Copy)]
enum Command {
    Up, Right, Down, Left
}

fn parse_command(s: &str) -> (Command, usize) {
    let mut tokens = s.split_whitespace();
    let cmd = tokens.next().expect("expected command");
    let cnt = tokens.next().expect("expected integer");

    let cmd = 
    match cmd {
        "R" => Command::Right,
        "D" => Command::Down,
        "L" => Command::Left,
        "U" => Command::Up,
        x => panic!("unexpected command: {x}")
    };
    (cmd, cnt.parse().expect("sould be int"))
} 


#[derive(Debug)]
struct Rope {
    rope: Vec<(i64, i64)>
}

impl Command {
    pub fn delta(self) -> (i64, i64) {
        match self {
            Command::Down => (0, -1),
            Command::Left => (-1, 0),
            Command::Right => (1, 0),
            Command::Up => (0, 1)
        }
    }
}

fn add_vec((xa, ya): (i64, i64), (xb, yb) : (i64, i64)) -> (i64, i64) {
    (xa + xb, ya + yb)
}

fn sub_vec((xa, ya): (i64, i64), (xb, yb) : (i64, i64)) -> (i64, i64) {
    (xa - xb, ya - yb)
}

impl Rope {
    pub fn new(len: usize) -> Self {
        assert!(len >= 2);
        Self {
            rope: vec![Default::default(); len]
        }
    }

    pub fn move_rope(&mut self, cmd: Command) {
        let ofs = cmd.delta();
        self.rope[0] = add_vec(self.rope[0], ofs);
        self.fix_tail();
    }

    fn fix_tail(&mut self) {
        for idx in 1..self.rope.len() {
            let head = self.rope[idx-1];
            let tail = self.rope[idx];
            let (mut dx, mut dy) = sub_vec(tail, head);
            if dx.abs() <= 1 && dy.abs() <= 1 {
                return;
            }
            if dx.abs() == 2 && dy.abs() == 2 {
                dx /= 2; dy /= 2;
            }
            if dx.abs() == 2 {
                dy = 0;
                dx /= 2;
            }
            if dy.abs() == 2 {
                dx = 0;
                dy /= 2;
            }
            self.rope[idx] = add_vec(head, (dx, dy))
        }
    }

    fn tail(&self) -> (i64, i64) {
        *self.rope.last().expect("rope has at least 2 knots")
    }
}

struct RopeOnField {
    tail_visited: HashSet<(i64, i64)>,
    rope: Rope,
}



impl RopeOnField {
    fn new(len: usize) -> Self {
        let mut tail_visited = HashSet::default();
        let rope = Rope::new(len);
        let _ = tail_visited.insert(rope.tail());
        RopeOnField { tail_visited, rope }
    }

    fn move_rope(&mut self, (command, cnt): (Command, usize)) -> &mut Self {
        for _ in 0..cnt {
            self.rope.move_rope(command);
            // println!("{command:?} {:?}", self.rope.rope);
            let _ = self.tail_visited.insert(self.rope.tail());
        };
        self
    }
}


fn main() {
    let infile = aoc2022::get_input_file();
    let mut rope = RopeOnField::new(10);
    let rope = aoc2022::parse_line_by_line(infile, parse_command).fold(&mut rope, RopeOnField::move_rope);
    
    
    println!("{}", rope.tail_visited.len())
}