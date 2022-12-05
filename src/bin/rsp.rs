#![allow(dead_code)]

use std::io::BufRead;


#[derive(Clone, Copy)]
enum Action {
    Rock,
    Scissors,
    Paper,
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(self) -> i64 {
        match self {
            Outcome::Draw => 3,
            Outcome::Win => 6,
            Outcome::Lose => 0
        }
    }
}

impl Action {
    fn score(self) -> i64 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }

    fn collide(self, other: Action) -> Outcome {
        use Action::*;
        match (self, other) {
          (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Outcome::Lose,
          (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Outcome::Win,
          _ => Outcome::Draw
        }
    }

    fn winner(self) -> Action {
        match self {
            Self::Paper => Self::Scissors,
            Self::Rock => Self::Paper,
            Self::Scissors => Self::Rock
        }
    }

    fn looser(self) -> Action {
        match self {
            Self::Paper => Self::Rock,
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper
        }
    }

    fn guide(self, outcome: Outcome) -> Action {
        use Outcome::*;
        match outcome {
            Draw => self,
            Win => self.winner(),
            Lose => self.looser()
        }
    }
}

fn parse_action(c: char) -> Action {
    use Action::*;
    match c {
      'A' | 'X' => Rock,
      'B' | 'Y' => Paper,
      'C' | 'Z' => Scissors,
       _  => panic!("Unknow action symbol {c}")
    }
}

fn parse_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Unknown outcome symbol {c}")
    }
}

fn main() {
    let infile = aoc2022::get_input_file();
    let reader = std::io::BufReader::new(infile);


    // part 1
    // let score : i64 = reader.lines().map(Result::unwrap).filter(|s| !s.is_empty()).map(
    //     |guide| {
    //         let mut guide = guide.chars().filter(|c| !c.is_whitespace()).map(parse_action);
    //         let other = guide.next().expect("should be at least 2 elements");
    //         let me =  guide.next().expect("should be at least 2 elements");
    //         me.collide(other).score() + me.score()
    //     }).sum();
    


    let score : i64 = reader.lines().map(Result::unwrap).filter(|s| !s.is_empty()).map(
        |guide| {
            let mut guide = guide.chars().filter(|c| !c.is_whitespace());
            let other = guide.next().map(parse_action).expect("should be at least 2 elements");
            let me =  guide.next().map(parse_outcome).expect("should be at least 2 elements");
            other.guide(me).score() + me.score()
        }).sum();

    println!("{score}")
}