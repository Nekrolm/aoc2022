

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div
}

type MonkeyName = String;
type MonkeyRef<'a> = &'a str;

enum Monkey {
    Val(i64),
    Op {
        op: Operation,
        left: MonkeyName,
        right: MonkeyName,
    }
}

impl Operation {
    fn eval(self, lhs: i64, rhs : i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Div => lhs / rhs,
            Self::Mul => lhs * rhs,
            Self::Sub => lhs - rhs
        }
    }

    fn parse(token: &str) -> Self {
        match token {
            "+" => Self::Add,
            "-" => Self::Sub,
            "/" => Self::Div,
            "*" => Self::Mul,
            _ => panic!("unexpected op")
        }
    }

    fn solve_right(self, lhs: i64, res: i64) -> i64 {
        match self {
            Self::Add => res - lhs,
            Self::Div => lhs / res,
            Self::Mul => res / lhs,
            Self::Sub => lhs - res,
        }
    }

    fn solve_left(self, rhs: i64, res: i64) -> i64 {
        match self {
            Self::Add => res - rhs,
            Self::Div => rhs * res,
            Self::Mul => res / rhs,
            Self::Sub => rhs + res,
        }
    }
}

impl Monkey {
    fn parse(line: &str) -> Self {
        if line.chars().all(|c| c.is_ascii_digit()) {
            Self::Val(line.parse().expect("it is int"))
        } else {
            let mut tokens = line.split_whitespace();
            let left = tokens.next().unwrap().to_string();
            let op = tokens.next().map(Operation::parse).unwrap();
            let right = tokens.next().unwrap().to_string();
            Self::Op { op, left, right }
        }
    }
}

type Monkeys = std::collections::HashMap<MonkeyName, Monkey>;
type EvaluatedMonkeys = std::collections::HashMap<MonkeyName, i64>;

fn evaluate(monkeys: &Monkeys, root: MonkeyRef<'_>, precomputed: &mut EvaluatedMonkeys) -> Option<i64> {
    if let Some(&val) = precomputed.get(root) {
        return Some(val);
    }

    let monkey = monkeys.get(root)?;

    let value = match monkey {
        &Monkey::Val(v) => v,
        Monkey::Op { op, left, right } => {
            let left = evaluate(monkeys, left, precomputed);
            let right = evaluate(monkeys, right, precomputed);
            op.eval(left?, right?)
        }
    };

    precomputed.insert(root.to_string(), value);

    Some(value)
}



fn evaluate2(monkeys: &Monkeys, root: MonkeyRef<'_>, hum: MonkeyRef<'_>, precomputed: &mut EvaluatedMonkeys) -> i64 {
    let Some(Monkey::Op { op, left, right }) = monkeys.get(root) else {
        panic!("root should be Op")
    };

    let left_v = evaluate(monkeys, &left, precomputed);
    let right_v = evaluate(monkeys, &right, precomputed);

    println!("{left_v:?} = {right_v:?}");

    match (left_v, right_v) {
        (Some(lhs), None) => {
            solve(monkeys, lhs, right, hum, precomputed)
        },
        (None, Some(rhs)) => {
            solve(monkeys, rhs, left, hum, precomputed)
        },
        _ => None
    }.expect("expected value")
}

fn solve(monkeys: &Monkeys, value: i64, cur: MonkeyRef<'_>, hum: MonkeyRef<'_>, precomputed: &mut EvaluatedMonkeys) -> Option<i64> {
    if cur == hum {
        return Some(value);
    }

    let Monkey::Op { op, left, right } = monkeys.get(cur)? 
    else {
        return None;
    };

    let left_v = evaluate(monkeys, left, precomputed);
    let right_v = evaluate(monkeys, right, precomputed);

    match (left_v, right_v) {
        (Some(lhs), None) => {
            let rhs = op.solve_right(lhs, value);
            solve(monkeys, rhs, right, hum, precomputed)
        },
        (None, Some(rhs)) => {
            let lhs = op.solve_left(rhs, value);
            solve(monkeys, lhs, left, hum, precomputed)
        },
        _ => {
            println!("something wrong");
            dbg!(left, right, left_v, right_v);
            None
        }
    }
}


fn parse_monkey(line: &str) -> (MonkeyName, Monkey) {
    let (name, desc) = line.split_once(':').expect("expected monkey_name: desc");
    let desc = desc.trim();
    
    let name = name.to_string();
    (name, Monkey::parse(desc))
}

fn main() {
    let mut monkeys : Monkeys = aoc2022::parse_line_by_line(aoc2022::get_input_file(), parse_monkey).collect();
    let mut cache = EvaluatedMonkeys::default();
    const ROOT : MonkeyRef = "root";
    const HUMN : MonkeyRef = "humn";

    // let val = evaluate(&monkeys, ROOT, &mut cache);
    // println!("{val:?}")


    monkeys.remove(HUMN);
    let val = evaluate2(&monkeys, ROOT, HUMN, &mut cache);
    println!("{val}");
}